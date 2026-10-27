// AE in-memory event recorder
// ---------------------------
// After Effects ExtendScript has no general event model (no "selection changed"
// or "hotkey pressed" callbacks). The supported technique is to poll the live
// application object model on a timer and log what changed. This script does
// exactly that: every tick it snapshots the active item, the active comp's
// playhead / work area / selected layers, and each tracked layer's transform
// values, keyframe counts, keyframe times and expressions, then appends a JSON
// line whenever anything differs from the previous tick.
//
// Layers are tracked by index, so a change of selection no longer produces a
// false diff, and each change is reported as a flat dotted key, e.g.
//   "comp.selected", "comp.time",
//   "layer:3.position.numKeys", "layer:3.position.keys",
//   "layer:3.opacity.expression", "layer:3.blendingMode".
// By default only the selected layers are tracked; set
//   $.global.AE_RECORDER_ALL_LAYERS = true;
// before starting to track every layer of the active composition.
//
// It reads the *running* After Effects instance. The easiest way to drive it is
// `run.ps1`, which loads this file into AE and calls `aeRecTick()` on a timer
// (see the note in the public-API section on why `app.scheduleTask` is not used).
// Loaded on its own it only defines the functions below; nothing is recorded
// until `aeRecStart()` / `aeRecTick()` are called.
//
// Output: a JSONL file (one JSON object per line). Default path:
//   %TEMP%\opencode\ae-events.jsonl
// Override before starting:  $.global.AE_RECORDER_PATH = "C:/some/where.jsonl";
//
// API (call from the ExtendScript console or another script):
//   aeRecStart()        begin a session (resets the diff baseline, writes a start line)
//   aeRecTick()         snapshot and append a line if anything changed
//   aeRecStop()         append a stop line
//   aeRecMark("label")  append a labelled marker line
//   aeRecSnapshot()     append one full snapshot line immediately
//
// The script never throws out of a tick (a thrown error would pop a modal
// dialog in AE); failures are logged into the JSONL as {"event":"error"}.

var AE_RECORDER_INTERVAL_MS = 250;
if (typeof $.global.AE_RECORDER_PATH === 'undefined') {
    $.global.AE_RECORDER_PATH = Folder.temp.fsName + '/ae-events.jsonl';
}
if (typeof $.global.AE_RECORDER_ALL_LAYERS === 'undefined') {
    $.global.AE_RECORDER_ALL_LAYERS = false;
}

// --- minimal JSON (ExtendScript has no guaranteed JSON object) --------------

function aeRec_quote(s) {
    var out = '"';
    for (var i = 0; i < s.length; i++) {
        var c = s.charAt(i);
        var code = s.charCodeAt(i);
        if (c === '"') out += '\\"';
        else if (c === '\\') out += '\\\\';
        else if (c === '\n') out += '\\n';
        else if (c === '\r') out += '\\r';
        else if (c === '\t') out += '\\t';
        else if (code < 0x20) out += '\\u' + ('000' + code.toString(16)).slice(-4);
        else out += c;
    }
    return out + '"';
}

function aeRec_json(value) {
    if (value === null || typeof value === 'undefined') return 'null';
    var t = typeof value;
    if (t === 'number') return isFinite(value) ? String(value) : 'null';
    if (t === 'boolean') return value ? 'true' : 'false';
    if (t === 'string') return aeRec_quote(value);
    if (value instanceof Array) {
        var parts = [];
        for (var i = 0; i < value.length; i++) parts.push(aeRec_json(value[i]));
        return '[' + parts.join(',') + ']';
    }
    var keys = [];
    for (var key in value) {
        if (value.hasOwnProperty(key)) keys.push(key);
    }
    keys.sort();
    var fields = [];
    for (var k = 0; k < keys.length; k++) {
        fields.push(aeRec_quote(keys[k]) + ':' + aeRec_json(value[keys[k]]));
    }
    return '{' + fields.join(',') + '}';
}

