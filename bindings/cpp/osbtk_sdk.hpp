#pragma once
// Typed, dependency-free C++ SDK over the osbTK native engine. It wraps the header-only `osbtk.hpp` binding: the transport talks to the
// native JSON bridge, and the typed models mirror the OpenAPI contract in
// `crates/osb-api`. There is no HTTP and no external JSON library.

#include <cctype>
#include <cstddef>
#include <cstdlib>
#include <map>
#include <optional>
#include <stdexcept>
#include <string>
#include <utility>
#include <variant>
#include <vector>

#include "osbtk.hpp"

namespace osbtk::sdk {

// --- Minimal JSON (objects are vector-of-pairs so the type may be incomplete) --

struct Json {
    using Array = std::vector<Json>;
    using Object = std::vector<std::pair<std::string, Json>>;
    std::variant<std::nullptr_t, bool, double, std::string, Array, Object> value;

    bool is_object() const { return std::holds_alternative<Object>(value); }
    bool is_array() const { return std::holds_alternative<Array>(value); }
    bool is_number() const { return std::holds_alternative<double>(value); }
    bool is_string() const { return std::holds_alternative<std::string>(value); }
    bool is_bool() const { return std::holds_alternative<bool>(value); }

    const Object& object() const { return std::get<Object>(value); }
    const Array& array() const { return std::get<Array>(value); }
    double number() const { return is_number() ? std::get<double>(value) : 0.0; }
    bool boolean() const { return is_bool() ? std::get<bool>(value) : false; }
    std::string string() const { return is_string() ? std::get<std::string>(value) : std::string(); }

    const Json* find(const std::string& key) const {
        if (!is_object()) {
            return nullptr;
        }
        for (const auto& entry : object()) {
            if (entry.first == key) {
                return &entry.second;
            }
        }
        return nullptr;
    }

    const Json& at(const std::string& key) const {
        const Json* found = find(key);
        if (found == nullptr) {
            throw std::runtime_error("osbtk: missing JSON key " + key);
        }
        return *found;
    }
};

namespace detail {

struct Parser {
    const std::string& text;
    std::size_t index = 0;

    explicit Parser(const std::string& input) : text(input) {}

    void skip() {
        while (index < text.size() && std::isspace(static_cast<unsigned char>(text[index]))) {
            ++index;
        }
    }

    [[noreturn]] void fail(const std::string& message) const {
        throw std::runtime_error("osbtk: invalid JSON (" + message + ") at " + std::to_string(index));
    }

    Json parse() {
        skip();
        Json value = parse_value();
        skip();
        return value;
    }

    Json parse_value() {
        skip();
        if (index >= text.size()) {
            fail("unexpected end");
        }
        const char c = text[index];
        if (c == '{') return parse_object();
        if (c == '[') return parse_array();
        if (c == '"') return Json{parse_string()};
        if (c == 't' || c == 'f') return parse_bool();
        if (c == 'n') return parse_null();
        return Json{parse_number()};
    }

    Json parse_object() {
        ++index;  // {
        Json::Object object;
        skip();
        if (index < text.size() && text[index] == '}') {
            ++index;
            return Json{object};
        }
        while (true) {
            skip();
            std::string key = parse_string();
            skip();
            if (index >= text.size() || text[index] != ':') fail("expected :");
            ++index;
            object.emplace_back(std::move(key), parse_value());
            skip();
            if (index >= text.size()) fail("unterminated object");
            if (text[index] == ',') {
                ++index;
                continue;
            }
            if (text[index] == '}') {
                ++index;
                break;
            }
            fail("expected , or }");
        }
        return Json{object};
    }

    Json parse_array() {
        ++index;  // [
        Json::Array array;
        skip();
        if (index < text.size() && text[index] == ']') {
            ++index;
            return Json{array};
        }
        while (true) {
            array.push_back(parse_value());
            skip();
            if (index >= text.size()) fail("unterminated array");
            if (text[index] == ',') {
                ++index;
                continue;
            }
            if (text[index] == ']') {
                ++index;
                break;
            }
            fail("expected , or ]");
        }
        return Json{array};
    }

    std::string parse_string() {
        if (index >= text.size() || text[index] != '"') fail("expected string");
        ++index;
        std::string result;
        while (index < text.size()) {
            const char c = text[index++];
            if (c == '"') {
                return result;
            }
            if (c != '\\') {
                result.push_back(c);
                continue;
            }
            if (index >= text.size()) fail("bad escape");
            const char escape = text[index++];
            switch (escape) {
                case '"': result.push_back('"'); break;
                case '\\': result.push_back('\\'); break;
                case '/': result.push_back('/'); break;
                case 'b': result.push_back('\b'); break;
                case 'f': result.push_back('\f'); break;
                case 'n': result.push_back('\n'); break;
                case 'r': result.push_back('\r'); break;
                case 't': result.push_back('\t'); break;
                case 'u': {
                    if (index + 4 > text.size()) fail("bad \\u escape");
                    const int code = std::strtol(text.substr(index, 4).c_str(), nullptr, 16);
                    index += 4;
                    if (code < 0x80) {
                        result.push_back(static_cast<char>(code));
                    } else if (code < 0x800) {
                        result.push_back(static_cast<char>(0xC0 | (code >> 6)));
                        result.push_back(static_cast<char>(0x80 | (code & 0x3F)));
                    } else {
                        result.push_back(static_cast<char>(0xE0 | (code >> 12)));
                        result.push_back(static_cast<char>(0x80 | ((code >> 6) & 0x3F)));
                        result.push_back(static_cast<char>(0x80 | (code & 0x3F)));
                    }
                    break;
                }
                default: fail("unknown escape");
            }
        }
        fail("unterminated string");
    }

    Json parse_bool() {
        if (text.compare(index, 4, "true") == 0) {
            index += 4;
            return Json{true};
        }
        if (text.compare(index, 5, "false") == 0) {
            index += 5;
            return Json{false};
        }
        fail("expected bool");
    }

    Json parse_null() {
        if (text.compare(index, 4, "null") == 0) {
            index += 4;
            return Json{nullptr};
        }
        fail("expected null");
    }

