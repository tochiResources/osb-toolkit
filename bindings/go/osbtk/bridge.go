package osbtk

import (
	"bytes"
	"encoding/json"
	"io"
	"net/http"
	"path"
	"strings"
)

// bridgeExecutor runs a JSON request against the native bridge and returns the
// JSON response.
type bridgeExecutor func(request string) (string, error)

// bridgeTransport is an http.RoundTripper that routes the generated client's
// requests to the native JSON bridge instead of the network. The operation is
// the last path segment (which is exactly the native `op`); the request body is
// merged with it. Bridge errors come back as a 400 so the client raises them.
type bridgeTransport struct {
	exec bridgeExecutor
}

func (transport bridgeTransport) RoundTrip(request *http.Request) (*http.Response, error) {
	body := map[string]any{}
	if request.Body != nil {
		data, err := io.ReadAll(request.Body)
		if err != nil {
			return nil, err
		}
		if len(bytes.TrimSpace(data)) > 0 {
			if err := json.Unmarshal(data, &body); err != nil {
				return nil, err
			}
		}
	}
	body["op"] = path.Base(request.URL.Path)
	encoded, err := json.Marshal(body)
	if err != nil {
		return nil, err
	}

	responseText, err := transport.exec(string(encoded))
	if err != nil {
		return nil, err
	}

	status := http.StatusOK
	var probe map[string]any
	if json.Unmarshal([]byte(responseText), &probe) == nil {
		if _, isError := probe["error"]; isError {
			status = http.StatusBadRequest
		}
	}

	return &http.Response{
		StatusCode: status,
		Status:     http.StatusText(status),
		Header:     http.Header{"Content-Type": []string{"application/json"}},
		Body:       io.NopCloser(strings.NewReader(responseText)),
		Request:    request,
	}, nil
}