// --- file output ------------------------------------------------------------

function aeRec_append(line) {
    var file = new File($.global.AE_RECORDER_PATH);
    if (file.exists) {
        file.open('e');
        file.seek(0, 2);
    } else {
        file.open('w');
    }
    file.encoding = 'UTF-8';
    file.lineFeed = 'Unix';
    file.write(line + '\n');
    file.close();
}

// --- snapshotting -----------------------------------------------------------

function aeRec_roundArray(values) {
    var out = [];
    for (var i = 0; i < values.length; i++) {
        out.push(Math.round(values[i] * 100000) / 100000);
    }
    return out;
}

// A property digest: current value, keyframe count, key times and expression.
function aeRec_prop(prop) {
    var digest = { value: null, numKeys: 0, keys: [], expression: null };
    try {
        digest.numKeys = prop.numKeys;
        if (prop.numKeys > 0) {
            digest.value = aeRec_roundArray(prop.keyValue(prop.numKeys));
            for (var k = 1; k <= prop.numKeys; k++) {
                digest.keys.push(Math.round(prop.keyTime(k) * 1000) / 1000);
            }
        } else {
            digest.value = aeRec_roundArray(prop.value);
        }
    } catch (e) {
        digest.error = String(e);
    }
    try {
        if (prop.expressionEnabled && prop.expression && prop.expression.length > 0) {
            digest.expression = prop.expression;
        }
    } catch (e) {}
    return digest;
}

function aeRec_layer(layer) {
    var info = {
        index: layer.index,
        name: layer.name,
        enabled: null,
        solo: null,
        locked: null,
        shy: null,
        inPoint: null,
        outPoint: null,
        startTime: null,
        blendingMode: null,
        threeDLayer: null,
        transform: {},
    };
    try {
        info.enabled = layer.enabled;
    } catch (e) {}
    try {
        info.solo = layer.solo;
    } catch (e) {}
    try {
        info.locked = layer.locked;
    } catch (e) {}
    try {
        info.shy = layer.shy;
    } catch (e) {}
    try {
        info.inPoint = layer.inPoint;
    } catch (e) {}
    try {
        info.outPoint = layer.outPoint;
    } catch (e) {}
    try {
        info.startTime = layer.startTime;
    } catch (e) {}
    try {
        info.blendingMode = layer.blendingMode;
    } catch (e) {}
    try {
        info.threeDLayer = layer.threeDLayer;
    } catch (e) {}
    try {
        var t = layer.transform;
        if (t) {
            info.transform.anchorPoint = aeRec_prop(t.anchorPoint);
            info.transform.position = aeRec_prop(t.position);
            info.transform.scale = aeRec_prop(t.scale);
            info.transform.rotation = aeRec_prop(t.rotation);
            info.transform.opacity = aeRec_prop(t.opacity);
        }
    } catch (e) {
        info.transform.error = String(e);
    }
    return info;
}

// Tracked layers, keyed by index (selected layers by default; every layer of the
// active composition when AE_RECORDER_ALL_LAYERS is set).
function aeRec_trackedLayers(comp) {
    var layers = {};
    var list;
    if ($.global.AE_RECORDER_ALL_LAYERS) {
        list = [];
        for (var i = 1; i <= comp.numLayers; i++) list.push(comp.layer(i));
    } else {
        list = comp.selectedLayers;
    }
    for (var l = 0; l < list.length; l++) {
        var digest = aeRec_layer(list[l]);
        layers[String(digest.index)] = digest;
    }
    return layers;
}