    double parse_number() {
        const char* start = text.c_str() + index;
        char* end = nullptr;
        const double value = std::strtod(start, &end);
        if (end == start) fail("expected number");
        index += static_cast<std::size_t>(end - start);
        return value;
    }
};

inline Json parse_json(const std::string& text) { return Parser(text).parse(); }

inline std::string number(double value) { return std::to_string(value); }

inline std::string quote(const std::string& value) {
    std::string result = "\"";
    for (const char c : value) {
        switch (c) {
            case '"': result += "\\\""; break;
            case '\\': result += "\\\\"; break;
            case '\n': result += "\\n"; break;
            case '\r': result += "\\r"; break;
            case '\t': result += "\\t"; break;
            default: result.push_back(c); break;
        }
    }
    result += "\"";
    return result;
}

}  // namespace detail

// --- Typed models -------------------------------------------------------------

struct Vector2 {
    double x = 0.0;
    double y = 0.0;
};

struct Vector3 {
    double x = 0.0;
    double y = 0.0;
    double z = 0.0;
};

struct Vector4 {
    double x = 0.0;
    double y = 0.0;
    double z = 0.0;
    double w = 0.0;
};

struct Color4 {
    double r = 1.0;
    double g = 1.0;
    double b = 1.0;
    double a = 1.0;
};

/// Sprite anchor origin (the wire value is the canonical name, e.g. "Centre").
enum class OsbOrigin { TopLeft, TopCentre, TopRight, CentreLeft, Centre, CentreRight, BottomLeft, BottomCentre, BottomRight };
enum class OsbLoopType { LoopOnce, LoopForever };
enum class OsbLayer { Background, Fail, Pass, Foreground, Overlay };
enum class ParameterType { FlipHorizontal, FlipVertical, Additive };
enum class HitObjectKind { Circle, Slider, Spinner, Hold };

inline const char* to_string(OsbOrigin value) {
    switch (value) {
        case OsbOrigin::TopLeft: return "TopLeft";
        case OsbOrigin::TopCentre: return "TopCentre";
        case OsbOrigin::TopRight: return "TopRight";
        case OsbOrigin::CentreLeft: return "CentreLeft";
        case OsbOrigin::Centre: return "Centre";
        case OsbOrigin::CentreRight: return "CentreRight";
        case OsbOrigin::BottomLeft: return "BottomLeft";
        case OsbOrigin::BottomCentre: return "BottomCentre";
        case OsbOrigin::BottomRight: return "BottomRight";
    }
    return "Centre";
}

inline const char* to_string(OsbLoopType value) {
    return value == OsbLoopType::LoopForever ? "LoopForever" : "LoopOnce";
}

inline const char* to_string(OsbLayer value) {
    switch (value) {
        case OsbLayer::Background: return "Background";
        case OsbLayer::Fail: return "Fail";
        case OsbLayer::Pass: return "Pass";
        case OsbLayer::Foreground: return "Foreground";
        case OsbLayer::Overlay: return "Overlay";
    }
    return "Background";
}

inline HitObjectKind hit_object_kind_from(const std::string& value) {
    if (value == "Slider") return HitObjectKind::Slider;
    if (value == "Spinner") return HitObjectKind::Spinner;
    if (value == "Hold") return HitObjectKind::Hold;
    return HitObjectKind::Circle;
}

struct Handle {
    int layer = 0;
    int sprite = 0;
};

struct SpriteState {
    Vector2 position;
    Vector2 scale;
    double rotation = 0.0;
    double opacity = 1.0;
    bool additive = false;
    Color4 color;
    /// Sampled origin offset in texture pixels (`Origins3d`).
    Vector2 anchor;
};

struct ControlPoint {
    double time = 0.0;
    double beat_duration = 0.0;
    double bpm = 0.0;
    bool timing_change = false;
};

struct HitObject {
    double start_time = 0.0;
    double end_time = 0.0;
    double x = 0.0;
    double y = 0.0;
    HitObjectKind kind = HitObjectKind::Circle;
};

struct Beatmap {
    std::string audio_filename;
    std::string title;
    std::string artist;
    std::string version;
    double approach_rate = 0.0;
    double circle_size = 0.0;
    std::vector<ControlPoint> timing_points;
    std::vector<HitObject> hit_objects;
};

inline Vector2 parse_vec2(const Json& json) {
    return Vector2{json.find("x") != nullptr ? json.at("x").number() : 0.0,
                json.find("y") != nullptr ? json.at("y").number() : 0.0};
}

inline Color4 parse_color(const Json& json) {
    return Color4{json.find("r") != nullptr ? json.at("r").number() : 1.0,
                  json.find("g") != nullptr ? json.at("g").number() : 1.0,
                  json.find("b") != nullptr ? json.at("b").number() : 1.0,
                  json.find("a") != nullptr ? json.at("a").number() : 1.0};
}

inline SpriteState parse_state(const Json& json) {
    SpriteState state;
    state.position = parse_vec2(json.at("position"));
    state.scale = parse_vec2(json.at("scale"));
    state.rotation = json.at("rotation").number();
    state.opacity = json.at("opacity").number();
    state.additive = json.at("additive").boolean();
    state.color = parse_color(json.at("color"));
    if (json.find("anchor") != nullptr) {
        state.anchor = parse_vec2(json.at("anchor"));
    }
    return state;
}

inline Beatmap parse_beatmap_json(const Json& json) {
    Beatmap beatmap;
    beatmap.audio_filename = json.find("audioFilename") != nullptr ? json.at("audioFilename").string() : "";
    beatmap.title = json.find("title") != nullptr ? json.at("title").string() : "";
    beatmap.artist = json.find("artist") != nullptr ? json.at("artist").string() : "";
    beatmap.version = json.find("version") != nullptr ? json.at("version").string() : "";
    beatmap.approach_rate = json.find("approachRate") != nullptr ? json.at("approachRate").number() : 0.0;
    beatmap.circle_size = json.find("circleSize") != nullptr ? json.at("circleSize").number() : 0.0;
    if (const Json* points = json.find("timingPoints"); points != nullptr && points->is_array()) {
        for (const Json& point : points->array()) {
            beatmap.timing_points.push_back(ControlPoint{
                point.at("time").number(),
                point.at("beatDuration").number(),
                point.at("bpm").number(),
                point.find("timingChange") != nullptr ? point.at("timingChange").boolean() : false,
            });
        }
    }
    if (const Json* objects = json.find("hitObjects"); objects != nullptr && objects->is_array()) {
        for (const Json& object : objects->array()) {
            beatmap.hit_objects.push_back(HitObject{
                object.at("startTime").number(),
                object.at("endTime").number(),
                object.at("x").number(),
                object.at("y").number(),
                hit_object_kind_from(object.find("kind") != nullptr ? object.at("kind").string() : ""),
            });
        }
    }
    return beatmap;
}

// --- Libraries ----------------------------------------------------------------

/// The storyboarding library: `.osu`/`.osb` parsing.
class Storyboarding {
public:
    explicit Storyboarding(const Library& library) : library_(&library) {}

    /// Parses a `.osu` beatmap into the typed model.
    Beatmap parse_beatmap(const std::string& osu_text) const {
        return parse_beatmap_json(call("Core.parseOsu", "{\"text\":" + detail::quote(osu_text) + "}"));
    }

    /// Parses a `.osb` storyboard into a summary.
    Json parse_osb(const std::string& osb_text) const {
        return call("Core.parseOsb", "{\"text\":" + detail::quote(osb_text) + "}");
    }

    /// `OsbSprite.GetOriginVector`.
    Vector2 get_origin_vector(OsbOrigin origin, double width, double height) const {
        Json json = call("Core.getOriginVector", "{\"origin\":" + detail::quote(to_string(origin))
                                                + ",\"width\":" + detail::number(width)
                                                + ",\"height\":" + detail::number(height) + "}");
        return parse_vec2(json);
    }

    /// `OsbSprite.InScreenBounds`.
    bool in_screen_bounds(Vector2 position, Vector2 size, double rotation, Vector2 origin) const {
        Json json = call("Core.inScreenBounds", "{\"position\":" + vec(position) + ",\"size\":" + vec(size)
                                               + ",\"rotation\":" + detail::number(rotation)
                                               + ",\"origin\":" + vec(origin) + "}");
        return json.at("value").boolean();
    }

    /// The 2D storyboard command utilities: pure transforms over a
    /// `StoryboardCommand[]`, canonical in `osb_runtime::storyboarding::helpers`.
    class Commands {
    public:
        explicit Commands(const Library& library) : library_(&library) {}

        /// Calls any helper by name with a raw input object.
        Json call(const std::string& name, const std::string& input_json) const {
            Json json = detail::parse_json(library_->exec("{\"op\":\"Helpers.commands\",\"name\":"
                                                          + detail::quote(name) + ",\"input\":" + input_json + "}"));
            if (const Json* error = json.find("error"); error != nullptr) {
                throw std::runtime_error("osbtk: " + error->string());
            }
            return json.at("value");
        }

        Json select(const std::string& commands, const std::string& extra = "") const {
            return call("selectCommands", "{\"commands\":" + commands + extra + "}");
        }

        Json shift(const std::string& commands, double delta) const {
            return call("shiftCommands", "{\"commands\":" + commands + ",\"delta\":" + detail::number(delta) + "}");
        }

        Json scale(const std::string& commands, double factor, double anchor = 0.0) const {
            return call("scaleCommands", "{\"commands\":" + commands + ",\"factor\":" + detail::number(factor)
                                             + ",\"anchor\":" + detail::number(anchor) + "}");
        }

        Json reverse(const std::string& commands, double start, double end) const {
            return call("reverseCommands", "{\"commands\":" + commands + ",\"start\":" + detail::number(start)
                                               + ",\"end\":" + detail::number(end) + "}");
        }

        Json merge(const std::string& commands, double tolerance = 0.0) const {
            return call("mergeCommands", "{\"commands\":" + commands + ",\"tolerance\":" + detail::number(tolerance) + "}");
        }

        Json split(const std::string& commands, double at) const {
            return call("splitCommands", "{\"commands\":" + commands + ",\"at\":" + detail::number(at) + "}");
        }

        Json flatten(const std::string& commands, const std::string& mode = "mean") const {
            return call("flattenCommands", "{\"commands\":" + commands + ",\"mode\":" + detail::quote(mode) + "}");
        }

        Json randomize(const std::string& commands, double min, double max, long long seed = 0) const {
            return call("randomizeCommands", "{\"commands\":" + commands + ",\"min\":" + detail::number(min)
                                                 + ",\"max\":" + detail::number(max) + ",\"seed\":"
                                                 + std::to_string(seed) + "}");
        }

        Json filter(const std::string& commands, const std::string& extra = "") const {
            return call("filterCommands", "{\"commands\":" + commands + extra + "}");
        }

        Json remove(const std::string& commands, const std::string& extra = "") const {
            return call("removeCommands", "{\"commands\":" + commands + extra + "}");
        }

        Json map(const std::string& commands, double value_scale = 1.0, double value_offset = 0.0) const {
            return call("mapCommands", "{\"commands\":" + commands + ",\"valueScale\":" + detail::number(value_scale)
                                           + ",\"valueOffset\":" + detail::number(value_offset) + "}");
        }

        Json duplicate(const std::string& commands, double offset = 0.0, int count = 1) const {
            return call("duplicateCommands", "{\"commands\":" + commands + ",\"offset\":" + detail::number(offset)
                                                 + ",\"count\":" + std::to_string(count) + "}");
        }

        Json align(const std::string& commands, double beat = 500.0, double offset = 0.0,
                   int subdivision = 1, double value_step = 0.0) const {
            return call("alignCommands", "{\"commands\":" + commands + ",\"beat\":" + detail::number(beat)
                                             + ",\"offset\":" + detail::number(offset) + ",\"subdivision\":"
                                             + std::to_string(subdivision) + ",\"valueStep\":"
                                             + detail::number(value_step) + "}");
        }

