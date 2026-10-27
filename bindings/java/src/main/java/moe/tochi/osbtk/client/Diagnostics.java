package moe.tochi.osbtk.client;

import moe.tochi.osbtk.ApiException;
import moe.tochi.osbtk.api.DiagnosticsAPI;
import moe.tochi.osbtk.model.DiagnosticReport;
import moe.tochi.osbtk.model.TextRequest;
import java.util.List;

public final class Diagnostics {
    private final DiagnosticsAPI api;

    public Diagnostics(DiagnosticsAPI api) {
        this.api = api;
    }

    public static boolean instanceOf(Object value) {
        return value instanceof Diagnostics;
    }

    public DiagnosticReport diagnoseOsb(TextRequest body) throws ApiException {
        return api.diagnoseOsb(body);
    }

    public DiagnosticReport diagnoseOsu(TextRequest body) throws ApiException {
        return api.diagnoseOsu(body);
    }
}