function aeRec_snapshot() {
    var snap = {
        projectFile: null,
        projectDirty: null,
        projectItems: null,
        activeItem: null,
        activeItemType: null,
        comp: null,
    };
    try {
        snap.projectFile = app.project.file ? app.project.file.fsName : null;
    } catch (e) {}
    try {
        snap.projectDirty = app.project.dirty;
    } catch (e) {}
    try {
        snap.projectItems = app.project.numItems;
    } catch (e) {}

    var active = null;
    try {
        active = app.project.activeItem;
    } catch (e) {}
    if (active) {
        try {
            snap.activeItem = active.name;
        } catch (e) {}
        try {
            snap.activeItemType = active.typeName;
        } catch (e) {}
    }

    if (active && active instanceof CompItem) {
        var comp = {
            id: active.id,
            name: active.name,
            time: null,
            workAreaStart: null,
            workAreaDuration: null,
            numLayers: null,
            selected: [],
            layers: {},
        };
        try {
            comp.time = active.time;
        } catch (e) {}
        try {
            comp.workAreaStart = active.workAreaStart;
        } catch (e) {}
        try {
            comp.workAreaDuration = active.workAreaDuration;
        } catch (e) {}
        try {
            comp.numLayers = active.numLayers;
        } catch (e) {}

        var selected = [];
        try {
            for (var s = 0; s < active.selectedLayers.length; s++) {
                selected.push(active.selectedLayers[s].index + ':' + active.selectedLayers[s].name);
            }
        } catch (e) {}
        comp.selected = selected;

        try {
            comp.layers = aeRec_trackedLayers(active);
        } catch (e) {}
        snap.comp = comp;
    }
    return snap;
}

// --- diffing and ticking ----------------------------------------------------

var AE_REC_SCALARS = [
    'projectFile',
    'projectDirty',
    'projectItems',
    'activeItem',
    'activeItemType',
];
var AE_REC_LAYER_FIELDS = [
    'name',
    'enabled',
    'solo',
    'locked',
    'shy',
    'inPoint',
    'outPoint',
    'startTime',
    'blendingMode',
    'threeDLayer',
];
var AE_REC_TRANSFORM_PROPS = ['anchorPoint', 'position', 'scale', 'rotation', 'opacity'];

function aeRec_put(changes, key, from, to) {
    changes[key] = { from: from, to: to };
}

// Diff two index-keyed layer maps, reporting only changes to the same layer.
function aeRec_diffLayers(prevLayers, curLayers, changes) {
    for (var index in curLayers) {
        if (!curLayers.hasOwnProperty(index)) continue;
        var cur = curLayers[index];
        var prev = prevLayers ? prevLayers[index] : null;
        if (!prev) {
            aeRec_put(changes, 'layer:' + index, null, { name: cur.name });
            continue;
        }
        for (var f = 0; f < AE_REC_LAYER_FIELDS.length; f++) {
            var field = AE_REC_LAYER_FIELDS[f];
            if (aeRec_json(prev[field]) !== aeRec_json(cur[field])) {
                aeRec_put(changes, 'layer:' + index + '.' + field, prev[field], cur[field]);
            }
        }
        for (var p = 0; p < AE_REC_TRANSFORM_PROPS.length; p++) {
            var name = AE_REC_TRANSFORM_PROPS[p];
            var before = prev.transform ? prev.transform[name] : null;
            var after = cur.transform ? cur.transform[name] : null;
            if (aeRec_json(before) === aeRec_json(after)) continue;
            var subs = ['value', 'numKeys', 'keys', 'expression'];
            for (var s = 0; s < subs.length; s++) {
                var sub = subs[s];
                var a = before ? before[sub] : null;
                var b = after ? after[sub] : null;
                if (aeRec_json(a) !== aeRec_json(b)) {
                    aeRec_put(changes, 'layer:' + index + '.' + name + '.' + sub, a, b);
                }
            }
        }
    }
    for (var gone in prevLayers) {
        if (!prevLayers.hasOwnProperty(gone)) continue;
        if (!curLayers || !curLayers.hasOwnProperty(gone)) {
            aeRec_put(changes, 'layer:' + gone, { name: prevLayers[gone].name }, null);
        }
    }
}