        Json quantize(const std::string& commands, double time_step = 0.0, double value_step = 0.0) const {
            return call("quantizeCommands", "{\"commands\":" + commands + ",\"timeStep\":"
                                                + detail::number(time_step) + ",\"valueStep\":"
                                                + detail::number(value_step) + "}");
        }

    private:
        const Library* library_;
    };

    /// The 2D command utilities facade.
    Commands commands() const { return Commands(*library_); }

protected:
    const Library* library_;

    static std::string vec(Vector2 value) {
        return "{\"x\":" + detail::number(value.x) + ",\"y\":" + detail::number(value.y) + "}";
    }

    Json call(const std::string& op, const std::string& body) const {
        Json json = detail::parse_json(library_->exec("{\"op\":" + detail::quote(op) + "," + body.substr(1)));
        if (const Json* error = json.find("error"); error != nullptr) {
            throw std::runtime_error("osbtk: " + error->string());
        }
        return json;
    }
};

/// The 3D storyboard library: a sibling that reuses the `Storyboarding` surface
/// (via inheritance) and adds the 3D -> 2D bake.
class Storyboarding3d : public Storyboarding {
public:
    using Storyboarding::Storyboarding;

    /// Bakes a 3D scene into a 2D storyboard.
    Json bake3d(const std::string& input_json) const {
        return call("Storyboarding3d.bake3d", "{\"input\":" + input_json + "}");
    }

    /// The 3D motion-graphics keyframe helpers: pure keyframe-channel
    /// utilities, canonical in `osb_runtime::storyboarding3d::helpers`.
    class Helpers {
    public:
        /// A single keyframe: a time and a value of one, two or three components.
        struct ChannelKey {
            double time = 0.0;
            std::vector<double> value;
        };

        /// A keyframed channel: a list of `(time, value)` samples.
        struct Channel {
            std::vector<ChannelKey> keys;
        };

        explicit Helpers(const Library& library) : library_(&library) {}

        /// Calls any shared helper by name (`keyframe.*`, `scene3d.*`, `command.*`,
        /// `origin.*`; unprefixed names are resolved by library).
        Json call(const std::string& name, const std::string& input_json) const {
            Json json = detail::parse_json(library_->exec("{\"op\":\"Helpers.call\",\"name\":"
                                                          + detail::quote(name) + ",\"input\":" + input_json + "}"));
            if (const Json* error = json.find("error"); error != nullptr) {
                throw std::runtime_error("osbtk: " + error->string());
            }
            return json.at("value");
        }

        /// Applies any keyframe transform (e.g. `keyframe.flatten`) to a 2D command channel.
        Json transform_commands(const std::string& commands, const std::string& kind,
                                const std::string& name, const std::string& options = "{}") const {
            std::string extra = options == "{}" ? std::string()
                                                : "," + options.substr(1, options.size() - 2);
            return call(name, "{\"commands\":" + commands + ",\"kind\":" + detail::quote(kind) + extra + "}");
        }

        /// The anchor offset for an enum origin, in texture pixels.
        Vector2 origin_vector(const std::string& origin, double width, double height) const {
            return parse_vec2(call("origin.getOriginVector",
                "{\"origin\":" + detail::quote(origin) + ",\"width\":" + detail::number(width)
                + ",\"height\":" + detail::number(height) + "}"));
        }

        /// The effective pivot: the enum origin vector plus an explicit anchor offset.
        Vector2 resolve_anchor(const std::string& origin, Vector2 anchor, double width, double height) const {
            return parse_vec2(call("origin.resolveAnchor",
                "{\"origin\":" + detail::quote(origin) + ",\"anchor\":" + point(anchor)
                + ",\"width\":" + detail::number(width) + ",\"height\":" + detail::number(height) + "}"));
        }

        /// Whether a rotated sprite's bounds intersect the screen.
        bool in_screen_bounds(Vector2 position, Vector2 size, double rotation, Vector2 origin) const {
            return call("origin.inScreenBounds", "{\"position\":" + point(position) + ",\"size\":"
                                                     + point(size) + ",\"rotation\":"
                                                     + detail::number(rotation) + ",\"origin\":"
                                                     + point(origin) + "}").boolean();
        }

        Channel flatten(const Channel& channel, const std::string& axis = "", const std::string& mode = "mean") const {
            return channel_call("flatten", "{\"channel\":" + channel_json(channel) + ",\"axis\":"
                                               + (axis.empty() ? "null" : detail::quote(axis))
                                               + ",\"mode\":" + detail::quote(mode) + "}");
        }

        Channel scale_keyframes(const Channel& channel, double factor) const {
            return channel_call("scaleKeyframes", "{\"channel\":" + channel_json(channel)
                                                      + ",\"factor\":" + detail::number(factor) + "}");
        }

        Channel offset_keyframes(const Channel& channel, double delta) const {
            return channel_call("offsetKeyframes", "{\"channel\":" + channel_json(channel)
                                                       + ",\"delta\":" + detail::number(delta) + "}");
        }

        Channel mirror(const Channel& channel, const std::string& axis = "", double pivot = 0.0) const {
            return channel_call("mirror", "{\"channel\":" + channel_json(channel) + ",\"axis\":"
                                              + (axis.empty() ? "null" : detail::quote(axis))
                                              + ",\"pivot\":" + detail::number(pivot) + "}");
        }

        Channel randomize(const Channel& channel, double min = -1.0, double max = 1.0,
                          long long seed = 0, double step = 0.0) const {
            return channel_call("randomize", "{\"channel\":" + channel_json(channel) + ",\"min\":"
                                                 + detail::number(min) + ",\"max\":" + detail::number(max)
                                                 + ",\"seed\":" + std::to_string(seed) + ",\"step\":"
                                                 + detail::number(step) + "}");
        }

        Channel reverse_keyframes(const Channel& channel, double start, double end) const {
            return channel_call("reverseKeyframes", "{\"channel\":" + channel_json(channel)
                                                        + ",\"start\":" + detail::number(start)
                                                        + ",\"end\":" + detail::number(end) + "}");
        }

        Channel repeat_keyframes(const Channel& channel, int count, double start, double end,
                                 bool ping_pong = false) const {
            return channel_call("repeatKeyframes", "{\"channel\":" + channel_json(channel)
                                                       + ",\"count\":" + std::to_string(count)
                                                       + ",\"start\":" + detail::number(start)
                                                       + ",\"end\":" + detail::number(end)
                                                       + ",\"pingPong\":" + (ping_pong ? "true" : "false") + "}");
        }

        Channel resample_keyframes(const Channel& channel, double step, int count = 0) const {
            return channel_call("resampleKeyframes", "{\"channel\":" + channel_json(channel)
                                                         + ",\"step\":" + detail::number(step)
                                                         + ",\"count\":" + std::to_string(count) + "}");
        }

        Channel smooth_keyframes(const Channel& channel, int window = 3) const {
            return channel_call("smoothKeyframes", "{\"channel\":" + channel_json(channel)
                                                       + ",\"window\":" + std::to_string(window) + "}");
        }

        Channel simplify_keyframes(const Channel& channel, double tolerance = 0.1, int decimals = 3) const {
            return channel_call("simplifyKeyframes", "{\"channel\":" + channel_json(channel)
                                                         + ",\"tolerance\":" + detail::number(tolerance)
                                                         + ",\"decimals\":" + std::to_string(decimals) + "}");
        }

        Channel align_keyframes(const Channel& channel, double beat = 500.0, double offset = 0.0,
                                int subdivision = 1) const {
            return channel_call("alignKeyframes", "{\"channel\":" + channel_json(channel)
                                                      + ",\"beat\":" + detail::number(beat)
                                                      + ",\"offset\":" + detail::number(offset)
                                                      + ",\"subdivision\":" + std::to_string(subdivision) + "}");
        }

        Channel ease_keyframes(const Channel& channel, const std::string& easing = "None", int samples = 8) const {
            return channel_call("easeKeyframes", "{\"channel\":" + channel_json(channel)
                                                     + ",\"easing\":" + detail::quote(easing)
                                                     + ",\"samples\":" + std::to_string(samples) + "}");
        }

        Channel merge_keyframes(const Channel& channel, const Channel& other, double amount = 0.5) const {
            return channel_call("mergeKeyframes", "{\"channel\":" + channel_json(channel)
                                                      + ",\"other\":" + channel_json(other)
                                                      + ",\"amount\":" + detail::number(amount) + "}");
        }

        double stagger(int index, double step) const {
            Json value = call("stagger", "{\"index\":" + std::to_string(index) + ",\"step\":"
                                             + detail::number(step) + "}");
            return value.number();
        }

        Channel orbit(double radius_x = 100.0, double radius_y = 100.0, double start_angle = 0.0,
                      double sweep = 6.283185307179586, int count = 32) const {
            return channel_call("orbit", "{\"radiusX\":" + detail::number(radius_x) + ",\"radiusY\":"
                                             + detail::number(radius_y) + ",\"startAngle\":"
                                             + detail::number(start_angle) + ",\"sweep\":"
                                             + detail::number(sweep) + ",\"count\":" + std::to_string(count) + "}");
        }

