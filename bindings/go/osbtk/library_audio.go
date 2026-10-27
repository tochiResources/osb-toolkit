// The audio library facade: FFT.

package osbtk

// audioLib is the audio library facade.
type audioLib struct{}

// Audio is the audio library.
var Audio = audioLib{}

// Fft reads FFT magnitudes from a `.wav` file.
func (audioLib) Fft(path string, time float64, magnitudes int) (*FftResult, error) {
	return decodeBridge[*FftResult](map[string]any{
		"op": "Audio.fft", "path": path, "time": time, "magnitudes": magnitudes,
	})
}
