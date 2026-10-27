using tochiRes.osbTK.Api;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK;

public static partial class osb
{
    /// <summary>The image-generation library: draw primitive shapes (M25).</summary>
    public static class ImageGen
    {
        private static readonly ImagegenAPI Api = new(
            StatelessTransport,
            StatelessTransport,
            DefaultConfiguration
        );

        /// <summary>Renders primitive shapes to a new image.</summary>
        public static ImageInfo RenderShapes(RenderShapesRequest request) => Api.RenderShapes(request);
    }
}