        Channel pulse(double base = 1.0, double amplitude = 0.5, double frequency = 1.0,
                      double phase = 0.0, double step = 0.0, double duration = 1000.0) const {
            return channel_call("pulse", "{\"base\":" + detail::number(base) + ",\"amplitude\":"
                                             + detail::number(amplitude) + ",\"frequency\":"
                                             + detail::number(frequency) + ",\"phase\":" + detail::number(phase)
                                             + ",\"step\":" + detail::number(step) + ",\"duration\":"
                                             + detail::number(duration) + "}");
        }

        Json scatter(long long seed = 0, int count = 8, double spread = 0.0) const {
            return call("scatter", "{\"seed\":" + std::to_string(seed) + ",\"count\":" + std::to_string(count)
                                       + ",\"spread\":" + detail::number(spread) + "}");
        }

        Channel spin(const std::string& axis = "z", double turns = 1.0, double duration = 1000.0,
                     int count = 16) const {
            return channel_call("spin", "{\"axis\":" + detail::quote(axis) + ",\"turns\":"
                                            + detail::number(turns) + ",\"duration\":"
                                            + detail::number(duration) + ",\"count\":" + std::to_string(count) + "}");
        }

        Channel path_follow(const std::string& points_json, int count = 16, bool by_arc_length = false) const {
            return channel_call("pathFollow", "{\"points\":" + points_json + ",\"count\":"
                                                  + std::to_string(count) + ",\"byArcLength\":"
                                                  + (by_arc_length ? "true" : "false") + "}");
        }

        /// The 3D effective pivot: a Vec3 anchor whose z anchors depth (raw Json).
        Json resolve_anchor_3d(const std::string& origin, const std::string& anchor_json,
                               double width, double height) const {
            return call("origin.resolveAnchor3d",
                "{\"origin\":" + detail::quote(origin) + ",\"anchor\":" + anchor_json
                + ",\"width\":" + detail::number(width) + ",\"height\":" + detail::number(height) + "}");
        }

        // -- timelines --

        /// The timeline's total duration.
        double timeline_duration(const std::string& timeline_json) const {
            return call("timeline.durationOf", "{\"timeline\":" + timeline_json + "}").number();
        }

        /// Resolves a timeline tree to flat absolute-time clips.
        Json flush_timeline(const std::string& timeline_json) const {
            return call("timeline.flushTimeline", "{\"timeline\":" + timeline_json + "}");
        }

        /// Overlays two timelines' clips and children.
        Json merge_timelines(const std::string& a, const std::string& b) const {
            return call("timeline.mergeTimelines", "{\"timeline\":" + a + ",\"other\":" + b + "}");
        }

        /// Adds a nested child timeline (a group).
        Json group_timeline(const std::string& timeline_json,
                            const std::string& child_json = "{\"clips\":[]}") const {
            return call("timeline.addGroup",
                "{\"timeline\":" + timeline_json + ",\"child\":" + child_json + "}");
        }

        /// The total clip count across the tree.
        int timeline_clip_count(const std::string& timeline_json) const {
            return static_cast<int>(
                call("timeline.clipCount", "{\"timeline\":" + timeline_json + "}").number());
        }

        // -- command easing --

        /// Sets an easing on every command (apply an easing to a whole group/scene).
        Json set_easing(const std::string& commands, const std::string& easing) const {
            return call("command.setEasing",
                "{\"commands\":" + commands + ",\"easing\":" + detail::quote(easing) + "}");
        }

        /// Re-eases a command channel with a named easing, resolving it into commands.
        Json apply_easing(const std::string& commands, const std::string& kind,
                          const std::string& easing, int samples = 8) const {
            return call("command.applyEasing", "{\"commands\":" + commands + ",\"kind\":"
                + detail::quote(kind) + ",\"easing\":" + detail::quote(easing)
                + ",\"samples\":" + std::to_string(samples) + "}");
        }

        /// Re-eases a command channel with a cubic-bezier, resolving it into commands.
        Json apply_bezier(const std::string& commands, const std::string& kind, double x1,
                          double y1, double x2, double y2, int samples = 8) const {
            return call("command.applyBezier", "{\"commands\":" + commands + ",\"kind\":"
                + detail::quote(kind) + ",\"x1\":" + detail::number(x1) + ",\"y1\":"
                + detail::number(y1) + ",\"x2\":" + detail::number(x2) + ",\"y2\":"
                + detail::number(y2) + ",\"samples\":" + std::to_string(samples) + "}");
        }

        // -- colors --

        /// Converts a color into its (x, y, z, w) components in `space` (raw Json).
        Json color_to_space(const std::string& color_json, const std::string& space = "rgb") const {
            return call("color.toSpace",
                "{\"color\":" + color_json + ",\"space\":" + detail::quote(space) + "}");
        }

        /// Builds a color from (x, y, z, w) components in `space`.
        Json color_from_space(const std::string& value_json, const std::string& space = "rgb") const {
            return call("color.fromSpace",
                "{\"value\":" + value_json + ",\"space\":" + detail::quote(space) + "}");
        }

        /// Adds a per-component `delta` in `space`.
        Json color_offset(const std::string& color_json, const std::string& delta_json,
                          const std::string& space = "rgb") const {
            return call("color.offset", "{\"color\":" + color_json + ",\"delta\":" + delta_json
                + ",\"space\":" + detail::quote(space) + "}");
        }

        /// Multiplies each component by a per-component `factor` in `space`.
        Json color_scale(const std::string& color_json, const std::string& factor_json,
                         const std::string& space = "rgb") const {
            return call("color.scale", "{\"color\":" + color_json + ",\"factor\":" + factor_json
                + ",\"space\":" + detail::quote(space) + "}");
        }

        /// Applies a named CSS/Sass helper with extra options.
        Json color_apply(const std::string& color_json, const std::string& op,
                         const std::string& extra = "{}") const {
            std::string body = extra == "{}" ? std::string()
                                             : "," + extra.substr(1, extra.size() - 2);
            return call("color.apply",
                "{\"color\":" + color_json + ",\"op\":" + detail::quote(op) + body + "}");
        }

        /// Raises HSL lightness.
        Json lighten(const std::string& color_json, double amount) const {
            return color_apply(color_json, "lighten", "{\"amount\":" + detail::number(amount) + "}");
        }
        /// Lowers HSL lightness.
        Json darken(const std::string& color_json, double amount) const {
            return color_apply(color_json, "darken", "{\"amount\":" + detail::number(amount) + "}");
        }
        /// Raises HSL saturation.
        Json saturate(const std::string& color_json, double amount) const {
            return color_apply(color_json, "saturate", "{\"amount\":" + detail::number(amount) + "}");
        }
        /// Lowers HSL saturation.
        Json desaturate(const std::string& color_json, double amount) const {
            return color_apply(color_json, "desaturate", "{\"amount\":" + detail::number(amount) + "}");
        }
        /// Rotates the hue by `degrees`.
        Json adjust_hue(const std::string& color_json, double degrees) const {
            return color_apply(color_json, "adjustHue", "{\"degrees\":" + detail::number(degrees) + "}");
        }
        /// The 180-degree hue rotation.
        Json complement(const std::string& color_json) const { return color_apply(color_json, "complement"); }
        /// Drops HSL saturation to zero.
        Json grayscale(const std::string& color_json) const { return color_apply(color_json, "grayscale"); }
        /// The RGB inverse, keeping alpha.
        Json invert(const std::string& color_json) const { return color_apply(color_json, "invert"); }
        /// The Sass weighted mix, accounting for alpha.
        Json mix(const std::string& color_json, const std::string& other_json, double weight = 0.5) const {
            return color_apply(color_json, "mix",
                "{\"other\":" + other_json + ",\"weight\":" + detail::number(weight) + "}");
        }
        /// Mixes toward white.
        Json tint(const std::string& color_json, double weight = 0.5) const {
            return color_apply(color_json, "tint", "{\"weight\":" + detail::number(weight) + "}");
        }
        /// Mixes toward black.
        Json shade(const std::string& color_json, double weight = 0.5) const {
            return color_apply(color_json, "shade", "{\"weight\":" + detail::number(weight) + "}");
        }
        /// Raises alpha.
        Json opacify(const std::string& color_json, double amount) const {
            return color_apply(color_json, "opacify", "{\"amount\":" + detail::number(amount) + "}");
        }
        /// Lowers alpha.
        Json transparentize(const std::string& color_json, double amount) const {
            return color_apply(color_json, "transparentize", "{\"amount\":" + detail::number(amount) + "}");
        }
        /// Sets the alpha channel.
        Json with_alpha(const std::string& color_json, double alpha) const {
            return color_apply(color_json, "withAlpha", "{\"alpha\":" + detail::number(alpha) + "}");
        }

        /// Samples a multi-stop gradient at `t` (0..=1).
        Json gradient(const std::string& stops_json, double t, const std::string& space = "rgb",
                      bool repeat = false) const {
            return call("color.gradient", "{\"stops\":" + stops_json + ",\"t\":"
                + detail::number(t) + ",\"space\":" + detail::quote(space) + ",\"repeat\":"
                + (repeat ? "true" : "false") + "}");
        }

