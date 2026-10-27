using System.Reflection;
using System.Runtime.InteropServices;

namespace tochiRes.osbTK.Native;

/// <summary>
/// P/Invoke declarations for the osb C ABI (<c>crates/osb-native</c>). The
/// library is resolved from <c>OSBTK_NATIVE_LIB</c> or by walking up to the
/// cargo <c>target/{release,debug}</c> directory, so tests and consumers do not
/// need the DLL copied next to the assembly.
/// </summary>
internal static class NativeMethods
{
    private const string Library = "osb_native";

    private static readonly string[] FileNames = ["osb_native.dll", "libosb_native.so", "libosb_native.dylib"];
    private static readonly string[] Profiles = ["release", "debug"];

    static NativeMethods()
    {
        NativeLibrary.SetDllImportResolver(typeof(NativeMethods).Assembly, Resolve);
    }

    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)]
    private static extern IntPtr osbtk_version();

    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)]
    private static extern IntPtr osbtk_bake3d(IntPtr inputJson);

    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)]
    private static extern IntPtr osbtk_exec(IntPtr requestJson);

    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)]
    private static extern void osbtk_string_free(IntPtr value);

    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)]
    private static extern IntPtr osbtk_runtime_new(double width, double height);

    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)]
    private static extern void osbtk_runtime_free(IntPtr runtime);

    [DllImport(Library, CallingConvention = CallingConvention.Cdecl)]
    private static extern IntPtr osbtk_runtime_exec(IntPtr runtime, IntPtr requestJson);

    public static string Version() => Take(osbtk_version());

    public static string Bake3d(string inputJson) => Call(osbtk_bake3d, inputJson);

    public static string Exec(string requestJson) => Call(osbtk_exec, requestJson);

    public static IntPtr RuntimeNew(double width, double height) => osbtk_runtime_new(width, height);

    public static void RuntimeFree(IntPtr runtime) => osbtk_runtime_free(runtime);

    public static string RuntimeExec(IntPtr runtime, string requestJson)
    {
        IntPtr input = Marshal.StringToCoTaskMemUTF8(requestJson);
        try
        {
            return Take(osbtk_runtime_exec(runtime, input));
        }
        finally
        {
            Marshal.FreeCoTaskMem(input);
        }
    }

    private static string Call(Func<IntPtr, IntPtr> function, string argument)
    {
        IntPtr input = Marshal.StringToCoTaskMemUTF8(argument);
        try
        {
            return Take(function(input));
        }
        finally
        {
            Marshal.FreeCoTaskMem(input);
        }
    }

    private static string Take(IntPtr result)
    {
        if (result == IntPtr.Zero)
        {
            return string.Empty;
        }

        try
        {
            return Marshal.PtrToStringUTF8(result) ?? string.Empty;
        }
        finally
        {
            osbtk_string_free(result);
        }
    }

    private static IntPtr Resolve(string libraryName, Assembly assembly, DllImportSearchPath? searchPath)
    {
        if (libraryName != Library)
        {
            return IntPtr.Zero;
        }

        string? overridePath = Environment.GetEnvironmentVariable("OSBTK_NATIVE_LIB");
        if (!string.IsNullOrEmpty(overridePath) && File.Exists(overridePath))
        {
            return NativeLibrary.Load(overridePath);
        }

        string? directory = AppContext.BaseDirectory;
        for (int depth = 0; depth < 8 && directory is not null; depth++)
        {
            foreach (string fileName in FileNames)
            {
                foreach (string profile in Profiles)
                {
                    string candidate = Path.Combine(directory, "target", profile, fileName);
                    if (File.Exists(candidate))
                    {
                        return NativeLibrary.Load(candidate);
                    }
                }
            }

            directory = Path.GetDirectoryName(directory);
        }

        return IntPtr.Zero;
    }
}
