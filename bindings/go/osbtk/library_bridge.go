// Shared helpers for the hand-written Go library facades. Every
// facade drives the native JSON bridge directly, so the typed models and the
// generated clients stay an escape hatch.

package osbtk

import (
	"encoding/json"
	"fmt"
)

// callBridge runs a stateless JSON request against the native bridge and returns
// the raw response bytes, mapping the error envelope to an error.
func callBridge(request map[string]any) ([]byte, error) {
	library, err := loadNative()
	if err != nil {
		return nil, err
	}
	encoded, err := json.Marshal(request)
	if err != nil {
		return nil, err
	}
	response, err := library.callExec(string(encoded))
	if err != nil {
		return nil, err
	}
	var probe map[string]any
	if json.Unmarshal([]byte(response), &probe) == nil {
		if message, ok := probe["error"].(string); ok {
			return nil, fmt.Errorf("%s", message)
		}
	}
	return []byte(response), nil
}

// decodeBridge runs a request and decodes the response into T.
func decodeBridge[T any](request map[string]any) (T, error) {
	var out T
	raw, err := callBridge(request)
	if err != nil {
		return out, err
	}
	if err := json.Unmarshal(raw, &out); err != nil {
		return out, err
	}
	return out, nil
}

// helperValue runs a motion-graphics helper op and returns its `value` field.
func helperValue(op, name string, input map[string]any) (any, error) {
	if input == nil {
		input = map[string]any{}
	}
	result, err := decodeBridge[HelperResult](map[string]any{
		"op":    op,
		"name":  name,
		"input": input,
	})
	if err != nil {
		return nil, err
	}
	return result.Value, nil
}

// remarshal converts an arbitrary bridge value into T via JSON.
func remarshal[T any](value any) (T, error) {
	var out T
	raw, err := json.Marshal(value)
	if err != nil {
		return out, err
	}
	if err := json.Unmarshal(raw, &out); err != nil {
		return out, err
	}
	return out, nil
}