        /// Builds a color channel by sampling a gradient `count` times.
        Json gradient_channel(const std::string& stops_json, int count = 16, double start = 0.0,
                              double end = 1000.0, const std::string& space = "rgb",
                              bool repeat = false) const {
            return call("color.gradientChannel", "{\"stops\":" + stops_json + ",\"count\":"
                + std::to_string(count) + ",\"start\":" + detail::number(start) + ",\"end\":"
                + detail::number(end) + ",\"space\":" + detail::quote(space) + ",\"repeat\":"
                + (repeat ? "true" : "false") + "}");
        }

        // -- 2D scenes --

        /// Resolves a 2D scene into per-sprite command sets.
        Json resolve_scene2d(const std::string& scene_json, const std::string& options = "{}") const {
            return call("scene2d.resolve",
                "{\"scene\":" + scene_json + ",\"options\":" + options + "}");
        }

        /// Bakes a 2D scene into a full storyboard model (per-node layers).
        Json bake_scene2d(const std::string& scene_json, const std::string& options = "{}") const {
            return call("scene2d.bake",
                "{\"scene\":" + scene_json + ",\"options\":" + options + "}");
        }

        /// The scene's occupied [start, end] range.
        Json scene2d_time_range(const std::string& scene_json) const {
            return call("scene2d.timeRange", "{\"scene\":" + scene_json + "}");
        }

        /// Translates the whole scene (children inherit).
        Json translate_scene2d(const std::string& scene_json, const std::string& delta_json) const {
            return call("scene2d.translate",
                "{\"scene\":" + scene_json + ",\"delta\":" + delta_json + "}");
        }

        /// Scales the whole scene about its origin.
        Json scale_scene2d(const std::string& scene_json, double factor) const {
            return call("scene2d.scale",
                "{\"scene\":" + scene_json + ",\"factor\":" + detail::number(factor) + "}");
        }

        /// Rotates the whole scene (radians).
        Json rotate_scene2d(const std::string& scene_json, double radians) const {
            return call("scene2d.rotate",
                "{\"scene\":" + scene_json + ",\"radians\":" + detail::number(radians) + "}");
        }

        /// Flattens the whole scene.
        Json flatten_scene2d(const std::string& scene_json, const std::string& axis = "",
                             const std::string& mode = "mean") const {
            return call("scene2d.flatten", "{\"scene\":" + scene_json + ",\"axis\":"
                + (axis.empty() ? "null" : detail::quote(axis)) + ",\"mode\":"
                + detail::quote(mode) + "}");
        }

        /// Shifts the whole scene in time.
        Json shift_scene2d(const std::string& scene_json, double delta) const {
            return call("scene2d.shift",
                "{\"scene\":" + scene_json + ",\"delta\":" + detail::number(delta) + "}");
        }

        /// Reverses the whole scene in time.
        Json reverse_scene2d(const std::string& scene_json, double start = 0.0,
                             double end = 0.0) const {
            return call("scene2d.reverse", "{\"scene\":" + scene_json + ",\"start\":"
                + detail::number(start) + ",\"end\":" + detail::number(end) + "}");
        }

        // -- 3D worlds / storyboard bake --

        /// Bakes a 3D world (`{ nodes, camera, options }`) into a storyboard model.
        Json bake_world(const std::string& world_json, const std::string& options2d = "{}") const {
            return call("scene3d.bakeWorld",
                "{\"world\":" + world_json + ",\"options2d\":" + options2d + "}");
        }

        /// Bakes a set of 2D and 3D worlds (`{ worlds2d, worlds3d, ... }`) into one storyboard.
        Json bake_scene(const std::string& input_json) const {
            return call("scene3d.bakeScene", input_json);
        }

    private:
        const Library* library_;

        static std::string point(Vector2 value) {
            return "{\"x\":" + detail::number(value.x) + ",\"y\":" + detail::number(value.y) + "}";
        }

        static std::string channel_json(const Channel& channel) {
            std::string out = "{\"keys\":[";
            for (std::size_t i = 0; i < channel.keys.size(); ++i) {
                if (i != 0) {
                    out += ",";
                }
                const ChannelKey& key = channel.keys[i];
                out += "{\"time\":" + detail::number(key.time) + ",\"value\":[";
                for (std::size_t j = 0; j < key.value.size(); ++j) {
                    if (j != 0) {
                        out += ",";
                    }
                    out += detail::number(key.value[j]);
                }
                out += "]}";
            }
            return out + "]}";
        }

        static Channel parse_channel(const Json& json) {
            Channel channel;
            if (const Json* keys = json.find("keys"); keys != nullptr && keys->is_array()) {
                for (const Json& key : keys->array()) {
                    ChannelKey entry;
                    if (const Json* time = key.find("time"); time != nullptr) {
                        entry.time = time->number();
                    }
                    if (const Json* values = key.find("value"); values != nullptr && values->is_array()) {
                        for (const Json& value : values->array()) {
                            entry.value.push_back(value.number());
                        }
                    }
                    channel.keys.push_back(entry);
                }
            }
            return channel;
        }

        Channel channel_call(const std::string& name, const std::string& input_json) const {
            return parse_channel(call(name, input_json));
        }
    };

    /// The 3D keyframe helpers facade.
    Helpers helpers() const { return Helpers(*library_); }
};

/// The interpolation a control-point list is interpreted as.
enum class CurveKind { Linear, Quadratic, Cubic, Bezier, CatmullRom };

inline const char* to_string(CurveKind value) {
    switch (value) {
        case CurveKind::Linear: return "Linear";
        case CurveKind::Quadratic: return "Quadratic";
        case CurveKind::Cubic: return "Cubic";
        case CurveKind::Bezier: return "Bezier";
        case CurveKind::CatmullRom: return "CatmullRom";
    }
    return "Linear";
}

/// A curve: the interpolation plus its control points.
struct CurveSpec {
    CurveKind kind = CurveKind::Linear;
    std::vector<Vector2> points;
};

/// An axis-aligned 2D bounding box.
struct Box2 {
    Vector2 min;
    Vector2 max;
};

/// A list of points (a curve sample or a simplified polyline).
struct PointList {
    std::vector<Vector2> points;
};

/// The curve library: Bezier/Catmull-Rom curves, sampling, arc length, tangents,
/// bounding boxes and Douglas-Peucker simplification.
class Curve {
public:
    explicit Curve(const Library& library) : library_(&library) {}

    /// The arc length of a curve.
    double length(const CurveSpec& curve) const {
        return call("Core.length", "{\"curve\":" + spec(curve) + "}").at("value").number();
    }

    /// The axis-aligned bounding box of a curve.
    Box2 bounding_box(const CurveSpec& curve) const {
        Json json = call("Core.boundingBox", "{\"curve\":" + spec(curve) + "}");
        return Box2{parse_vec2(json.at("min")), parse_vec2(json.at("max"))};
    }

    /// The point at `t`, parametrically or by arc length.
    Vector2 point(const CurveSpec& curve, double t, bool by_arc_length = false) const {
        Json json = call("Core.point", "{\"curve\":" + spec(curve) + ",\"t\":" + detail::number(t)
                                          + ",\"byArcLength\":" + (by_arc_length ? "true" : "false") + "}");
        return parse_vec2(json);
    }

    /// The unit tangent at `t`.
    Vector2 tangent(const CurveSpec& curve, double t) const {
        Json json = call("Core.tangent", "{\"curve\":" + spec(curve) + ",\"t\":" + detail::number(t) + "}");
        return parse_vec2(json);
    }

    /// `count` points along the curve, parametrically or by arc length.
    PointList sample(const CurveSpec& curve, int count, bool by_arc_length = false) const {
        Json json = call("Core.sample", "{\"curve\":" + spec(curve) + ",\"count\":" + std::to_string(count)
                                           + ",\"byArcLength\":" + (by_arc_length ? "true" : "false") + "}");
        return parse_points(json);
    }

    /// Simplifies a polyline with the Douglas-Peucker algorithm.
    PointList simplify(const std::vector<Vector2>& points, double tolerance) const {
        std::string body = "{\"points\":[";
        for (std::size_t index = 0; index < points.size(); ++index) {
            if (index != 0) {
                body += ",";
            }
            body += vec(points[index]);
        }
        body += "],\"tolerance\":" + detail::number(tolerance) + "}";
        return parse_points(call("Core.simplify", body));
    }

    /// The unit normal at `t`.
    Vector2 normal(const CurveSpec& curve, double t) const {
        Json json = call("Core.normal", "{\"curve\":" + spec(curve) + ",\"t\":" + detail::number(t) + "}");
        return parse_vec2(json);
    }

    /// The curvature at `t`.
    double curvature(const CurveSpec& curve, double t) const {
        return call("Core.curvature", "{\"curve\":" + spec(curve) + ",\"t\":" + detail::number(t) + "}")
            .at("value")
            .number();
    }

    /// The distance from `point` to the curve.
    double distance_to(const CurveSpec& curve, Vector2 point) const {
        Json json =
            call("Core.distanceTo", "{\"curve\":" + spec(curve) + ",\"point\":" + vec(point) + "}");
        return json.at("value").number();
    }

