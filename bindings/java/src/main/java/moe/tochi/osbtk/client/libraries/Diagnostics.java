package moe.tochi.osbtk.client.libraries;

import moe.tochi.osbtk.ApiException;
import moe.tochi.osbtk.api.DiagnosticsAPI;
import moe.tochi.osbtk.model.DiagnosticReport;
import moe.tochi.osbtk.model.TextRequest;
import moe.tochi.osbtk.client.TextOptions;
import java.util.List;

public final class Diagnostics {
    private final DiagnosticsAPI api;

    public Diagnostics(DiagnosticsAPI api) {
        this.api = api;
    }

    public static boolean instanceOf(Object value) {
        return value instanceof Diagnostics;
    }

    public DiagnosticReport diagnoseOsb(TextOptions options) throws ApiException {
        return api.diagnoseOsb(new TextRequest()
            .text(options.text));
    }

    public DiagnosticReport diagnoseOsu(TextOptions options) throws ApiException {
        return api.diagnoseOsu(new TextRequest()
            .text(options.text));
    }
}
