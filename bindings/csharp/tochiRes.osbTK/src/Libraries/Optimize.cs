using tochiRes.osbTK.Api;
using tochiRes.osbTK.Model;

using OptimizeApi = tochiRes.osbTK.Api.OptimizeAPI;

namespace tochiRes.osbTK;

public static partial class osb
{
    /// <summary>The optimization library: storyboard optimization and `.osb` round-trip (M30).</summary>
    public static class Optimize
    {
        private static readonly OptimizeApi Api = new(
            StatelessTransport,
            StatelessTransport,
            DefaultConfiguration
        );

        /// <summary>Optimizes an `.osb` storyboard, returning the text and stats.</summary>
        public static OptimizeResult OptimizeOsb(string text, OptimizeOptionsInput options) =>
            Api.OptimizeOsb(new OptimizeRequest(text: text, options: options));
    }
}
