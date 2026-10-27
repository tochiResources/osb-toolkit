#pragma once
// Header-only RAII wrapper over the osbTK C ABI (crates/osb-native, section
// 5.13). The shared library is loaded at runtime, so consumers need no import
// library or link step - just compile with the header and point `Library` at the
// `osb_native` shared object (or set `OSBTK_NATIVE_LIB`).

#include <stdexcept>
#include <string>

#if defined(_WIN32)
#include <windows.h>
#else
#include <dlfcn.h>
#endif

namespace osbtk {

class Library {
public:
    explicit Library(const std::string& path) {
#if defined(_WIN32)
        handle_ = ::LoadLibraryA(path.c_str());
#else
        handle_ = ::dlopen(path.c_str(), RTLD_NOW);
#endif
        if (handle_ == nullptr) {
            throw std::runtime_error("osbtk: cannot load " + path);
        }
        version_ = reinterpret_cast<VersionFn>(symbol("osbtk_version"));
        bake3d_ = reinterpret_cast<CharFn>(symbol("osbtk_bake3d"));
        exec_ = reinterpret_cast<CharFn>(symbol("osbtk_exec"));
        string_free_ = reinterpret_cast<FreeFn>(symbol("osbtk_string_free"));
        runtime_new_ = reinterpret_cast<RuntimeNewFn>(symbol("osbtk_runtime_new"));
        runtime_free_ = reinterpret_cast<RuntimeFreeFn>(symbol("osbtk_runtime_free"));
        runtime_exec_ = reinterpret_cast<RuntimeExecFn>(symbol("osbtk_runtime_exec"));
    }

    ~Library() { close(); }
    Library(const Library&) = delete;
    Library& operator=(const Library&) = delete;

    /// The runtime version string.
    std::string version() const { return take(version_()); }

    /// Bakes a plain 3D input JSON into baked command-span JSON.
    std::string bake3d(const std::string& input) const { return take(bake3d_(input.c_str())); }

    /// Runs a JSON request (`{"op": "..."}`) against the runtime.
    std::string exec(const std::string& request) const { return take(exec_(request.c_str())); }

    void* runtime_new(double width, double height) const { return runtime_new_(width, height); }
    void runtime_free(void* handle) const { runtime_free_(handle); }
    std::string runtime_exec(void* handle, const std::string& request) const {
        return take(runtime_exec_(handle, request.c_str()));
    }

private:
    using VersionFn = char* (*)();
    using CharFn = char* (*)(const char*);
    using FreeFn = void (*)(char*);
    using RuntimeNewFn = void* (*)(double, double);
    using RuntimeFreeFn = void (*)(void*);
    using RuntimeExecFn = char* (*)(void*, const char*);

    void* handle_ = nullptr;
    VersionFn version_ = nullptr;
    CharFn bake3d_ = nullptr;
    CharFn exec_ = nullptr;
    FreeFn string_free_ = nullptr;
    RuntimeNewFn runtime_new_ = nullptr;
    RuntimeFreeFn runtime_free_ = nullptr;
    RuntimeExecFn runtime_exec_ = nullptr;

    void* symbol(const char* name) const {
#if defined(_WIN32)
        void* address = reinterpret_cast<void*>(::GetProcAddress(static_cast<HMODULE>(handle_), name));
#else
        void* address = ::dlsym(handle_, name);
#endif
        if (address == nullptr) {
            throw std::runtime_error(std::string("osbtk: missing symbol ") + name);
        }
        return address;
    }

    void close() {
        if (handle_ == nullptr) {
            return;
        }
#if defined(_WIN32)
        ::FreeLibrary(static_cast<HMODULE>(handle_));
#else
        ::dlclose(handle_);
#endif
        handle_ = nullptr;
    }

    std::string take(char* value) const {
        if (value == nullptr) {
            return {};
        }
        std::string result(value);
        string_free_(value);
        return result;
    }
};

/// An RAII handle to a live runtime (storyboard + context). Drive the
/// layer/sprite/command surface with JSON requests through `exec`.
class Runtime {
public:
    Runtime(const Library& library, double width = 640.0, double height = 480.0)
        : library_(&library), handle_(library.runtime_new(width, height)) {}

    ~Runtime() {
        if (handle_ != nullptr) {
            library_->runtime_free(handle_);
        }
    }

    Runtime(const Runtime&) = delete;
    Runtime& operator=(const Runtime&) = delete;

    std::string exec(const std::string& request) const { return library_->runtime_exec(handle_, request); }

private:
    const Library* library_;
    void* handle_;
};

}  // namespace osbtk
