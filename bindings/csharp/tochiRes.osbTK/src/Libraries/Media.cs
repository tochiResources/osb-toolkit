using tochiRes.osbTK.Api;
using tochiRes.osbTK.Model;

using MediaApi = tochiRes.osbTK.Api.MediaAPI;

namespace tochiRes.osbTK;

public static partial class osb
{
    /// <summary>The media library: probing, frame extraction/encoding and audio.</summary>
    public static class Media
    {
        private static readonly MediaApi Api = new(
            StatelessTransport,
            StatelessTransport,
            DefaultConfiguration
        );

        /// <summary>Probes a media file.</summary>
        public static MediaInfo Info(string path) => Api.Info(new PathRequest(path: path));

        /// <summary>Extracts frames from a video into a directory.</summary>
        public static FrameResult ExtractFrames(ExtractFramesRequest request) =>
            Api.ExtractFrames(request);

        /// <summary>Encodes an image sequence into a video.</summary>
        public static MediaInfo EncodeVideo(EncodeVideoRequest request) => Api.EncodeVideo(request);

        /// <summary>Extracts the audio track of a media file.</summary>
        public static OkResult ExtractAudio(ExtractAudioRequest request) =>
            Api.ExtractAudio(request);
    }
}
