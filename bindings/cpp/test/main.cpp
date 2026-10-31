// Golden-vector parity for the C++ binding (header-only wrapper over the cdylib).
#include <filesystem>
#include <iostream>
#include <string>

#include "osbtk.hpp"

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

static int json_int(const std::string& json, const std::string& key) {
    std::string needle = "\"" + key + "\":";
    std::size_t position = json.find(needle);
    if (position == std::string::npos) {
        return -1;
    }
    return std::stoi(json.substr(position + needle.size()));
}

int main() {
    std::string path = find_library();
    if (path.empty()) {
        return fail("osb_native library not found");
    }

    osbtk::Library lib(path);
    if (lib.version().empty()) {
        return fail("empty version");
    }

    std::string baked = lib.bake3d(
        R"({"nodes":[{"sprite":"sb/dot.png"}],"camera":{"position":[{"time":0,"value":[0,0,5]}]},"options":{"endTime":0}})");
    if (baked.find("\"spritePath\":\"sb/dot.png\"") == std::string::npos) {
        return fail("bake3d: " + baked);
    }

    std::string beatmap = lib.exec(
        R"({"op":"Core.parseOsu","text":"osu file format v14\n\n[General]\nAudioFilename: song.mp3\nMode: 0\n[TimingPoints]\n0,500,4,2,1,100,1,0\n[HitObjects]\n256,192,1000,1,0\n"})");
    if (beatmap.find("\"audioFilename\":\"song.mp3\"") == std::string::npos) {
        return fail("parseOsu: " + beatmap);
    }

    osbtk::Runtime runtime(lib);
    std::string created = runtime.exec(R"({"op":"Layer.createSprite","layer":"Foreground","path":"sb/dot.png"})");
    int layer = json_int(created, "layer");
    int sprite = json_int(created, "sprite");
    runtime.exec(
        "{\"op\":\"Sprite.command\",\"kind\":\"move\",\"layer\":" + std::to_string(layer)
        + ",\"sprite\":" + std::to_string(sprite)
        + ",\"startTime\":0,\"endTime\":1000,\"start\":{\"x\":0,\"y\":0},\"end\":{\"x\":100,\"y\":0}}");
    std::string state = runtime.exec(
        "{\"op\":\"Sprite.stateAt\",\"layer\":" + std::to_string(layer)
        + ",\"sprite\":" + std::to_string(sprite) + ",\"time\":500}");
    if (state.find("\"x\":50") == std::string::npos) {
        return fail("runtime state: " + state);
    }

    std::cout << "ok\n";
    return 0;
}
