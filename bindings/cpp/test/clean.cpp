// The clean C++ clients over the native bridge (osbtk_clean.hpp).
#include <cmath>
#include <filesystem>
#include <iostream>
#include <string>

#include "osbtk_clean.hpp"

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
    osbtk::clean::Client clean(lib);

    if (clean.core.version().empty()) {
        return fail("clean version");
    }
    if (std::abs(clean.core.apply_easing({"InQuad", 0.5}) - 0.25) > 1e-6) {
        return fail("clean easing");
    }
    if (std::abs(clean.core.easing_value({1.0, "None", {0.42, 0.0, 0.58, 1.0}}) - 1.0) >
        1e-6) {
        return fail("clean cubic bezier");
    }

    osbtk::sdk::CurveSpec curve;
    curve.kind = osbtk::sdk::CurveKind::Linear;
    curve.points = {{0.0, 0.0}, {100.0, 0.0}};
    if (std::abs(clean.core.length({curve}) - 100.0) > 1e-6) {
        return fail("clean curve length");
    }
    if (std::abs(clean.core.point({curve, 0.5, false}).x - 50.0) > 1e-6) {
        return fail("clean curve point");
    }
    if (clean.core.sample({3, curve, false}).points.size() != 3) {
        return fail("clean curve sample");
    }
    if (clean.core.arc({0.0, 0.0, 10.0, 0.0, 180.0, 3}).points.size() != 3) {
        return fail("clean curve arc");
    }

    osbtk::sdk::Beatmap beatmap =
        clean.core.parse_osu({"osu file format v14\n\n[General]\nAudioFilename: song.mp3\nMode: 0\n"});
    if (beatmap.audio_filename != "song.mp3") {
        return fail("clean parse");
    }
    osbtk::sdk::Vector2 origin =
        clean.core.get_origin_vector({osbtk::sdk::OsbOrigin::Centre, 100.0, 100.0});
    if (std::abs(origin.x - 50.0) > 1e-6) {
        return fail("clean origin");
    }

    osbtk::clean::Runtime runtime(lib);
    osbtk::clean::Generator generator = runtime.generator();
    if (generator.sprite_count() != 0) {
        return fail("clean runtime initial sprite count");
    }
    osbtk::clean::Layer layer = runtime.layer("Foreground");
    osbtk::clean::Sprite sprite =
        layer.create_sprite({"sb/dot.png", osbtk::sdk::OsbOrigin::Centre});
    sprite.command({"move", 0.0, 1000.0, "None", "{\"x\":0,\"y\":0}", "{\"x\":100,\"y\":0}", ""});
    osbtk::sdk::SpriteState state = sprite.state_at({500.0});
    if (std::abs(state.position.x - 50.0) > 0.1) {
        return fail("clean runtime sprite position");
    }
    if (generator.sprite_count() != 1) {
        return fail("clean runtime sprite count");
    }

    osbtk::clean::Sprite animation = layer.create_animation(
        {"sb/anim.png", 2, 100.0, osbtk::sdk::OsbLoopType::LoopForever,
         osbtk::sdk::OsbOrigin::Centre, {0.0, 0.0}, ""});
    if (generator.sprite_count() != 2) {
        return fail("clean runtime animation count");
    }
    if (!animation.info().is_object()) {
        return fail("clean runtime sprite info");
    }

    osbtk::clean::Layer child = layer.create_segment();
    child.create_sprite({"sb/child.png", osbtk::sdk::OsbOrigin::Centre});
    if (generator.sprite_count() != 3) {
        return fail("clean runtime segment count");
    }
    if (!layer.discard_sprite(animation.sprite())) {
        return fail("clean runtime discard sprite");
    }

    osbtk::sdk::Json baked = clean.storyboarding3d.bake3d(
        {"{\"nodes\":[{\"sprite\":\"sb/dot.png\"}],\"camera\":{\"position\":[{\"time\":0,\"value\":[0,0,5]}]}}"});
    if (baked.find("sprites") == nullptr || baked.at("sprites").array().size() != 1) {
        return fail("clean bake3d");
    }

    osbtk::clean::RenderShapesOptions shapes;
    shapes.width = 32;
    shapes.height = 32;
    shapes.output = (fs::temp_directory_path() / "osbtk_clean_shapes.png").string();
    osbtk::clean::Shape rect;
    rect.kind = "Rect";
    rect.center = {16.0, 16.0};
    rect.size = {16.0, 16.0};
    osbtk::clean::RgbaColor fill;
    fill.r = 255;
    fill.a = 255;
    rect.fill = fill;
    shapes.shapes.push_back(rect);
    osbtk::sdk::Json rendered = clean.image_gen.render_shapes(shapes);
    if (rendered.at("width").number() != 32) {
        return fail("clean render shapes");
    }

    osbtk::clean::ImageProcessOptions process;
    process.path = shapes.output;
    process.output = (fs::temp_directory_path() / "osbtk_clean_processed.png").string();
    process.options.grayscale = true;
    clean.image.process(process);
    osbtk::sdk::Json processed = clean.image.info({process.output});
    if (processed.at("format").string() != "PNG") {
        return fail("clean image process");
    }

    osbtk::sdk::Json report = clean.diagnostics.diagnose_osu(
        {"osu file format v14\n\n[General]\nAudioFilename: song.mp3\nMode: 0\n"});
    if (report.find("errorCount") == nullptr || report.at("errorCount").number() < 1) {
        return fail("clean diagnostics");
    }

    osbtk::sdk::Json optimized = clean.optimize.optimize_osb(
        {"[Events]\nSprite,Foreground,Centre,\"sb/dot.png\",320,240\n F,0,0,100,0,1\n F,0,0,100,0,1\n",
         true});
    if (optimized.at("stats").at("deduplicated").number() != 1) {
        return fail("clean optimize");
    }

    osbtk::sdk::Json scaled = clean.helpers.call(
        {"keyframe.scaleKeyframes",
         "{\"channel\":{\"keys\":[{\"time\":0,\"value\":[1,2]},"
         "{\"time\":100,\"value\":[3,4]}]},\"factor\":2}"});
    if (scaled.at("keys").array()[1].at("value").array()[0].number() != 6) {
        return fail("clean helpers");
    }

    std::cout << "ok\n";
    return 0;
}
