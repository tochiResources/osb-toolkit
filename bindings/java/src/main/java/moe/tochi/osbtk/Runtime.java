package moe.tochi.osbtk;

import java.lang.foreign.MemorySegment;
import java.lang.ref.Cleaner;
import moe.tochi.osbtk.api.AudioAPI;
import moe.tochi.osbtk.api.GeneratorAPI;
import moe.tochi.osbtk.api.LayerAPI;
import moe.tochi.osbtk.api.SpriteAPI;
import moe.tochi.osbtk.bridge.BridgeInterceptor;
import moe.tochi.osbtk.bridge.NativeBridge;
import okhttp3.OkHttpClient;

/**
 * A live storyboard runtime bound to a native handle. Its clients are driven
 * through the bridge transport; {@link #close()} releases the native runtime (a
 * {@link Cleaner} frees it as a fallback if you forget).
 */
public final class Runtime implements AutoCloseable {
    private static final Cleaner CLEANER = Cleaner.create();

    private final Cleaner.Cleanable cleanable;
    private final GeneratorAPI generator;
    private final LayerAPI layer;
    private final SpriteAPI sprite;
    private final AudioAPI audio;

    public Runtime() {
        this(640, 480);
    }

    public Runtime(double width, double height) {
        MemorySegment handle = NativeBridge.runtimeNew(width, height);
        this.cleanable = CLEANER.register(this, new NativeState(handle));

        OkHttpClient httpClient =
                new OkHttpClient.Builder()
                        .addInterceptor(
                                new BridgeInterceptor(request -> NativeBridge.runtimeExec(handle, request)))
                        .build();
        ApiClient apiClient = new ApiClient(httpClient);
        apiClient.setBasePath("http://osbtk.invalid");
        this.generator = new GeneratorAPI(apiClient);
        this.layer = new LayerAPI(apiClient);
        this.sprite = new SpriteAPI(apiClient);
        this.audio = new AudioAPI(apiClient);
    }

    /** The generated generator client bound to this runtime. */
    public GeneratorAPI generator() {
        return generator;
    }

    /** The generated layer client bound to this runtime. */
    public LayerAPI layer() {
        return layer;
    }

    /** The generated sprite client bound to this runtime. */
    public SpriteAPI sprite() {
        return sprite;
    }

    /** The generated audio client bound to this runtime. */
    public AudioAPI audio() {
        return audio;
    }

    @Override
    public void close() {
        cleanable.clean();
    }

    private static final class NativeState implements Runnable {
        private MemorySegment handle;

        NativeState(MemorySegment handle) {
            this.handle = handle;
        }

        @Override
        public void run() {
            if (handle != null) {
                NativeBridge.runtimeFree(handle);
                handle = null;
            }
        }
    }
}
