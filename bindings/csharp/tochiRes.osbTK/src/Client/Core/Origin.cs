#nullable disable
using tochiRes.osbTK.Api;
using tochiRes.osbTK.Client;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client.Core;

public sealed class Origin
{
    private readonly OriginAPI api;

    public Origin(OriginAPI api)
    {
        this.api = api;
    }

    public static bool InstanceOf(object value) => value is Origin;

    /// <summary>
    /// The anchor offset for an origin.
    /// </summary>
    /// <param name="Origin">The sprite anchor origin.</param>
    /// <param name="Width">The sprite width.</param>
    /// <param name="Height">The sprite height.</param>
    /// <returns>The anchor offset for the origin</returns>
    public Vector2 GetOriginVector(OsbOrigin Origin, double Width, double Height) =>
        GetOriginVector(new GetOriginVectorOptions { Origin = Origin, Width = Width, Height = Height });

    /// <summary>
    /// The anchor offset for an origin.
    /// </summary>
    /// <param name="options">The anchor offset for an origin.</param>
    /// <returns>The anchor offset for the origin</returns>
    public Vector2 GetOriginVector(GetOriginVectorOptions options) => api.GetOriginVector(new GetOriginVectorRequest(options.Height, options.Origin, options.Width));

    /// <summary>
    /// Whether a rotated sprite's bounding box intersects the widescreen storyboard area.
    /// </summary>
    /// <param name="Position">The sprite centre position.</param>
    /// <param name="Size">The sprite size.</param>
    /// <param name="Rotation">The rotation in radians.</param>
    /// <param name="Origin">The anchor offset (from `GetOriginVector`).</param>
    /// <returns>Whether the sprite's bounds intersect the screen</returns>
    public bool InScreenBounds(Vector2 Position, Vector2 Size, double Rotation, Vector2 Origin) =>
        InScreenBounds(new InScreenBoundsOptions { Position = Position, Size = Size, Rotation = Rotation, Origin = Origin });

    /// <summary>
    /// Whether a rotated sprite's bounding box intersects the widescreen storyboard area.
    /// </summary>
    /// <param name="options">Whether a rotated sprite's bounding box intersects the widescreen storyboard area.</param>
    /// <returns>Whether the sprite's bounds intersect the screen</returns>
    public bool InScreenBounds(InScreenBoundsOptions options) => api.InScreenBounds(new InScreenBoundsRequest(options.Origin, options.Position, options.Rotation, options.Size)).Value;
}
