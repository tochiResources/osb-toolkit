package moe.tochi.osbtk.bridge;

import java.lang.foreign.Arena;
import java.lang.foreign.FunctionDescriptor;
import java.lang.foreign.Linker;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.SymbolLookup;
import java.lang.foreign.ValueLayout;
import java.lang.invoke.MethodHandle;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;

/**
 * The native bridge: loads the {@code osb-native} cdylib and calls the C ABI
 * ({@code osbtk_exec} / {@code osbtk_runtime_exec}) with the Foreign Function &amp;
 * Memory API, so no JNI glue is required.
 */
public final class NativeBridge {
    private static final Linker LINKER = Linker.nativeLinker();
    private static final SymbolLookup LOOKUP;
    private static final MethodHandle VERSION;
    private static final MethodHandle EXEC;
    private static final MethodHandle STRING_FREE;
    private static final MethodHandle RUNTIME_NEW;
    private static final MethodHandle RUNTIME_FREE;
    private static final MethodHandle RUNTIME_EXEC;

    static {
        System.load(findLibrary());
        LOOKUP = SymbolLookup.loaderLookup();
        VERSION = downcall("osbtk_version", FunctionDescriptor.of(ValueLayout.ADDRESS));
        EXEC = downcall("osbtk_exec", FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS));
        STRING_FREE = downcall("osbtk_string_free", FunctionDescriptor.ofVoid(ValueLayout.ADDRESS));
        RUNTIME_NEW =
                downcall(
                        "osbtk_runtime_new",
                        FunctionDescriptor.of(
                                ValueLayout.ADDRESS, ValueLayout.JAVA_DOUBLE, ValueLayout.JAVA_DOUBLE));
        RUNTIME_FREE = downcall("osbtk_runtime_free", FunctionDescriptor.ofVoid(ValueLayout.ADDRESS));
        RUNTIME_EXEC =
                downcall(
                        "osbtk_runtime_exec",
                        FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS));
    }

    private NativeBridge() {}

    /** The runtime version string. */
    public static String version() {
        try {
            return take((MemorySegment) VERSION.invoke());
        } catch (Throwable error) {
            throw new IllegalStateException(error);
        }
    }

    /** Runs a stateless JSON request against the bridge. */
    public static String exec(String request) {
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment argument = arena.allocateFrom(request);
            return take((MemorySegment) EXEC.invoke(argument));
        } catch (Throwable error) {
            throw new IllegalStateException(error);
        }
    }

    /** Creates a runtime handle (storyboard + context). */
    public static MemorySegment runtimeNew(double width, double height) {
        try {
            return (MemorySegment) RUNTIME_NEW.invoke(width, height);
        } catch (Throwable error) {
            throw new IllegalStateException(error);
        }
    }

    /** Releases a runtime handle. */
    public static void runtimeFree(MemorySegment handle) {
        try {
            RUNTIME_FREE.invoke(handle);
        } catch (Throwable error) {
            throw new IllegalStateException(error);
        }
    }

    /** Runs a JSON request against a runtime handle. */
    public static String runtimeExec(MemorySegment handle, String request) {
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment argument = arena.allocateFrom(request);
            return take((MemorySegment) RUNTIME_EXEC.invoke(handle, argument));
        } catch (Throwable error) {
            throw new IllegalStateException(error);
        }
    }

    private static MethodHandle downcall(String name, FunctionDescriptor descriptor) {
        MemorySegment address =
                LOOKUP.find(name).orElseThrow(() -> new IllegalStateException("osbtk: missing symbol " + name));
        return LINKER.downcallHandle(address, descriptor);
    }

    private static String take(MemorySegment result) throws Throwable {
        if (result.equals(MemorySegment.NULL)) {
            return "";
        }
        String text = result.reinterpret(Long.MAX_VALUE).getString(0);
        STRING_FREE.invoke(result);
        return text;
    }

    private static String findLibrary() {
        String override = System.getenv("OSBTK_NATIVE_LIB");
        if (override != null && Files.isRegularFile(Path.of(override))) {
            return override;
        }
        List<String> names =
                System.getProperty("os.name", "").toLowerCase().contains("win")
                        ? List.of("osb_native.dll")
                        : List.of("libosb_native.dylib", "libosb_native.so");
        Path directory = Path.of("").toAbsolutePath();
        for (int depth = 0; depth < 8 && directory != null; depth++) {
            for (String name : names) {
                for (String profile : new String[] {"release", "debug"}) {
                    Path candidate = directory.resolve("target").resolve(profile).resolve(name);
                    if (Files.isRegularFile(candidate)) {
                        return candidate.toString();
                    }
                }
            }
            directory = directory.getParent();
        }
        throw new IllegalStateException(
                "osbtk: osb_native shared library not found; build with `cargo build -p osb-native`");
    }
}
