using TochiRes.Errors;

namespace tochiRes.osbTK;

/// <summary>Thrown when the native runtime returns an error for a bridge call.</summary>
public sealed class osbTKException : TochiResException
{
    /// <summary>Creates an exception with the default native error code.</summary>
    public osbTKException()
        : base(ErrorCodes.NativeCallFailed, "unknown native error") { }

    /// <summary>Creates an exception with a message.</summary>
    public osbTKException(string message)
        : base(ErrorCodes.NativeCallFailed, message) { }

    /// <summary>Creates an exception with a message and an inner exception.</summary>
    public osbTKException(string message, Exception innerException)
        : base(ErrorCodes.NativeCallFailed, message, innerException) { }
}
