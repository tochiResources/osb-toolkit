// Package osbtk is the typed Go SDK over the osbTK native engine. The models and services in this package are generated from the
// OpenAPI contract in `crates/osb-api`; this file wraps the native C ABI
// (`osbtk_exec` / `osbtk_runtime_exec`) with `purego`, so no cgo is required.
package osbtk

import (
	"fmt"
	"os"
	"path/filepath"
	"runtime"
	"sync"
	"unsafe"

	"github.com/ebitengine/purego"
)

type nativeLibrary struct {
	version     func() *byte
	exec        func(*byte) *byte
	stringFree  func(*byte)
	runtimeNew  func(float64, float64) uintptr
	runtimeFree func(uintptr)
	runtimeExec func(uintptr, *byte) *byte
}

var (
	nativeOnce sync.Once
	nativeLib  *nativeLibrary
	nativeErr  error
)

func libraryNames() []string {
	switch runtime.GOOS {
	case "windows":
		return []string{"osb_native.dll"}
	case "darwin":
		return []string{"libosb_native.dylib"}
	default:
		return []string{"libosb_native.so"}
	}
}

// findLibrary locates the cdylib from OSBTK_NATIVE_LIB or by walking up to the
// cargo target/{release,debug} directory, so no install step is required.
func findLibrary() (string, error) {
	if override := os.Getenv("OSBTK_NATIVE_LIB"); override != "" {
		if _, err := os.Stat(override); err == nil {
			return override, nil
		}
	}
	directory, err := os.Getwd()
	if err != nil {
		return "", err
	}
	for depth := 0; depth < 8; depth++ {
		for _, name := range libraryNames() {
			for _, profile := range []string{"release", "debug"} {
				candidate := filepath.Join(directory, "target", profile, name)
				if _, err := os.Stat(candidate); err == nil {
					return candidate, nil
				}
			}
		}
		parent := filepath.Dir(directory)
		if parent == directory {
			break
		}
		directory = parent
	}
	return "", fmt.Errorf("osbtk: osb_native shared library not found; build with `cargo build -p osb-native`")
}

func loadNative() (*nativeLibrary, error) {
	nativeOnce.Do(func() {
		path, err := findLibrary()
		if err != nil {
			nativeErr = err
			return
		}
		handle, err := openLibrary(path)
		if err != nil {
			nativeErr = fmt.Errorf("osbtk: %w", err)
			return
		}
		library := &nativeLibrary{}
		purego.RegisterLibFunc(&library.version, handle, "osbtk_version")
		purego.RegisterLibFunc(&library.exec, handle, "osbtk_exec")
		purego.RegisterLibFunc(&library.stringFree, handle, "osbtk_string_free")
		purego.RegisterLibFunc(&library.runtimeNew, handle, "osbtk_runtime_new")
		purego.RegisterLibFunc(&library.runtimeFree, handle, "osbtk_runtime_free")
		purego.RegisterLibFunc(&library.runtimeExec, handle, "osbtk_runtime_exec")
		nativeLib = library
	})
	return nativeLib, nativeErr
}

func cString(value string) []byte {
	buffer := make([]byte, len(value)+1)
	copy(buffer, value)
	return buffer
}

func goString(pointer *byte) string {
	if pointer == nil {
		return ""
	}
	length := 0
	for ptr := unsafe.Pointer(pointer); *(*byte)(unsafe.Add(ptr, length)) != 0; length++ {
	}
	return string(unsafe.Slice(pointer, length))
}

// callExec runs a stateless JSON request against the native bridge.
func (library *nativeLibrary) callExec(request string) (string, error) {
	buffer := cString(request)
	result := library.exec(&buffer[0])
	runtime.KeepAlive(buffer)
	if result == nil {
		return "", fmt.Errorf("osbtk: null response")
	}
	text := goString(result)
	library.stringFree(result)
	return text, nil
}

// Version returns the native runtime version string.
func Version() (string, error) {
	library, err := loadNative()
	if err != nil {
		return "", err
	}
	result := library.version()
	if result == nil {
		return "", nil
	}
	text := goString(result)
	library.stringFree(result)
	return text, nil
}