    /// The polyline offset along the curve's local normal.
    PointList offset(const CurveSpec& curve, double distance) const {
        Json json = call("Core.offset", "{\"curve\":" + spec(curve) + ",\"distance\":" + detail::number(distance) + "}");
        return parse_points(json);
    }

    /// Points along a circular arc.
    PointList arc(Vector2 center, double radius, double start_angle, double sweep_angle, int count) const {
        Json json = call("Core.arc", "{\"center\":" + vec(center) + ",\"radius\":" + detail::number(radius)
                                         + ",\"startAngle\":" + detail::number(start_angle)
                                         + ",\"sweepAngle\":" + detail::number(sweep_angle)
                                         + ",\"count\":" + std::to_string(count) + "}");
        return parse_points(json);
    }

    /// Points around an ellipse.
    PointList ellipse(Vector2 center, double radius_x, double radius_y, int count) const {
        Json json = call("Core.ellipse", "{\"center\":" + vec(center) + ",\"radiusX\":" + detail::number(radius_x)
                                             + ",\"radiusY\":" + detail::number(radius_y)
                                             + ",\"count\":" + std::to_string(count) + "}");
        return parse_points(json);
    }

    /// The intersection of two line segments (a JSON object with a nullable `point`).
    Json line_intersect(Vector2 a1, Vector2 a2, Vector2 b1, Vector2 b2) const {
        return call("Core.lineIntersect", "{\"a1\":" + vec(a1) + ",\"a2\":" + vec(a2) + ",\"b1\":" + vec(b1)
                                         + ",\"b2\":" + vec(b2) + "}");
    }

private:
    const Library* library_;

    static std::string vec(Vector2 value) {
        return "{\"x\":" + detail::number(value.x) + ",\"y\":" + detail::number(value.y) + "}";
    }

    static std::string spec(const CurveSpec& curve) {
        std::string body = "{\"kind\":" + detail::quote(to_string(curve.kind)) + ",\"points\":[";
        for (std::size_t index = 0; index < curve.points.size(); ++index) {
            if (index != 0) {
                body += ",";
            }
            body += vec(curve.points[index]);
        }
        body += "]}";
        return body;
    }

    static PointList parse_points(const Json& json) {
        PointList list;
        if (const Json* points = json.find("points"); points != nullptr && points->is_array()) {
            for (const Json& point : points->array()) {
                list.points.push_back(parse_vec2(point));
            }
        }
        return list;
    }

    Json call(const std::string& op, const std::string& body) const {
        Json json = detail::parse_json(library_->exec("{\"op\":" + detail::quote(op) + "," + body.substr(1)));
        if (const Json* error = json.find("error"); error != nullptr) {
            throw std::runtime_error("osbtk: " + error->string());
        }
        return json;
    }
};

/// The diagnostics library: storyboard and mapset analysis.
class Diagnostics {
public:
    explicit Diagnostics(const Library& library) : library_(&library) {}

    /// Diagnoses an `.osb` storyboard (the raw report JSON).
    Json diagnose_osb(const std::string& osb_text) const {
        return call("Diagnostics.diagnoseOsb", "{\"text\":" + detail::quote(osb_text) + "}");
    }

    /// Diagnoses an `.osu` beatmap (the raw report JSON).
    Json diagnose_osu(const std::string& osu_text) const {
        return call("Diagnostics.diagnoseOsu", "{\"text\":" + detail::quote(osu_text) + "}");
    }

private:
    const Library* library_;

    Json call(const std::string& op, const std::string& body) const {
        Json json = detail::parse_json(library_->exec("{\"op\":" + detail::quote(op) + "," + body.substr(1)));
        if (const Json* error = json.find("error"); error != nullptr) {
            throw std::runtime_error("osbtk: " + error->string());
        }
        return json;
    }
};

/// The optimization library: storyboard optimization and `.osb` round-trip.
class Optimize {
public:
    explicit Optimize(const Library& library) : library_(&library) {}

    /// Optimizes an `.osb` storyboard (the raw result JSON: `osb` + `stats`).
    Json optimize_osb(const std::string& osb_text, bool deduplicate = false) const {
        std::string options =
            "{\"deduplicate\":" + std::string(deduplicate ? "true" : "false") + "}";
        return call("Optimize.optimizeOsb",
                    "{\"text\":" + detail::quote(osb_text) + ",\"options\":" + options + "}");
    }

private:
    const Library* library_;

    Json call(const std::string& op, const std::string& body) const {
        Json json = detail::parse_json(library_->exec("{\"op\":" + detail::quote(op) + "," + body.substr(1)));
        if (const Json* error = json.find("error"); error != nullptr) {
            throw std::runtime_error("osbtk: " + error->string());
        }
        return json;
    }
};

/// The image library: decode/encode and processing.
class Image {
public:
    explicit Image(const Library& library) : library_(&library) {}

    /// Reads image metadata (the raw `ImageInfo` JSON).
    Json info(const std::string& path) const {
        return call("Image.info", "{\"path\":" + detail::quote(path) + "}");
    }

    /// Processes an image (read, apply options, write).
    Json process(const std::string& path, const std::string& output, const std::string& options_json = "{}") const {
        return call("Image.process", "{\"path\":" + detail::quote(path) + ",\"output\":" + detail::quote(output)
                                        + ",\"options\":" + options_json + "}");
    }

private:
    const Library* library_;

    Json call(const std::string& op, const std::string& body) const {
        Json json = detail::parse_json(library_->exec("{\"op\":" + detail::quote(op) + "," + body.substr(1)));
        if (const Json* error = json.find("error"); error != nullptr) {
            throw std::runtime_error("osbtk: " + error->string());
        }
        return json;
    }
};

/// The image-generation library: draw primitive shapes.
class ImageGen {
public:
    explicit ImageGen(const Library& library) : library_(&library) {}

    /// Renders primitive shapes (the request JSON minus `op`; returns `ImageInfo`).
    Json render_shapes(const std::string& request_json) const {
        return call("Imagegen.renderShapes", request_json);
    }

private:
    const Library* library_;

    Json call(const std::string& op, const std::string& body) const {
        Json json = detail::parse_json(library_->exec("{\"op\":" + detail::quote(op) + "," + body.substr(1)));
        if (const Json* error = json.find("error"); error != nullptr) {
            throw std::runtime_error("osbtk: " + error->string());
        }
        return json;
    }
};

/// The media library: probing, frame extraction/encoding and audio.
class Media {
public:
    explicit Media(const Library& library) : library_(&library) {}

    /// Probes a media file (the raw `MediaInfo` JSON).
    Json info(const std::string& path) const {
        return call("Media.info", "{\"path\":" + detail::quote(path) + "}");
    }

    /// Extracts frames (the request JSON minus `op`; returns `FrameResult`).
    Json extract_frames(const std::string& request_json) const {
        return call("Media.extractFrames", request_json);
    }

    /// Encodes an image sequence (the request JSON minus `op`; returns `MediaInfo`).
    Json encode_video(const std::string& request_json) const {
        return call("Media.encodeVideo", request_json);
    }

    /// Extracts the audio track of a media file.
    Json extract_audio(const std::string& input, const std::string& output) const {
        return call("Media.extractAudio", "{\"input\":" + detail::quote(input) + ",\"output\":" + detail::quote(output) + "}");
    }

private:
    const Library* library_;

    Json call(const std::string& op, const std::string& body) const {
        Json json = detail::parse_json(library_->exec("{\"op\":" + detail::quote(op) + "," + body.substr(1)));
        if (const Json* error = json.find("error"); error != nullptr) {
            throw std::runtime_error("osbtk: " + error->string());
        }
        return json;
    }
};

/// The stateless SDK entry point: version and easing over the storyboarding and
/// 3D libraries (which it reuses by inheritance).
class Client : public Storyboarding3d {
public:
    using Storyboarding3d::Storyboarding3d;

    std::string version() const { return library_->version(); }

    double apply_easing(const std::string& easing, double t) const {
        Json json = call("Core.applyEasing", "{\"easing\":" + detail::quote(easing) + ",\"t\":" + detail::number(t) + "}");
        return json.at("value").number();
    }
};

/// A live storyboard runtime bound to a native handle.
class Runtime {
public:
    Runtime(const Library& library, double width = 640.0, double height = 480.0)
        : native_(library, width, height) {}

    Handle create_sprite(const std::string& layer, const std::string& path, OsbOrigin origin = OsbOrigin::Centre) {
        Json json = call("Layer.createSprite",
                         "{\"layer\":" + detail::quote(layer) + ",\"path\":" + detail::quote(path)
                             + ",\"origin\":" + detail::quote(to_string(origin)) + "}");
        return Handle{static_cast<int>(json.at("layer").number()), static_cast<int>(json.at("sprite").number())};
    }

    void move(const Handle& handle, double start_time, double end_time, Vector2 start, Vector2 end) {
        call("Sprite.command",
             "{\"kind\":\"move\",\"layer\":" + std::to_string(handle.layer) + ",\"sprite\":"
                 + std::to_string(handle.sprite) + ",\"startTime\":" + detail::number(start_time)
                 + ",\"endTime\":" + detail::number(end_time) + ",\"start\":" + vec(start)
                 + ",\"end\":" + vec(end) + "}");
    }

