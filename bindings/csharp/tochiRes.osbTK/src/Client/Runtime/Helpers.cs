#nullable disable
using tochiRes.osbTK.Api;
using tochiRes.osbTK.Client;
using tochiRes.osbTK.Model;

namespace tochiRes.osbTK.Client.Runtime;

/// <summary>
/// The clean `Helpers` client.
/// </summary>
public sealed class Helpers
{
    private readonly HelpersAPI api;

    public Helpers(HelpersAPI api)
    {
        this.api = api;
    }

    public static bool InstanceOf(object value) => value is Helpers;

    /// <summary>
    /// A motion-graphics helper call: a helper `name` plus its input.
    /// </summary>
    /// <param name="Name">The helper name, e.g. `flatten`, `orbit`, `selectCommands`.</param>
    /// <returns>The shared motion-graphics helper's result</returns>
    public object Call(string Name) =>
        Call(new HelperOptions { Name = Name });

    /// <summary>
    /// A motion-graphics helper call: a helper `name` plus its input.
    /// </summary>
    /// <param name="Name">The helper name, e.g. `flatten`, `orbit`, `selectCommands`.</param>
    /// <param name="Input">The helper's input (a keyframe channel, a command list, or its parameters).</param>
    /// <returns>The shared motion-graphics helper's result</returns>
    public object Call(string Name, object Input) =>
        Call(new HelperOptions { Name = Name, Input = Input });

    /// <summary>
    /// A motion-graphics helper call: a helper `name` plus its input.
    /// </summary>
    /// <param name="options">A motion-graphics helper call: a helper `name` plus its input.</param>
    /// <returns>The shared motion-graphics helper's result</returns>
    public object Call(HelperOptions options) => api.Call(new HelperRequest(options.Input, options.Name)).Value;

    /// <summary>
    /// A motion-graphics helper call: a helper `name` plus its input.
    /// </summary>
    /// <param name="Name">The helper name, e.g. `flatten`, `orbit`, `selectCommands`.</param>
    /// <returns>The helper's result (a command list or scalar)</returns>
    public object Commands(string Name) =>
        Commands(new HelperOptions { Name = Name });

    /// <summary>
    /// A motion-graphics helper call: a helper `name` plus its input.
    /// </summary>
    /// <param name="Name">The helper name, e.g. `flatten`, `orbit`, `selectCommands`.</param>
    /// <param name="Input">The helper's input (a keyframe channel, a command list, or its parameters).</param>
    /// <returns>The helper's result (a command list or scalar)</returns>
    public object Commands(string Name, object Input) =>
        Commands(new HelperOptions { Name = Name, Input = Input });

    /// <summary>
    /// A motion-graphics helper call: a helper `name` plus its input.
    /// </summary>
    /// <param name="options">A motion-graphics helper call: a helper `name` plus its input.</param>
    /// <returns>The helper's result (a command list or scalar)</returns>
    public object Commands(HelperOptions options) => api.Commands(new HelperRequest(options.Input, options.Name)).Value;

    /// <summary>
    /// A motion-graphics helper call: a helper `name` plus its input.
    /// </summary>
    /// <param name="Name">The helper name, e.g. `flatten`, `orbit`, `selectCommands`.</param>
    /// <returns>The helper's result (a keyframe channel or vectors)</returns>
    public object Keyframes(string Name) =>
        Keyframes(new HelperOptions { Name = Name });

    /// <summary>
    /// A motion-graphics helper call: a helper `name` plus its input.
    /// </summary>
    /// <param name="Name">The helper name, e.g. `flatten`, `orbit`, `selectCommands`.</param>
    /// <param name="Input">The helper's input (a keyframe channel, a command list, or its parameters).</param>
    /// <returns>The helper's result (a keyframe channel or vectors)</returns>
    public object Keyframes(string Name, object Input) =>
        Keyframes(new HelperOptions { Name = Name, Input = Input });

    /// <summary>
    /// A motion-graphics helper call: a helper `name` plus its input.
    /// </summary>
    /// <param name="options">A motion-graphics helper call: a helper `name` plus its input.</param>
    /// <returns>The helper's result (a keyframe channel or vectors)</returns>
    public object Keyframes(HelperOptions options) => api.Keyframes(new HelperRequest(options.Input, options.Name)).Value;
}
