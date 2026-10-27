// The diagnostics library facade: storyboard and mapset analysis.

package osbtk

// diagnosticsLib is the diagnostics library facade.
type diagnosticsLib struct{}

// Diagnostics is the diagnostics library.
var Diagnostics = diagnosticsLib{}

// DiagnoseOsb diagnoses an `.osb` storyboard.
func (diagnosticsLib) DiagnoseOsb(text string) (*DiagnosticReport, error) {
	return decodeBridge[*DiagnosticReport](map[string]any{"op": "Diagnostics.diagnoseOsb", "text": text})
}

// DiagnoseOsu diagnoses an `.osu` beatmap.
func (diagnosticsLib) DiagnoseOsu(text string) (*DiagnosticReport, error) {
	return decodeBridge[*DiagnosticReport](map[string]any{"op": "Diagnostics.diagnoseOsu", "text": text})
}
