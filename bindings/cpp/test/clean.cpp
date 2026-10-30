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

    osbtk::sdk::Json baked = clean.storyboarding3d.bake3d(
        {"{\"nodes\":[{\"sprite\":\"sb/dot.png\"}],\"camera\":{\"position\":[{\"time\":0,\"value\":[0,0,5]}]}}"});
    if (baked.find("sprites") == nullptr || baked.at("sprites").array().size() != 1) {
        return fail("clean bake3d");
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
