package moe.tochi.osbtk.client.libraries;

import moe.tochi.osbtk.ApiException;
import moe.tochi.osbtk.api.DiagnosticsAPI;
import moe.tochi.osbtk.model.DiagnosticReport;
import moe.tochi.osbtk.model.TextRequest;
import moe.tochi.osbtk.client.TextOptions;
import java.util.List;

/**
 * The clean `Diagnostics` client.
 */
public final class Diagnostics {
    private final DiagnosticsAPI api;

    public Diagnostics(DiagnosticsAPI api) {
        this.api = api;
    }

    public static boolean instanceOf(Object value) {
        return value instanceof Diagnostics;
    }

    /**
     * A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @param text The text payload.
     *
     * @return Diagnostics for an `.osb` storyboard
     */
    public DiagnosticReport diagnoseOsb(String text) throws ApiException {
        return diagnoseOsb(TextOptions.builder().text(text).build());
    }

    /**
     * A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @param options A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @return Diagnostics for an `.osb` storyboard
     */
    public DiagnosticReport diagnoseOsb(TextOptions options) throws ApiException {
        return api.diagnoseOsb(new TextRequest()
            .text(options.text));
    }

    /**
     * A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @param text The text payload.
     *
     * @return Diagnostics for an `.osu` beatmap
     */
    public DiagnosticReport diagnoseOsu(String text) throws ApiException {
        return diagnoseOsu(TextOptions.builder().text(text).build());
    }

    /**
     * A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @param options A request carrying a block of text (an effect script, `.osu` or `.osb`).
     *
     * @return Diagnostics for an `.osu` beatmap
     */
    public DiagnosticReport diagnoseOsu(TextOptions options) throws ApiException {
        return api.diagnoseOsu(new TextRequest()
            .text(options.text));
    }
}
