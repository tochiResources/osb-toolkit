using Newtonsoft.Json.Linq;

using tochiRes.osbTK.Model;

namespace tochiRes.osbTK;

public static partial class osb
{
    public static partial class Helpers
    {
        /// <summary>Amplifies an easing about linear by <paramref name="amount"/> (&gt;1 stronger).</summary>
        public static double AmplifyEasing(Easing easing, double t, double amount) =>
            Call("easing.amplifyEasing", new JObject
            {
                ["easing"] = JToken.FromObject(easing),
                ["t"] = t,
                ["amount"] = amount,
            }).Value<double>();

        /// <summary>Samples an easing at <paramref name="count"/> points in [0, 1].</summary>
        public static double[] SampleEasing(Easing easing, int count) =>
            Call("easing.sampleEasing", new JObject
            {
                ["easing"] = JToken.FromObject(easing),
                ["count"] = count,
            }).ToObject<double[]>()!;
    }
}
