package moe.tochi.osbtk.bridge;

import com.google.gson.JsonElement;
import com.google.gson.JsonObject;
import com.google.gson.JsonParser;
import java.io.IOException;
import okhttp3.Interceptor;
import okhttp3.MediaType;
import okhttp3.Protocol;
import okhttp3.Request;
import okhttp3.Response;
import okhttp3.ResponseBody;
import okio.Buffer;

/**
 * An OkHttp interceptor that routes the generated client's requests to the native
 * JSON bridge instead of the network. The operation is the last path segment
 * (which is exactly the native {@code op}); the request body is merged with it.
 * Bridge errors come back as a 400, so the client raises {@code ApiException}.
 *
 * <p>The body is handled as a {@link JsonObject} (not a map) so integer fields
 * stay integers on the wire.
 */
public final class BridgeInterceptor implements Interceptor {
    /** Runs a JSON request against the native bridge. */
    public interface Bridge {
        String exec(String request);
    }

    private static final MediaType JSON = MediaType.get("application/json");
    private final Bridge bridge;

    public BridgeInterceptor(Bridge bridge) {
        this.bridge = bridge;
    }

    @Override
    public Response intercept(Chain chain) throws IOException {
        Request request = chain.request();
        String path = request.url().encodedPath();
        String op = path.substring(path.lastIndexOf('/') + 1);

        JsonObject body = new JsonObject();
        if (request.body() != null) {
            Buffer buffer = new Buffer();
            request.body().writeTo(buffer);
            String json = buffer.readUtf8();
            if (!json.isBlank()) {
                JsonElement parsed = JsonParser.parseString(json);
                if (parsed.isJsonObject()) {
                    body = parsed.getAsJsonObject();
                }
            }
        }
        body.addProperty("op", op);

        String responseJson = bridge.exec(body.toString());
        boolean error = isError(responseJson);

        return new Response.Builder()
                .request(request)
                .protocol(Protocol.HTTP_1_1)
                .code(error ? 400 : 200)
                .message(error ? "Bad Request" : "OK")
                .body(ResponseBody.create(responseJson, JSON))
                .build();
    }

    private boolean isError(String responseJson) {
        try {
            JsonElement parsed = JsonParser.parseString(responseJson);
            return parsed.isJsonObject() && parsed.getAsJsonObject().has("error");
        } catch (RuntimeException ignored) {
            return false;
        }
    }
}
