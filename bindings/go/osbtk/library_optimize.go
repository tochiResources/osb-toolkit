// The optimization library facade: storyboard optimization and `.osb` round-trip.

package osbtk

// optimizeLib is the optimization library facade.
type optimizeLib struct{}

// Optimize is the optimization library.
var Optimize = optimizeLib{}

// OptimizeOsb optimizes an `.osb` storyboard, returning the optimized text and stats.
func (optimizeLib) OptimizeOsb(text string, options OptimizeOptionsInput) (*OptimizeResult, error) {
	return decodeBridge[*OptimizeResult](map[string]any{
		"op": "Optimize.optimizeOsb", "text": text, "options": options,
	})
}
