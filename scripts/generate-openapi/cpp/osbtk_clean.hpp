#pragma once
// Clean high-level C++ clients: one class per contract tag with `<X>Options`
// inputs, mirroring the generated clean clients in the other SDKs. The classes
// are thin wrappers over `osbtk_sdk.hpp`; the runtime classes are re-exported
// because they are already handle-bound.

#include <string>
#include <utility>
#include <vector>

#include "osbtk_sdk.hpp"

namespace osbtk::clean {

using Json = sdk::Json;
using Library = osbtk::Library;

/// A curve query.
struct CurveOptions {
    sdk::CurveSpec curve;
};

/// A curve query at a parametric or arc-length position.
struct CurvePointOptions {
    sdk::CurveSpec curve;
    double t = 0.0;
    bool by_arc_length = false;
};

/// A curve sampling request.
struct CurveSampleOptions {
    int count = 0;
    sdk::CurveSpec curve;
    bool by_arc_length = false;
};

/// A curve hit-test request.
struct CurveDistanceOptions {
    sdk::CurveSpec curve;
    sdk::Vector2 point;
};

/// A curve offset request.
struct CurveOffsetOptions {
    sdk::CurveSpec curve;
    double distance = 0.0;
};

/// A circular-arc sampling request.
struct ArcOptions {
    sdk::Vector2 center;
    double radius = 0.0;
    double start_angle = 0.0;
    double sweep_angle = 0.0;
    int count = 64;
};

/// An ellipse sampling request.
struct EllipseOptions {
    sdk::Vector2 center;
    double radius_x = 0.0;
    double radius_y = 0.0;
    int count = 64;
};

/// A line-segment intersection request.
struct LineIntersectOptions {
    sdk::Vector2 a1;
    sdk::Vector2 a2;
    sdk::Vector2 b1;
    sdk::Vector2 b2;
};

/// A polyline simplification request.
struct SimplifyOptions {
    std::vector<sdk::Vector2> points;
    double tolerance = 0.0;
};

/// The clean curve client.
class Curve {
public:
    explicit Curve(const Library& library) : impl_(library) {}

    double length(const CurveOptions& options) const { return impl_.length(options.curve); }
    sdk::Box2 bounding_box(const CurveOptions& options) const {
        return impl_.bounding_box(options.curve);
    }
    sdk::Vector2 point(const CurvePointOptions& options) const {
        return impl_.point(options.curve, options.t, options.by_arc_length);
    }
    sdk::Vector2 tangent(const CurvePointOptions& options) const {
        return impl_.tangent(options.curve, options.t);
    }
    sdk::PointList sample(const CurveSampleOptions& options) const {
        return impl_.sample(options.curve, options.count, options.by_arc_length);
    }
    sdk::PointList simplify(const SimplifyOptions& options) const {
        return impl_.simplify(options.points, options.tolerance);
    }
    sdk::Vector2 normal(const CurvePointOptions& options) const {
        return impl_.normal(options.curve, options.t);
    }
    double curvature(const CurvePointOptions& options) const {
        return impl_.curvature(options.curve, options.t);
    }
    double distance_to(const CurveDistanceOptions& options) const {
        return impl_.distance_to(options.curve, options.point);
    }
    sdk::PointList offset(const CurveOffsetOptions& options) const {
        return impl_.offset(options.curve, options.distance);
    }
    sdk::PointList arc(const ArcOptions& options) const {
        return impl_.arc(options.center, options.radius, options.start_angle, options.sweep_angle,
                         options.count);
    }
    sdk::PointList ellipse(const EllipseOptions& options) const {
        return impl_.ellipse(options.center, options.radius_x, options.radius_y, options.count);
    }
    Json line_intersect(const LineIntersectOptions& options) const {
        return impl_.line_intersect(options.a1, options.a2, options.b1, options.b2);
    }

private:
    sdk::Curve impl_;
};

/// A named easing evaluation request.
struct ApplyEasingOptions {
    std::string easing = "None";
    double t = 0.0;
};

/// A named-or-custom easing evaluation request.
struct EasingValueOptions {
    double t = 0.0;
    std::string easing = "None";
    std::vector<double> cubic_bezier;
};

/// The clean animations client.
class Animations {
public:
    explicit Animations(const Library& library) : library_(&library), impl_(library) {}

