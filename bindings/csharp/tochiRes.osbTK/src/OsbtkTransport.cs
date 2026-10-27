using System.Net;
using System.Threading;
using System.Threading.Tasks;

using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

using tochiRes.osbTK.Client;

using NativeFacade = global::tochiRes.osbTK.Native.osb;
using NativeRuntime = global::tochiRes.osbTK.Native.Runtime;

namespace tochiRes.osbTK;

/// <summary>
/// The bridge transport: implements the generated client's HTTP-client interfaces
/// by routing every call to the native JSON bridge - <c>osbtk_exec</c>, or
/// <c>osbtk_runtime_exec</c> when a runtime handle is bound - instead of the
/// network. The operation name is the last path segment, which is exactly the
/// native <c>op</c>.
/// </summary>
internal sealed class OsbtkTransport : ISynchronousClient, IAsynchronousClient
{
    private readonly NativeRuntime? _runtime;

    internal OsbtkTransport(NativeRuntime? runtime = null)
    {
        _runtime = runtime;
    }

    public ApiResponse<T> Post<T>(
        string path,
        RequestOptions options,
        IReadableConfiguration configuration = null!
    ) => Execute<T>(path, options);

    public Task<ApiResponse<T>> PostAsync<T>(
        string path,
        RequestOptions options,
        IReadableConfiguration configuration = null!,
        CancellationToken cancellationToken = default
    ) => Task.FromResult(Execute<T>(path, options));

    public ApiResponse<T> Get<T>(
        string path,
        RequestOptions options,
        IReadableConfiguration configuration = null!
    ) => throw Unsupported("GET");

    public Task<ApiResponse<T>> GetAsync<T>(
        string path,
        RequestOptions options,
        IReadableConfiguration configuration = null!,
        CancellationToken cancellationToken = default
    ) => throw Unsupported("GET");

    public ApiResponse<T> Put<T>(
        string path,
        RequestOptions options,
        IReadableConfiguration configuration = null!
    ) => throw Unsupported("PUT");

    public Task<ApiResponse<T>> PutAsync<T>(
        string path,
        RequestOptions options,
        IReadableConfiguration configuration = null!,
        CancellationToken cancellationToken = default
    ) => throw Unsupported("PUT");

    public ApiResponse<T> Delete<T>(
        string path,
        RequestOptions options,
        IReadableConfiguration configuration = null!
    ) => throw Unsupported("DELETE");

    public Task<ApiResponse<T>> DeleteAsync<T>(
        string path,
        RequestOptions options,
        IReadableConfiguration configuration = null!,
        CancellationToken cancellationToken = default
    ) => throw Unsupported("DELETE");

    public ApiResponse<T> Head<T>(
        string path,
        RequestOptions options,
        IReadableConfiguration configuration = null!
    ) => throw Unsupported("HEAD");

    public Task<ApiResponse<T>> HeadAsync<T>(
        string path,
        RequestOptions options,
        IReadableConfiguration configuration = null!,
        CancellationToken cancellationToken = default
    ) => throw Unsupported("HEAD");

    public ApiResponse<T> Options<T>(
        string path,
        RequestOptions options,
        IReadableConfiguration configuration = null!
    ) => throw Unsupported("OPTIONS");

    public Task<ApiResponse<T>> OptionsAsync<T>(
        string path,
        RequestOptions options,
        IReadableConfiguration configuration = null!,
        CancellationToken cancellationToken = default
    ) => throw Unsupported("OPTIONS");

    public ApiResponse<T> Patch<T>(
        string path,
        RequestOptions options,
        IReadableConfiguration configuration = null!
    ) => throw Unsupported("PATCH");

    public Task<ApiResponse<T>> PatchAsync<T>(
        string path,
        RequestOptions options,
        IReadableConfiguration configuration = null!,
        CancellationToken cancellationToken = default
    ) => throw Unsupported("PATCH");

    private static NotSupportedException Unsupported(string method) =>
        new($"the osb bridge only supports POST (got {method})");

    private ApiResponse<T> Execute<T>(string path, RequestOptions options)
    {
        string op = path.TrimEnd('/').Split('/')[^1];
        JObject request = options?.Data is null ? new JObject() : JObject.FromObject(options.Data);
        request["op"] = op;
        string requestJson = request.ToString(Formatting.None);

        string responseJson =
            _runtime is null ? NativeFacade.Exec(requestJson) : _runtime.Exec(requestJson);

        JToken token = JToken.Parse(responseJson);
        if (token is JObject error && error.TryGetValue("error", out JToken? message))
        {
            throw new OsbTkException(message.Value<string>() ?? "unknown native error");
        }

        T data = token.Type == JTokenType.Null ? default! : token.ToObject<T>()!;
        return new ApiResponse<T>(HttpStatusCode.OK, new Multimap<string, string>(), data, responseJson);
    }
}
