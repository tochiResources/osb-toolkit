// The typed C++ SDK over the native bridge (osbtk_sdk.hpp).
#include <cmath>
#include <filesystem>
#include <iostream>
#include <string>

#include "osbtk_sdk.hpp"

namespace fs = std::filesystem;

static std::string find_library() {
    fs::path directory = fs::current_path();
    for (int depth = 0; depth < 8; ++depth) {
        for (const char* name : {"osb_native.dll", "libosb_native.so", "libosb_native.dylib"}) {
            for (const char* profile : {"release", "debug"}) {
                fs::path candidate = directory / "target" / profile / name;
                if (fs::exists(candidate)) {
                    return candidate.string();
                }
            }
        }
        if (!directory.has_parent_path()) {
            break;
        }
        directory = directory.parent_path();
    }
    return {};
}

static int fail(const std::string& message) {
    std::cerr << "FAIL: " << message << "\n";
    return 1;
}

int main() {
    std::string path = find_library();
    if (path.empty()) {
        return fail("osb_native library not found");
    }

    osbtk::Library lib(path);
    osbtk::sdk::Client client(lib);
    if (client.version().empty()) {
        return fail("empty version");
    }

    osbtk::sdk::Beatmap beatmap = client.parse_beatmap(
        "osu file format v14\n\n[General]\nAudioFilename: song.mp3\nMode: 0\n"
        "[TimingPoints]\n0,500,4,2,1,100,1,0\n[HitObjects]\n256,192,1000,1,0\n");
    if (beatmap.audio_filename != "song.mp3") {
        return fail("beatmap audio: " + beatmap.audio_filename);
    }
    if (beatmap.hit_objects.empty() || beatmap.hit_objects[0].kind != osbtk::sdk::HitObjectKind::Circle) {
        return fail("beatmap hit object kind");
    }
    if (beatmap.timing_points.empty() || beatmap.timing_points[0].bpm < 119.9) {
        return fail("beatmap timing point bpm");
    }

    double eased = client.apply_easing("InQuad", 0.5);
    if (eased < 0.249 || eased > 0.251) {
        return fail("easing: " + std::to_string(eased));
    }

    // M28: the 3D library reuses the storyboarding surface.
    osbtk::sdk::Storyboarding3d storyboarding3d(lib);
    osbtk::sdk::Beatmap reused = storyboarding3d.parse_beatmap(
        "osu file format v14\n\n[General]\nAudioFilename: song.mp3\nMode: 0\n"
        "[TimingPoints]\n0,500,4,2,1,100,1,0\n[HitObjects]\n256,192,1000,1,0\n");
    if (reused.audio_filename != "song.mp3") {
        return fail("storyboarding3d beatmap audio: " + reused.audio_filename);
    }
    osbtk::sdk::Json baked = storyboarding3d.bake3d(
        "{\"nodes\":[{\"sprite\":\"sb/dot.png\"}],\"camera\":{\"position\":[{\"time\":0,\"value\":[0,0,5]}]}}");
    if (baked.find("sprites") == nullptr || baked.at("sprites").array().size() != 1) {
        return fail("storyboarding3d bake3d sprites");
    }

    osbtk::sdk::Curve curve_lib(lib);
    osbtk::sdk::CurveSpec curve;
    curve.kind = osbtk::sdk::CurveKind::Linear;
    curve.points = {{0.0, 0.0}, {100.0, 0.0}};
    if (std::abs(curve_lib.length(curve) - 100.0) > 1e-6) {
        return fail("curve length");
    }
    if (std::abs(curve_lib.point(curve, 0.5).x - 50.0) > 1e-6) {
        return fail("curve point");
    }
    if (curve_lib.sample(curve, 3).points.size() != 3) {
        return fail("curve sample");
    }
    if (std::abs(curve_lib.bounding_box(curve).max.x - 100.0) > 1e-6) {
        return fail("curve bounding box");
    }
    osbtk::sdk::PointList simplified =
        curve_lib.simplify({{0.0, 0.0}, {1.0, 0.05}, {2.0, 0.0}, {3.0, 0.0}}, 0.1);
    if (simplified.points.size() != 2) {
        return fail("curve simplify");
    }
    if (std::abs(curve_lib.normal(curve, 0.5).x) > 1e-6) {
        return fail("curve normal");
    }
    if (std::abs(curve_lib.curvature(curve, 0.5)) > 1e-6) {
        return fail("curve curvature");
    }
    if (std::abs(curve_lib.distance_to(curve, {50.0, 10.0}) - 10.0) > 1e-6) {
        return fail("curve distance");
    }
    if (curve_lib.offset(curve, 5.0).points.size() != 2) {
        return fail("curve offset");
    }
    if (curve_lib.arc({0.0, 0.0}, 10.0, 0.0, 180.0, 3).points.size() != 3) {
        return fail("curve arc");
    }
    if (curve_lib.ellipse({0.0, 0.0}, 4.0, 2.0, 4).points.size() != 4) {
        return fail("curve ellipse");
    }
    osbtk::sdk::Json hit =
        curve_lib.line_intersect({0.0, 0.0}, {10.0, 10.0}, {0.0, 10.0}, {10.0, 0.0});
    if (std::abs(hit.at("point").at("x").number() - 5.0) > 1e-6) {
        return fail("curve lineIntersect");
    }

    osbtk::sdk::Diagnostics diagnostics(lib);
    osbtk::sdk::Json report = diagnostics.diagnose_osu(
        "osu file format v14\n\n[General]\nAudioFilename: song.mp3\nMode: 0\n");
    if (report.find("errorCount") == nullptr || report.at("errorCount").number() < 1) {
        return fail("diagnostics error count");
    }

    osbtk::sdk::Optimize optimize(lib);
    osbtk::sdk::Json optimized = optimize.optimize_osb(
        "[Events]\nSprite,Foreground,Centre,\"sb/dot.png\",320,240\n F,0,0,100,0,1\n F,0,0,100,0,1\n",
        true);
    if (optimized.at("stats").at("deduplicated").number() != 1) {
        return fail("optimize deduplicated");
    }

    osbtk::sdk::Image image_lib(lib);
    bool image_threw = false;
    try {
        image_lib.info("does-not-exist.png");
    } catch (const std::exception&) {
        image_threw = true;
    }
    if (!image_threw) {
        return fail("image error not surfaced");
    }

    osbtk::sdk::ImageGen image_gen(lib);
    osbtk::sdk::Json rendered = image_gen.render_shapes(
        "{\"width\":32,\"height\":32,\"shapes\":[{\"kind\":\"Rect\",\"center\":{\"x\":16,\"y\":16},"
        "\"size\":{\"x\":20,\"y\":20},\"fill\":{\"r\":10,\"g\":20,\"b\":30,\"a\":255}}],"
        "\"output\":\"osbtk-cpp-render.png\"}");
    if (rendered.at("width").number() != 32) {
        return fail("renderShapes width");
    }

    osbtk::sdk::Media media(lib);
    bool media_threw = false;
    try {
        media.info("does-not-exist.mp4");
    } catch (const std::exception&) {
        media_threw = true;
    }
    if (!media_threw) {
        return fail("media error not surfaced");
    }

    class Pulse : public osbtk::sdk::Generator {
    public:
        using Generator::Generator;
        void generate() override { create_sprite("Foreground", "sb/dot.png"); }
    };
    Pulse effect(lib);
    effect.generate();
    if (effect.sprite_count() != 1) {
        return fail("generator sprite count");
    }

    osbtk::sdk::Layer layer = effect.get_layer("Foreground");
    osbtk::sdk::Sprite sprite = layer.create_sprite("sb/dot.png");
    sprite.move("None", 0.0, 1000.0, {0.0, 0.0}, {100.0, 0.0})
        .start_loop_group(1000.0, 2.0)
        .fade(0.0, 100.0, 0.0, 1.0)
        .fade(200.0, 1.0)
        .end_group();
    if (std::abs(sprite.position_at(500.0).x - 50.0) > 0.1) {
        return fail("sprite position");
    }

    osbtk::sdk::Sprite anchored = effect.get_layer("Anchor").create_sprite("sb/dot.png");
    anchored.anchor("None", 0.0, 1000.0, {0.0, 0.0}, {100.0, 50.0});
    if (std::abs(anchored.anchor_at(500.0).x - 50.0) > 0.1 ||
        std::abs(anchored.anchor_at(500.0).y - 25.0) > 0.1) {
        return fail("sprite anchor");
    }

    int before = effect.sprite_count();
    osbtk::sdk::Json model = effect.storyboard();
    if (model.find("layers") == nullptr) {
        return fail("storyboard layers");
    }
    bool found_foreground = false;
    for (const osbtk::sdk::Json& entry : model.at("layers").array()) {
        if (entry.at("identifier").string() == "Foreground") {
            found_foreground = true;
            if (entry.at("sprites").array().empty()) {
                return fail("storyboard foreground sprites");
            }
            if (entry.at("sprites").array()[0].at("texturePath").string() != "sb/dot.png") {
                return fail("storyboard texture path");
            }
        }
    }
    if (!found_foreground) {
        return fail("storyboard foreground layer");
    }

    osbtk::sdk::Layer group = effect.get_layer("Group");
    group.create_sprite("sb/group.png");
    osbtk::sdk::Layer child = group.create_segment();
    child.create_sprite("sb/child.png");
    if (effect.sprite_count() != before + 2) {
        return fail("segment sprite count");
    }
    child.set_reverse_depth(true).add_index(3).add_index(-1);
    auto indexes = child.indexes();
    if (indexes.at("highest").number() != 3.0 || indexes.at("lowest").number() != -1.0) {
        return fail("segment indexes");
    }
    child.discard();
    if (effect.sprite_count() != before + 1) {
        return fail("segment discard count");
    }

    if (std::abs(client.get_origin_vector(osbtk::sdk::OsbOrigin::Centre, 100.0, 100.0).x - 50.0) > 1e-6) {
        return fail("origin vector");
    }
    if (!client.in_screen_bounds({427.0, 240.0}, {100.0, 100.0}, 0.0, {50.0, 50.0})) {
        return fail("in screen bounds");
    }

    osbtk::sdk::Runtime runtime(lib);
    osbtk::sdk::Handle handle = runtime.create_sprite("Foreground", "sb/dot.png");
    runtime.move(handle, 0.0, 1000.0, {0.0, 0.0}, {100.0, 0.0});
    osbtk::sdk::SpriteState state = runtime.sprite_state_at(handle, 500.0);
    if (state.position.x < 49.9 || state.position.x > 50.1) {
        return fail("runtime state x: " + std::to_string(state.position.x));
    }

    osbtk::sdk::Storyboarding3d::Helpers::Channel channel;
    channel.keys.push_back({0.0, {1.0, 2.0}});
    channel.keys.push_back({100.0, {3.0, 4.0}});
    auto scaled = client.helpers().scale_keyframes(channel, 2.0);
    if (scaled.keys.size() != 2 || std::abs(scaled.keys[1].value[0] - 6.0) > 1e-6
        || std::abs(scaled.keys[1].value[1] - 8.0) > 1e-6) {
        return fail("scale_keyframes");
    }
    auto orbit = client.helpers().orbit(10.0, 10.0, 0.0, 6.283185307179586, 5);
    if (orbit.keys.size() != 5) {
        return fail("orbit keys");
    }
    if (std::abs(client.helpers().stagger(3, 100.0) - 300.0) > 1e-6) {
        return fail("stagger");
    }

    std::string commands =
        "[{\"kind\":\"fade\",\"startTime\":0,\"endTime\":100,\"easing\":\"None\","
        "\"start\":{\"kind\":\"scalar\",\"value\":0},\"end\":{\"kind\":\"scalar\",\"value\":1}},"
        "{\"kind\":\"fade\",\"startTime\":200,\"endTime\":300,\"easing\":\"None\","
        "\"start\":{\"kind\":\"scalar\",\"value\":1},\"end\":{\"kind\":\"scalar\",\"value\":0}}]";
    auto shifted = client.commands().shift(commands, 50.0);
    if (shifted.array().size() != 2
        || std::abs(shifted.array()[0].at("startTime").number() - 50.0) > 1e-6) {
        return fail("shift_commands");
    }
    auto merged = client.commands().merge(commands, 0.0);
    if (merged.array().size() != 2) {
        return fail("merge_commands");
    }
    auto duplicated = client.commands().duplicate(commands, 1000.0, 2);
    if (duplicated.array().size() != 4) {
        return fail("duplicate_commands");
    }

    auto scaled_2d = client.helpers().call(
        "keyframe.scaleKeyframes",
        "{\"channel\":{\"keys\":[{\"time\":0,\"value\":[1,2]},{\"time\":100,\"value\":[3,4]}]},\"factor\":2}");
    if (std::abs(scaled_2d.at("keys").array()[1].at("value").array()[0].number() - 6.0) > 1e-6
        || std::abs(scaled_2d.at("keys").array()[1].at("value").array()[1].number() - 8.0) > 1e-6) {
        return fail("shared keyframe scale");
    }

    auto resolved = client.helpers().resolve_anchor("Centre", {10.0, -5.0}, 100.0, 100.0);
    if (std::abs(resolved.x - 60.0) > 1e-6 || std::abs(resolved.y - 45.0) > 1e-6) {
        return fail("resolve anchor");
    }

    std::string fade_commands =
        "[{\"kind\":\"fade\",\"startTime\":0,\"endTime\":100,\"easing\":\"None\","
        "\"start\":{\"kind\":\"scalar\",\"value\":0},\"end\":{\"kind\":\"scalar\",\"value\":1}},"
        "{\"kind\":\"fade\",\"startTime\":100,\"endTime\":200,\"easing\":\"None\","
        "\"start\":{\"kind\":\"scalar\",\"value\":1},\"end\":{\"kind\":\"scalar\",\"value\":0}}]";
    auto flattened = client.helpers().transform_commands(
        fade_commands, "fade", "keyframe.flatten", "{\"mode\":\"mean\"}");
    if (flattened.array().size() != 1
        || std::abs(flattened.array()[0].at("startTime").number()) > 1e-6
        || std::abs(flattened.array()[0].at("endTime").number() - 200.0) > 1e-6) {
        return fail("transform commands");
    }

    auto amplified = client.helpers().call(
        "easing.amplifyEasing", "{\"easing\":\"InQuad\",\"t\":0.5,\"amount\":2}");
    if (std::abs(amplified.number()) > 1e-6) {
        return fail("amplify easing");
    }
    auto anchor3 = client.helpers().call(
        "origin.resolveAnchor3d",
        "{\"origin\":\"Centre\",\"anchor\":{\"x\":10,\"y\":-5,\"z\":42},\"width\":100,\"height\":100}");
    if (std::abs(anchor3.at("z").number() - 42.0) > 1e-6) {
        return fail("resolve anchor 3d");
    }
    auto timeline_duration = client.helpers().call(
        "timeline.durationOf",
        "{\"timeline\":{\"clips\":["
        "{\"start\":0,\"channel\":{\"keys\":[{\"time\":0,\"value\":[0]},{\"time\":100,\"value\":[1]}]}},"
        "{\"start\":200,\"channel\":{\"keys\":[{\"time\":0,\"value\":[0]},{\"time\":100,\"value\":[1]}]}}]}}");
    if (std::abs(timeline_duration.number() - 300.0) > 1e-6) {
        return fail("timeline duration");
    }

    std::cout << "ok\n";
    return 0;
}