    double apply_easing(const ApplyEasingOptions& options) const {
        return impl_.apply_easing(options.easing, options.t);
    }

    double easing_value(const EasingValueOptions& options) const {
        std::string body = "{\"t\":" + sdk::detail::number(options.t);
        if (!options.cubic_bezier.empty()) {
            body += ",\"cubicBezier\":[";
            for (std::size_t index = 0; index < options.cubic_bezier.size(); ++index) {
                if (index != 0) body += ",";
                body += sdk::detail::number(options.cubic_bezier[index]);
            }
            body += "]";
        } else {
            body += ",\"easing\":" + sdk::detail::quote(options.easing);
        }
        body += "}";
        return call("Animations.easingValue", body).at("value").number();
    }

private:
    const Library* library_;
    sdk::Client impl_;
    Json call(const std::string& op, const std::string& body) const {
        Json json = sdk::detail::parse_json(
            library_->exec("{\"op\":" + sdk::detail::quote(op) + "," + body.substr(1)));
        if (const Json* error = json.find("error"); error != nullptr) {
            throw std::runtime_error("osbtk: " + error->string());
        }
        return json;
    }
};

/// An FFT request.
struct FftOptions {
    std::string path;
    double time = 0.0;
    int magnitudes = 64;
};

/// The clean audio client.
class Audio {
public:
    explicit Audio(const Library& library) : library_(&library) {}

    Json fft(const FftOptions& options) const {
        Json json = sdk::detail::parse_json(library_->exec(
            "{\"op\":\"Audio.fft\",\"path\":" + sdk::detail::quote(options.path)
            + ",\"time\":" + sdk::detail::number(options.time)
            + ",\"magnitudes\":" + std::to_string(options.magnitudes) + "}"));
        if (const Json* error = json.find("error"); error != nullptr) {
            throw std::runtime_error("osbtk: " + error->string());
        }
        return json;
    }

private:
    const Library* library_;
};

/// A request carrying a block of text.
struct TextOptions {
    std::string text;
};

/// The clean parsers.
class Parsing {
public:
    explicit Parsing(const Library& library) : impl_(library) {}

    sdk::Beatmap parse_osu(const TextOptions& options) const {
        return impl_.parse_beatmap(options.text);
    }
    Json parse_osb(const TextOptions& options) const { return impl_.parse_osb(options.text); }

private:
    sdk::Storyboarding impl_;
};

/// A request to resolve an origin's anchor offset.
struct GetOriginVectorOptions {
    sdk::OsbOrigin origin = sdk::OsbOrigin::Centre;
    double width = 0.0;
    double height = 0.0;
};

/// Whether a rotated sprite's bounding box intersects the screen.
struct InScreenBoundsOptions {
    sdk::Vector2 position;
    sdk::Vector2 size;
    double rotation = 0.0;
    sdk::Vector2 origin;
};

/// The clean origin client.
class Origin {
public:
    explicit Origin(const Library& library) : impl_(library) {}

    sdk::Vector2 get_origin_vector(const GetOriginVectorOptions& options) const {
        return impl_.get_origin_vector(options.origin, options.width, options.height);
    }
    bool in_screen_bounds(const InScreenBoundsOptions& options) const {
        return impl_.in_screen_bounds(options.position, options.size, options.rotation,
                                      options.origin);
    }

private:
    sdk::Storyboarding impl_;
};

/// The clean diagnostics client.
class Diagnostics {
public:
    explicit Diagnostics(const Library& library) : impl_(library) {}

    Json diagnose_osb(const TextOptions& options) const {
        return impl_.diagnose_osb(options.text);
    }
    Json diagnose_osu(const TextOptions& options) const {
        return impl_.diagnose_osu(options.text);
    }

private:
    sdk::Diagnostics impl_;
};

/// An optimization request.
struct OptimizeOptions {
    std::string text;
    bool deduplicate = false;
};

/// The clean optimization client.
class Optimize {
public:
    explicit Optimize(const Library& library) : impl_(library) {}

