using tochiRes.osbTK.Api;
using tochiRes.osbTK.Model;

using ImageApi = tochiRes.osbTK.Api.ImageAPI;

namespace tochiRes.osbTK;

public static partial class osb
{
    /// <summary>The image library: decode/encode and processing.</summary>
    public static class Image
    {
        private static readonly ImageApi Api = new(
            StatelessTransport,
            StatelessTransport,
            DefaultConfiguration
        );

        /// <summary>Reads image metadata.</summary>
        public static ImageInfo Info(string path) => Api.Info(new PathRequest(path: path));

        /// <summary>Processes an image (read, apply options, write).</summary>
        public static ImageInfo Process(string path, string output, ImageOptions options) =>
            Api.Process(
                new ImageProcessRequest(path: path, output: output, options: options)
            );
    }
}
