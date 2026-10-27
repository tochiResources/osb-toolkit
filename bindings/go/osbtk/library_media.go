// The media library facade: probing, frame extraction/encoding and audio.

package osbtk

// mediaLib is the media library facade.
type mediaLib struct{}

// Media is the media library.
var Media = mediaLib{}

// Info probes a media file for its duration, dimensions and codecs.
func (mediaLib) Info(path string) (*MediaInfo, error) {
	return decodeBridge[*MediaInfo](map[string]any{"op": "Media.info", "path": path})
}

// ExtractFrames extracts frames from a video into `output_dir`.
func (mediaLib) ExtractFrames(request ExtractFramesRequest) (*FrameResult, error) {
	payload, err := remarshal[map[string]any](request)
	if err != nil {
		return nil, err
	}
	payload["op"] = "Media.extractFrames"
	return decodeBridge[*FrameResult](payload)
}

// EncodeVideo encodes an image sequence into a video.
func (mediaLib) EncodeVideo(request EncodeVideoRequest) (*MediaInfo, error) {
	payload, err := remarshal[map[string]any](request)
	if err != nil {
		return nil, err
	}
	payload["op"] = "Media.encodeVideo"
	return decodeBridge[*MediaInfo](payload)
}