    Json optimize_osb(const OptimizeOptions& options) const {
        return impl_.optimize_osb(options.text, options.deduplicate);
    }

private:
    sdk::Optimize impl_;
};

/// A request carrying a single file path.
struct PathOptions {
    std::string path;
};

/// An image processing request (`options_json` is the `ImageOptions` object).
struct ImageProcessOptions {
    std::string path;
    std::string output;
    std::string options_json = "{}";
};

/// The clean image client.
class Image {
public:
    explicit Image(const Library& library) : impl_(library) {}

    Json info(const PathOptions& options) const { return impl_.info(options.path); }
    Json process(const ImageProcessOptions& options) const {
        return impl_.process(options.path, options.output, options.options_json);
    }

private:
    sdk::Image impl_;
};

/// A primitive-shape render request (`request_json` is the request body).
struct RenderShapesOptions {
    std::string request_json;
};

/// The clean image-generation client.
class ImageGen {
public:
    explicit ImageGen(const Library& library) : impl_(library) {}

    Json render_shapes(const RenderShapesOptions& options) const {
        return impl_.render_shapes(options.request_json);
    }

private:
    sdk::ImageGen impl_;
};

/// A frame-extraction request (`request_json` is the request body).
struct ExtractFramesOptions {
    std::string request_json;
};

/// A video-encoding request (`request_json` is the request body).
struct EncodeVideoOptions {
    std::string request_json;
};

/// An audio-extraction request.
struct ExtractAudioOptions {
    std::string input;
    std::string output;
};

/// The clean media client.
class Media {
public:
    explicit Media(const Library& library) : impl_(library) {}

    Json info(const PathOptions& options) const { return impl_.info(options.path); }
    Json extract_frames(const ExtractFramesOptions& options) const {
        return impl_.extract_frames(options.request_json);
    }
    Json encode_video(const EncodeVideoOptions& options) const {
        return impl_.encode_video(options.request_json);
    }
    Json extract_audio(const ExtractAudioOptions& options) const {
        return impl_.extract_audio(options.input, options.output);
    }

private:
    sdk::Media impl_;
};

/// A 3D bake request (`input_json` is the scene JSON).
struct Bake3dOptions {
    std::string input_json;
};

/// The clean 3D storyboard client.
class Storyboarding3d {
public:
    explicit Storyboarding3d(const Library& library) : impl_(library) {}

    Json bake3d(const Bake3dOptions& options) const { return impl_.bake3d(options.input_json); }

private:
    sdk::Storyboarding3d impl_;
};

/// A motion-graphics helper call (`input_json` is the helper input object).
struct HelperOptions {
    std::string name;
    std::string input_json = "{}";
};

/// The clean shared-helpers client.
class Helpers {
public:
    explicit Helpers(const Library& library) : impl_(library) {}

    Json call(const HelperOptions& options) const {
        return impl_.call(options.name, options.input_json);
    }

private:
    sdk::Storyboarding3d::Helpers impl_;
};

/// The clean common client.
class Common {
public:
    explicit Common(const Library& library) : library_(&library) {}
    std::string version() const { return library_->version(); }

private:
    const Library* library_;
};

/// The runtime classes are already high-level and handle-bound.
using Runtime = sdk::Runtime;
using Generator = sdk::Generator;
using Layer = sdk::Layer;
using Sprite = sdk::Sprite;

/// The composed stateless clean client.
class Client {
public:
    explicit Client(const Library& library)
        : common(library),
          curve(library),
          animations(library),
          parsing(library),
          origin(library),
          audio(library),
          diagnostics(library),
          optimize(library),
          image(library),
          image_gen(library),
          media(library),
          storyboarding3d(library),
          helpers(library) {}

    Common common;
    Curve curve;
    Animations animations;
    Parsing parsing;
    Origin origin;
    Audio audio;
    Diagnostics diagnostics;
    Optimize optimize;
    Image image;
    ImageGen image_gen;
    Media media;
    Storyboarding3d storyboarding3d;
    Helpers helpers;
};

} // namespace osbtk::clean