    SpriteState sprite_state_at(const Handle& handle, double time) {
        Json json = call("Sprite.stateAt",
                         "{\"layer\":" + std::to_string(handle.layer) + ",\"sprite\":"
                             + std::to_string(handle.sprite) + ",\"time\":" + detail::number(time) + "}");
        return parse_state(json);
    }

    /// The number of sprites on the storyboard.
    int sprite_count() { return static_cast<int>(call("Generator.spriteCount", "{}").at("spriteCount").number()); }

    /// The built storyboard model (`storyboard`).
    Json storyboard() { return call("Generator.storyboard", "{}"); }

    /// Adds a sound sample to a layer (`CreateSample`).
    void create_sample(const std::string& layer, const std::string& path, double time, double volume) {
        call("Layer.createSample", "{\"layer\":" + detail::quote(layer) + ",\"path\":" + detail::quote(path)
                                 + ",\"time\":" + detail::number(time) + ",\"volume\":" + detail::number(volume) + "}");
    }

    /// The FFT magnitudes of the loaded audio (`GetFft`).
    Json get_fft(double time, int magnitudes) {
        return call("Audio.getFft", "{\"time\":" + detail::number(time) + ",\"magnitudes\":" + std::to_string(magnitudes) + "}");
    }

    /// The active red timing point (`GetTimingPointAt`).
    Json timing_point_at(double time) {
        return call("Generator.timingPointAt", "{\"time\":" + detail::number(time) + "}");
    }

    /// Runs a raw bridge request (`op` + a JSON body) against this runtime.
    Json exec(const std::string& op, const std::string& body) { return call(op, body); }

private:
    osbtk::Runtime native_;

    static std::string vec(Vector2 value) {
        return "{\"x\":" + detail::number(value.x) + ",\"y\":" + detail::number(value.y) + "}";
    }

    Json call(const std::string& op, const std::string& body) {
        std::string fields = body;
        if (!fields.empty() && fields.front() == '{' && fields.back() == '}') {
            fields = fields.substr(1, fields.size() - 2);
        }
        std::string request = "{\"op\":\"" + op + "\"";
        if (!fields.empty()) {
            request += "," + fields;
        }
        request += "}";
        Json json = detail::parse_json(native_.exec(request));
        if (const Json* error = json.find("error"); error != nullptr) {
            throw std::runtime_error("osbtk: " + error->string());
        }
        return json;
    }
};

/// A storyboard sprite (`OsbSprite`).
class Sprite {
public:
    Sprite(Runtime& runtime, int layer, int sprite) : runtime_(&runtime), layer_(layer), sprite_(sprite) {}

    /// The sprite's initial state and metadata.
    Json info() { return runtime_->exec("Sprite.info", handle_json()); }

    /// `Move`.
    Sprite& move(double start_time, double end_time, Vector2 start, Vector2 end, const std::string& easing = "None") {
        return add("move", start_time, end_time, easing, vec(start), vec(end));
    }

    /// `Move` (easing-first).
    Sprite& move(const std::string& easing, double start_time, double end_time, Vector2 start, Vector2 end) {
        return add("move", start_time, end_time, easing, vec(start), vec(end));
    }

    /// `Move` (instant).
    Sprite& move(double time, Vector2 position) {
        return add("move", time, time, "None", vec(position), vec(position));
    }

    /// `Scale`.
    Sprite& scale(double start_time, double end_time, double start, double end, const std::string& easing = "None") {
        return add("scale", start_time, end_time, easing, detail::number(start), detail::number(end));
    }

    /// `Scale` (easing-first).
    Sprite& scale(const std::string& easing, double start_time, double end_time, double start, double end) {
        return add("scale", start_time, end_time, easing, detail::number(start), detail::number(end));
    }

    /// `Scale` (instant).
    Sprite& scale(double time, double value) {
        return add("scale", time, time, "None", detail::number(value), detail::number(value));
    }

    /// `Rotate` (radians).
    Sprite& rotate(double start_time, double end_time, double start, double end, const std::string& easing = "None") {
        return add("rotate", start_time, end_time, easing, detail::number(start), detail::number(end));
    }

    /// `Rotate` (easing-first).
    Sprite& rotate(const std::string& easing, double start_time, double end_time, double start, double end) {
        return add("rotate", start_time, end_time, easing, detail::number(start), detail::number(end));
    }

    /// `Rotate` (instant).
    Sprite& rotate(double time, double value) {
        return add("rotate", time, time, "None", detail::number(value), detail::number(value));
    }

    /// `Fade`.
    Sprite& fade(double start_time, double end_time, double start, double end, const std::string& easing = "None") {
        return add("fade", start_time, end_time, easing, detail::number(start), detail::number(end));
    }

    /// `Fade` (easing-first).
    Sprite& fade(const std::string& easing, double start_time, double end_time, double start, double end) {
        return add("fade", start_time, end_time, easing, detail::number(start), detail::number(end));
    }

    /// `Fade` (instant).
    Sprite& fade(double time, double opacity) {
        return add("fade", time, time, "None", detail::number(opacity), detail::number(opacity));
    }

    /// `MoveX`.
    Sprite& move_x(double start_time, double end_time, double start, double end, const std::string& easing = "None") {
        return add("moveX", start_time, end_time, easing, detail::number(start), detail::number(end));
    }

    /// `MoveX` (easing-first).
    Sprite& move_x(const std::string& easing, double start_time, double end_time, double start, double end) {
        return add("moveX", start_time, end_time, easing, detail::number(start), detail::number(end));
    }

    /// `MoveX` (instant).
    Sprite& move_x(double time, double x) {
        return add("moveX", time, time, "None", detail::number(x), detail::number(x));
    }

    /// `MoveY`.
    Sprite& move_y(double start_time, double end_time, double start, double end, const std::string& easing = "None") {
        return add("moveY", start_time, end_time, easing, detail::number(start), detail::number(end));
    }

    /// `MoveY` (easing-first).
    Sprite& move_y(const std::string& easing, double start_time, double end_time, double start, double end) {
        return add("moveY", start_time, end_time, easing, detail::number(start), detail::number(end));
    }

    /// `MoveY` (instant).
    Sprite& move_y(double time, double y) {
        return add("moveY", time, time, "None", detail::number(y), detail::number(y));
    }

    /// `ScaleVec`.
    Sprite& scale_vec(double start_time, double end_time, Vector2 start, Vector2 end, const std::string& easing = "None") {
        return add("scaleVec", start_time, end_time, easing, vec(start), vec(end));
    }

    /// `ScaleVec` (easing-first).
    Sprite& scale_vec(const std::string& easing, double start_time, double end_time, Vector2 start, Vector2 end) {
        return add("scaleVec", start_time, end_time, easing, vec(start), vec(end));
    }

    /// `ScaleVec` (instant).
    Sprite& scale_vec(double time, Vector2 scale) {
        return add("scaleVec", time, time, "None", vec(scale), vec(scale));
    }

    /// `Color`.
    Sprite& color(double start_time, double end_time, Color4 start, Color4 end, const std::string& easing = "None") {
        return add("color", start_time, end_time, easing, color_json(start), color_json(end));
    }

    /// `Color` (easing-first).
    Sprite& color(const std::string& easing, double start_time, double end_time, Color4 start, Color4 end) {
        return add("color", start_time, end_time, easing, color_json(start), color_json(end));
    }

    /// `Color` (instant).
    Sprite& color(double time, Color4 value) {
        return add("color", time, time, "None", color_json(value), color_json(value));
    }

    /// `ColorHsb`.
    Sprite& color_hsb(double start_time, double end_time, Color4 start, Color4 end, const std::string& easing = "None") {
        return add("colorHsb", start_time, end_time, easing, color_json(start), color_json(end));
    }

    /// `ColorHsb` (easing-first).
    Sprite& color_hsb(const std::string& easing, double start_time, double end_time, Color4 start, Color4 end) {
        return add("colorHsb", start_time, end_time, easing, color_json(start), color_json(end));
    }

    /// `ColorHsb` (instant).
    Sprite& color_hsb(double time, Color4 value) {
        return add("colorHsb", time, time, "None", color_json(value), color_json(value));
    }

    /// `Parameter`.
    Sprite& parameter(const std::string& parameter, double start_time, double end_time, const std::string& easing = "None") {
        std::string body = "{\"kind\":\"parameter\",\"layer\":" + std::to_string(layer_) + ",\"sprite\":" + std::to_string(sprite_)
                           + ",\"startTime\":" + detail::number(start_time) + ",\"endTime\":" + detail::number(end_time)
                           + ",\"easing\":" + detail::quote(easing) + ",\"start\":0,\"end\":1,\"parameter\":"
                           + detail::quote(parameter) + "}";
        runtime_->exec("Sprite.command", body);
        return *this;
    }

    /// `FlipH`.
    Sprite& flip_h(double start_time, double end_time) {
        return parameter("FlipHorizontal", start_time, end_time);
    }

