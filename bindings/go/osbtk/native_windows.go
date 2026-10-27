//go:build windows

package osbtk

import "syscall"

// openLibrary loads a shared library and returns its module handle, which
// `purego.RegisterLibFunc` resolves symbols against on Windows.
func openLibrary(path string) (uintptr, error) {
	handle, err := syscall.LoadLibrary(path)
	if err != nil {
		return 0, err
	}
	return uintptr(handle), nil
}
