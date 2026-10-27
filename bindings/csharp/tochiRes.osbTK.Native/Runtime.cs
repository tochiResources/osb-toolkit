namespace tochiRes.osbTK.Native;

/// <summary>
/// An RAII handle to a live osb runtime (storyboard + context). Drive the
/// layer/sprite/command surface through <see cref="Exec"/> with JSON requests
/// (`createSprite`, `command`, `spriteStateAt`, `importOsb`, `spriteCount`, ...).
/// </summary>
public sealed class Runtime : IDisposable
{
    private IntPtr _handle;

    public Runtime(double width = 640, double height = 480)
    {
        _handle = NativeMethods.RuntimeNew(width, height);
    }

    /// <summary>Runs a JSON request against this runtime.</summary>
    public string Exec(string requestJson) => NativeMethods.RuntimeExec(_handle, requestJson);

    public void Dispose()
    {
        if (_handle != IntPtr.Zero)
        {
            NativeMethods.RuntimeFree(_handle);
            _handle = IntPtr.Zero;
        }
    }
}