    /// `FlipH` (instant).
    Sprite& flip_h(double time) { return flip_h(time, time); }

    /// `FlipV`.
    Sprite& flip_v(double start_time, double end_time) {
        return parameter("FlipVertical", start_time, end_time);
    }

    /// `FlipV` (instant).
    Sprite& flip_v(double time) { return flip_v(time, time); }

    /// `Additive`.
    Sprite& additive(double start_time, double end_time) {
        return parameter("Additive", start_time, end_time);
    }

    /// `Additive` (instant).
    Sprite& additive(double time) { return additive(time, time); }

    /// `Anchor`: an animatable origin offset in texture pixels.
    Sprite& anchor(double start_time, double end_time, Vector2 start, Vector2 end, const std::string& easing = "None") {
        return add("anchor", start_time, end_time, easing, vec(start), vec(end));
    }

    /// `Anchor` (easing-first).
    Sprite& anchor(const std::string& easing, double start_time, double end_time, Vector2 start, Vector2 end) {
        return add("anchor", start_time, end_time, easing, vec(start), vec(end));
    }

    /// `Anchor` (instant).
    Sprite& anchor(double time, Vector2 point) {
        return add("anchor", time, time, "None", vec(point), vec(point));
    }

    /// Sets a constant origin offset.
    Sprite& set_anchor(Vector2 point) {
        return add("anchor", 0, 0, "None", vec(point), vec(point));
    }

    /// The sampled origin offset at a time (`AnchorAt`).
    Vector2 anchor_at(double time) { return parse_vec2(state_at(time).at("anchor")); }

    /// `StartLoopGroup`.
    Sprite& start_loop_group(double start_time, double count) {
        runtime_->exec("Sprite.startLoopGroup", "{\"layer\":" + std::to_string(layer_) + ",\"sprite\":" + std::to_string(sprite_)
                                             + ",\"startTime\":" + detail::number(start_time) + ",\"count\":" + detail::number(count) + "}");
        return *this;
    }

    /// `EndGroup`.
    Sprite& end_group() {
        runtime_->exec("Sprite.endGroup", "{\"layer\":" + std::to_string(layer_) + ",\"sprite\":" + std::to_string(sprite_) + "}");
        return *this;
    }

    /// The sampled position at a time (`PositionAt`).
    Vector2 position_at(double time) { return parse_vec2(state_at(time).at("position")); }

    /// The sampled opacity at a time (`OpacityAt`).
    double opacity_at(double time) { return state_at(time).at("opacity").number(); }

    int layer() const { return layer_; }
    int sprite() const { return sprite_; }

private:
    std::string handle_json() const {
        return "{\"layer\":" + std::to_string(layer_) + ",\"sprite\":" + std::to_string(sprite_) + "}";
    }

    Json state_at(double time) {
        return runtime_->exec("Sprite.stateAt", "{\"layer\":" + std::to_string(layer_) + ",\"sprite\":" + std::to_string(sprite_)
                                                   + ",\"time\":" + detail::number(time) + "}");
    }

    Sprite& add(const std::string& kind, double start_time, double end_time, const std::string& easing,
                const std::string& start, const std::string& end) {
        runtime_->exec("Sprite.command", "{\"kind\":" + detail::quote(kind) + ",\"layer\":" + std::to_string(layer_)
                                     + ",\"sprite\":" + std::to_string(sprite_) + ",\"startTime\":" + detail::number(start_time)
                                     + ",\"endTime\":" + detail::number(end_time) + ",\"easing\":" + detail::quote(easing)
                                     + ",\"start\":" + start + ",\"end\":" + end + "}");
        return *this;
    }

    static std::string vec(Vector2 value) {
        return "{\"x\":" + detail::number(value.x) + ",\"y\":" + detail::number(value.y) + "}";
    }

    static std::string color_json(Color4 value) {
        return "{\"r\":" + detail::number(value.r) + ",\"g\":" + detail::number(value.g)
               + ",\"b\":" + detail::number(value.b) + ",\"a\":" + detail::number(value.a) + "}";
    }

    Runtime* runtime_;
    int layer_;
    int sprite_;
};

/// A storyboard segment (`StoryboardSegment`). The handle is implicit.
class Layer {
public:
    Layer(Runtime& runtime, std::string name, int handle)
        : runtime_(&runtime), name_(std::move(name)), handle_(handle) {}

    /// `CreateSprite`.
    Sprite create_sprite(const std::string& path, OsbOrigin origin = OsbOrigin::Centre) {
        Json handle = runtime_->exec("Layer.createSprite", "{\"layerHandle\":" + std::to_string(handle_) + ",\"path\":" + detail::quote(path)
                                                        + ",\"origin\":" + detail::quote(to_string(origin)) + "}");
        return Sprite(*runtime_, static_cast<int>(handle.at("layer").number()), static_cast<int>(handle.at("sprite").number()));
    }

    /// `CreateSample`.
    Layer& create_sample(const std::string& path, double time, double volume = 100.0) {
        runtime_->exec("Layer.createSample", "{\"layerHandle\":" + std::to_string(handle_) + ",\"path\":" + detail::quote(path)
                                          + ",\"time\":" + detail::number(time) + ",\"volume\":" + detail::number(volume) + "}");
        return *this;
    }

    /// `CreateSegment`: a segment nested in this one.
    Layer create_segment() {
        Json handle = runtime_->exec("Layer.createSegment", "{\"layer\":" + std::to_string(handle_) + "}");
        return Layer(*runtime_, std::string(), static_cast<int>(handle.at("layer").number()));
    }

    /// `ReverseDepth`.
    Layer& set_reverse_depth(bool value) {
        runtime_->exec("Layer.reverseDepth", "{\"layer\":" + std::to_string(handle_) + ",\"value\":" + (value ? "true" : "false") + "}");
        return *this;
    }

    /// `AddIndex`.
    Layer& add_index(double index) {
        runtime_->exec("Layer.addIndex", "{\"layer\":" + std::to_string(handle_) + ",\"index\":" + detail::number(index) + "}");
        return *this;
    }

    /// `Indexes`/`LowestIndex`/`HighestIndex`.
    Json indexes() { return runtime_->exec("Layer.indexes", "{\"layer\":" + std::to_string(handle_) + "}"); }

    /// `ChangePosition`.
    Layer& change_position(OsbLayer layer) {
        runtime_->exec("Layer.changePosition", "{\"layer\":" + std::to_string(handle_) + ",\"osbLayer\":" + detail::quote(to_string(layer)) + "}");
        return *this;
    }

    /// `MoveToTop`.
    Layer& move_to_top() {
        runtime_->exec("Layer.moveToTop", "{\"layer\":" + std::to_string(handle_) + "}");
        return *this;
    }

    /// `MoveToBottom`.
    Layer& move_to_bottom() {
        runtime_->exec("Layer.moveToBottom", "{\"layer\":" + std::to_string(handle_) + "}");
        return *this;
    }

    /// `Clean`.
    Json clean() { return runtime_->exec("Layer.clean", "{\"layer\":" + std::to_string(handle_) + "}"); }

    /// `Discard`.
    Json discard() { return runtime_->exec("Layer.discard", "{\"layer\":" + std::to_string(handle_) + "}"); }

    /// `DiscardSprite`.
    Layer& discard_sprite(const Sprite& sprite) {
        runtime_->exec("Layer.discardSprite", "{\"layer\":" + std::to_string(handle_) + ",\"sprite\":" + std::to_string(sprite.sprite()) + "}");
        return *this;
    }

    const std::string& name() const { return name_; }

private:
    Runtime* runtime_;
    std::string name_;
    int handle_;
};

/// A storyboard generator (the `StoryboardObjectGenerator` equivalent).
/// Subclass it and override `generate`.
class Generator {
public:
    explicit Generator(const Library& library, double width = 640.0, double height = 480.0)
        : runtime_(library, width, height) {}

    /// `CreateSprite`.
    Handle create_sprite(const std::string& layer, const std::string& path, OsbOrigin origin = OsbOrigin::Centre) {
        return runtime_.create_sprite(layer, path, origin);
    }

    /// `GetLayer`: a layer that creates sprites/animations/samples.
    Layer get_layer(const std::string& name) {
        Json handle = runtime_.exec("Generator.layerHandle", "{\"name\":" + detail::quote(name) + "}");
        return Layer(runtime_, name, static_cast<int>(handle.at("layer").number()));
    }

    /// `CreateSample`.
    void create_sample(const std::string& layer, const std::string& path, double time, double volume) {
        runtime_.create_sample(layer, path, time, volume);
    }

    /// The number of sprites on the storyboard.
    int sprite_count() { return runtime_.sprite_count(); }

    /// The built storyboard model (`storyboard`).
    Json storyboard() { return runtime_.storyboard(); }

    /// The underlying runtime (for command authoring).
    Runtime& runtime() { return runtime_; }

    /// `Generate`: build the storyboard.
    virtual void generate() = 0;

    virtual ~Generator() = default;

private:
    Runtime runtime_;
};

}  // namespace osbtk::sdk