function aeRec_diffSnapshot(prev, cur) {
    if (!prev) {
        return { snapshot: cur };
    }
    var changes = {};
    for (var i = 0; i < AE_REC_SCALARS.length; i++) {
        var key = AE_REC_SCALARS[i];
        if (aeRec_json(prev[key]) !== aeRec_json(cur[key])) {
            aeRec_put(changes, key, prev[key], cur[key]);
        }
    }

    var pc = prev.comp;
    var cc = cur.comp;
    if (aeRec_json(pc) !== aeRec_json(cc)) {
        if (!pc && cc) {
            aeRec_put(changes, 'comp', null, { name: cc.name });
        } else if (pc && !cc) {
            aeRec_put(changes, 'comp', { name: pc.name }, null);
        } else {
            var compScalars = [
                'id',
                'name',
                'time',
                'workAreaStart',
                'workAreaDuration',
                'numLayers',
            ];
            for (var c = 0; c < compScalars.length; c++) {
                var ck = compScalars[c];
                if (aeRec_json(pc[ck]) !== aeRec_json(cc[ck])) {
                    aeRec_put(changes, 'comp.' + ck, pc[ck], cc[ck]);
                }
            }
            if (aeRec_json(pc.selected) !== aeRec_json(cc.selected)) {
                aeRec_put(changes, 'comp.selected', pc.selected, cc.selected);
            }
            aeRec_diffLayers(pc.layers, cc.layers, changes);
        }
    }
    return changes;
}

function aeRec_hasKeys(object) {
    for (var key in object) {
        if (object.hasOwnProperty(key)) return true;
    }
    return false;
}

function aeRec_now() {
    var d = new Date();
    return d.getTime();
}

function aeRecTick() {
    try {
        var snap = aeRec_snapshot();
        var changes = aeRec_diffSnapshot($.global.AE_RECORDER_PREV, snap);
        $.global.AE_RECORDER_PREV = snap;
        if (aeRec_hasKeys(changes)) {
            aeRec_append(aeRec_json({ ts: aeRec_now(), event: 'change', changes: changes }));
        }
    } catch (e) {
        try {
            aeRec_append(aeRec_json({ ts: aeRec_now(), event: 'error', message: String(e) }));
        } catch (ignored) {}
    }
}

// --- public API -------------------------------------------------------------
//
// These functions are defined in the persistent ExtendScript global scope, so
// they survive between separate `AfterFX.exe -s` invocations. The host-side
// `run.ps1` drives the recording by calling `aeRecTick()` on a timer, because
// `app.scheduleTask` is not pumped in the command-line engine.

function aeRecStart() {
    $.global.AE_RECORDER_PREV = null;
    aeRec_append(
        aeRec_json({
            ts: aeRec_now(),
            event: 'start',
            path: $.global.AE_RECORDER_PATH,
            appVersion: app.version,
        })
    );
}

function aeRecStop() {
    aeRec_append(aeRec_json({ ts: aeRec_now(), event: 'stop' }));
}

function aeRecMark(label) {
    aeRec_append(aeRec_json({ ts: aeRec_now(), event: 'mark', label: String(label) }));
}

function aeRecSnapshot() {
    aeRec_append(aeRec_json({ ts: aeRec_now(), event: 'snapshot', snapshot: aeRec_snapshot() }));
}

// Best-effort in-application scheduling for when this file is run from AE's UI
// (File > Scripts > Run Script File). The command-line engine used by run.ps1
// does not pump scheduled tasks, so run.ps1 polls aeRecTick() instead.
function aeRecStartScheduled(intervalMs) {
    aeRecStart();
    var interval = intervalMs || AE_RECORDER_INTERVAL_MS;
    if ($.global.AE_RECORDER_TASK) {
        app.cancelTask($.global.AE_RECORDER_TASK);
    }
    $.global.AE_RECORDER_TASK = app.scheduleTask('aeRecTick()', interval, true);
}
