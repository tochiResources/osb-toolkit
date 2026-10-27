namespace tochiRes.osbTK;

/// <summary>Thrown when the native runtime returns an error for a bridge call.</summary>
public sealed class OsbTkException : Exception
{
    /// <summary>Creates an empty exception.</summary>
    public OsbTkException() { }

    /// <summary>Creates an exception with a message.</summary>
    public OsbTkException(string message)
        : base(message) { }

    /// <summary>Creates an exception with a message and an inner exception.</summary>
    public OsbTkException(string message, Exception innerException)
        : base(message, innerException) { }
}
