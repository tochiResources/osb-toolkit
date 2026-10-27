// Live After Effects project exporter
// -----------------------------------
// Walks the *running* After Effects project and writes a JSON model that mirrors
// the static `osb-parser-ae` crate (items / compositions / layers / properties /
// keyframes / easing / expressions), so live and on-disk sources produce the same
// shape. It deliberately captures what Jam3 `ae-to-json` omits: expression text,
// layer in/out/start times and stretch, parent links, precomp/footage source ids,
// markers and per-keyframe easing/tangents.
//
// Load it into the running AE instance and call `aeExportDump(path)`:
//   & "<AfterFX.exe>" -s "$.evalFile('.../live-dump.jsx'); aeExportDump('C:/out.json')"
// or use run.ps1. The script never throws (AE would show a modal); problems are
// recorded as {"error": "..."} entries.

function aeExp_quote(s) {
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

function aeExp_json(value) {
    if (value === null || typeof value === 'undefined') return 'null';
    var t = typeof value;
    if (t === 'number') return isFinite(value) ? String(value) : 'null';
    if (t === 'boolean') return value ? 'true' : 'false';
    if (t === 'string') return aeExp_quote(value);
    if (value instanceof Array) {
        var parts = [];
        for (var i = 0; i < value.length; i++) parts.push(aeExp_json(value[i]));
        return '[' + parts.join(',') + ']';
    }
    var keys = [];
    for (var key in value) {
        if (value.hasOwnProperty(key)) keys.push(key);
    }
    var fields = [];
    for (var k = 0; k < keys.length; k++) {
        var item = value[keys[k]];
        if (item === null || typeof item === 'undefined') continue;
        fields.push(aeExp_quote(keys[k]) + ':' + aeExp_json(item));
    }
    return '{' + fields.join(',') + '}';
}

function aeExp_round(values) {
    if (typeof values === 'number') return Math.round(values * 1000000) / 1000000;
    if (!(values instanceof Array)) return values;
    var out = [];
    for (var i = 0; i < values.length; i++) {
        out.push(
            typeof values[i] === 'number' ? Math.round(values[i] * 1000000) / 1000000 : values[i]
        );
    }
    return out;
}

function aeExp_safe(fn, fallback) {
    try {
        return fn();
    } catch (e) {
        return fallback;
    }
}

// --- property values --------------------------------------------------------

function aeExp_markerValue(marker) {
    return aeExp_safe(function () {
        return {
            time: marker.time,
            duration: marker.duration,
            comment: marker.comment,
            chapter: marker.chapter,
        };
    }, null);
}

function aeExp_textValue(doc) {
    return aeExp_safe(function () {
        return {
            text: doc.text,
            fontSize: doc.fontSize,
            font: doc.font,
            fillColor: aeExp_round(doc.fillColor),
            justification: doc.justification,
        };
    }, null);
}

function aeExp_value(prop) {
    return aeExp_safe(function () {
        var type = prop.propertyValueType;
        if (type === PropertyValueType.NO_VALUE) return null;
        if (type === PropertyValueType.MARKER) return aeExp_markerValue(prop.value);
        if (type === PropertyValueType.TEXT_DOCUMENT) return aeExp_textValue(prop.value);
        if (type === PropertyValueType.SHAPE) return null;
        if (type === PropertyValueType.LAYER_INDEX || type === PropertyValueType.MASK_INDEX)
            return prop.value;
        return aeExp_round(prop.value);
    }, null);
}

function aeExp_ease(eases) {
    var out = [];
    if (!eases) return out;
    for (var i = 0; i < eases.length; i++) {
        out.push({ speed: eases[i].speed, influence: eases[i].influence });
    }
    return out;
}

function aeExp_keyframe(prop, index) {
    var keyframe = {
        time: aeExp_safe(function () {
            return Math.round(prop.keyTime(index) * 1000000) / 1000000;
        }, null),
        value: aeExp_safe(function () {
            return aeExp_round(prop.keyValue(index));
        }, null),
    };
    keyframe.inInterpolation = aeExp_safe(function () {
        return prop.keyInInterpolationType(index);
    }, null);
    keyframe.outInterpolation = aeExp_safe(function () {
        return prop.keyOutInterpolationType(index);
    }, null);
    keyframe.inTemporalEase = aeExp_safe(function () {
        return aeExp_ease(prop.keyInTemporalEase(index));
    }, []);
    keyframe.outTemporalEase = aeExp_safe(function () {
        return aeExp_ease(prop.keyOutTemporalEase(index));
    }, []);
    keyframe.inSpatialTangent = aeExp_safe(function () {
        return aeExp_round(prop.keyInSpatialTangent(index));
    }, null);
    keyframe.outSpatialTangent = aeExp_safe(function () {
        return aeExp_round(prop.keyOutSpatialTangent(index));
    }, null);
    keyframe.spatialContinuous = aeExp_safe(function () {
        return prop.keySpatialContinuous(index);
    }, null);
    keyframe.spatialAutoBezier = aeExp_safe(function () {
        return prop.keySpatialAutoBezier(index);
    }, null);
    keyframe.temporalContinuous = aeExp_safe(function () {
        return prop.keyTemporalContinuous(index);
    }, null);
    keyframe.temporalAutoBezier = aeExp_safe(function () {
        return prop.keyTemporalAutoBezier(index);
    }, null);
    keyframe.roving = aeExp_safe(function () {
        return prop.keyRoving(index);
    }, null);
    keyframe.label = aeExp_safe(function () {
        return prop.keyLabel(index);
    }, null);
    return keyframe;
}

function aeExp_property(prop, depth, scope) {
    if (depth > 10) return null;
    var node = {
        matchName: aeExp_safe(function () {
            return prop.matchName;
        }, null),
        name: aeExp_safe(function () {
            return prop.name;
        }, null),
        propertyIndex: aeExp_safe(function () {
            return prop.propertyIndex;
        }, null),
        propertyDepth: aeExp_safe(function () {
            return prop.propertyDepth;
        }, null),
    };
    var numProperties = aeExp_safe(function () {
        return prop.numProperties;
    }, 0);

    if (numProperties > 0) {
        node.propertyType = 'GROUP';
        node.children = [];
        for (var i = 1; i <= numProperties; i++) {
            var child = aeExp_safe(function () {
                return prop.property(i);
            }, null);
            if (child) node.children.push(aeExp_property(child, depth + 1, scope));
        }
        return node;
    }

    node.propertyType = 'PROPERTY';
    node.propertyValueType = aeExp_safe(function () {
        return prop.propertyValueType;
    }, null);
    node.unitsText = aeExp_safe(function () {
        return prop.unitsText;
    }, null);
    node.canVaryOverTime = aeExp_safe(function () {
        return prop.canVaryOverTime;
    }, null);
    node.value = aeExp_value(prop);
    node.numKeys = aeExp_safe(function () {
        return prop.numKeys;
    }, 0);
    node.keyframes = [];
    for (var k = 1; k <= node.numKeys; k++) {
        node.keyframes.push(aeExp_keyframe(prop, k));
    }
    node.expression = aeExp_safe(function () {
        return prop.expressionEnabled && prop.expression ? prop.expression : null;
    }, null);
    node.expressionEnabled = aeExp_safe(function () {
        return prop.expressionEnabled;
    }, null);
    return node;
}

// --- layers and items -------------------------------------------------------

// In "storyboard" scope only these top-level layer groups are exported, which
// keeps the dump small and fast (shape/effect/mask trees are skipped).
var AE_EXP_STORYBOARD_GROUPS = {
    'ADBE Transform Group': true,
    'ADBE Marker': true,
};

function aeExp_source(layer) {
    return aeExp_safe(function () {
        var source = layer.source;
        if (!source) return null;
        var kind = 'other';
        if (source instanceof CompItem) kind = 'composition';
        else if (source instanceof FootageItem) kind = 'footage';
        return { kind: kind, id: source.id, name: source.name, typeName: source.typeName };
    }, null);
}

function aeExp_layer(layer, scope) {
    var node = {
        index: aeExp_safe(function () {
            return layer.index;
        }, null),
        id: aeExp_safe(function () {
            return layer.id;
        }, null),
        name: aeExp_safe(function () {
            return layer.name;
        }, null),
        matchName: aeExp_safe(function () {
            return layer.matchName;
        }, null),
        typeName: aeExp_safe(function () {
            return layer.typeName;
        }, null),
        source: aeExp_source(layer),
        parentId: aeExp_safe(function () {
            return layer.parent ? layer.parent.id : 0;
        }, 0),
        inPoint: aeExp_safe(function () {
            return layer.inPoint;
        }, null),
        outPoint: aeExp_safe(function () {
            return layer.outPoint;
        }, null),
        startTime: aeExp_safe(function () {
            return layer.startTime;
        }, null),
        stretch: aeExp_safe(function () {
            return layer.stretch;
        }, null),
        enabled: aeExp_safe(function () {
            return layer.enabled;
        }, null),
        solo: aeExp_safe(function () {
            return layer.solo;
        }, null),
        locked: aeExp_safe(function () {
            return layer.locked;
        }, null),
        shy: aeExp_safe(function () {
            return layer.shy;
        }, null),
        threeDLayer: aeExp_safe(function () {
            return layer.threeDLayer;
        }, null),
        adjustmentLayer: aeExp_safe(function () {
            return layer.adjustmentLayer;
        }, null),
        guideLayer: aeExp_safe(function () {
            return layer.guideLayer;
        }, null),
        blendingMode: aeExp_safe(function () {
            return layer.blendingMode;
        }, null),
        trackMatteType: aeExp_safe(function () {
            return layer.trackMatteType;
        }, null),
        properties: [],
    };
    var numProperties = aeExp_safe(function () {
        return layer.numProperties;
    }, 0);
    for (var i = 1; i <= numProperties; i++) {
        var prop = aeExp_safe(function () {
            return layer.property(i);
        }, null);
        if (!prop) continue;
        if (scope === 'storyboard') {
            var matchName = aeExp_safe(function () {
                return prop.matchName;
            }, '');
            if (!AE_EXP_STORYBOARD_GROUPS[matchName]) continue;
        }
        node.properties.push(aeExp_property(prop, 0, scope));
    }
    return node;
}

function aeExp_item(item, depth, scope) {
    if (depth > 20) return null;
    var typeName = aeExp_safe(function () {
        return item.typeName;
    }, 'Unknown');
    var node = {
        type: typeName,
        id: aeExp_safe(function () {
            return item.id;
        }, null),
        name: aeExp_safe(function () {
            return item.name;
        }, null),
        label: aeExp_safe(function () {
            return item.label;
        }, null),
    };

    if (item instanceof CompItem) {
        node.width = item.width;
        node.height = item.height;
        node.frameRate = item.frameRate;
        node.duration = item.duration;
        node.pixelAspect = aeExp_safe(function () {
            return item.pixelAspect;
        }, null);
        node.bgColor = aeExp_round(item.bgColor);
        node.numLayers = item.numLayers;
        node.layers = [];
        for (var l = 1; l <= item.numLayers; l++) {
            node.layers.push(aeExp_layer(item.layer(l), scope));
        }
    } else if (item instanceof FootageItem) {
        node.width = aeExp_safe(function () {
            return item.width;
        }, null);
        node.height = aeExp_safe(function () {
            return item.height;
        }, null);
        node.frameRate = aeExp_safe(function () {
            return item.frameRate;
        }, null);
        node.duration = aeExp_safe(function () {
            return item.duration;
        }, null);
        node.footageType = aeExp_safe(function () {
            return item.mainSource.isStill ? 'still' : 'file';
        }, null);
        node.file = aeExp_safe(function () {
            return item.mainSource.file ? item.mainSource.file.fsName : null;
        }, null);
        node.solid = aeExp_safe(function () {
            return item.mainSource.color ? aeExp_round(item.mainSource.color) : null;
        }, null);
    } else if (item instanceof FolderItem) {
        // All items are also emitted at the top level, so folders reference their
        // children by id rather than duplicating them.
        node.childIds = [];
        for (var c = 1; c <= item.numItems; c++) {
            var child = aeExp_safe(function () {
                return item.item(c);
            }, null);
            if (child) node.childIds.push(child.id);
        }
    }
    return node;
}

// --- entry points -----------------------------------------------------------

function aeExp_build(scope) {
    var model = {
        appVersion: aeExp_safe(function () {
            return app.version;
        }, null),
        bitsPerChannel: aeExp_safe(function () {
            return app.project.bitsPerChannel;
        }, null),
        expressionEngine: aeExp_safe(function () {
            return app.project.expressionEngine;
        }, null),
        file: aeExp_safe(function () {
            return app.project.file ? app.project.file.fsName : null;
        }, null),
        scope: scope,
        items: [],
    };
    for (var i = 1; i <= app.project.numItems; i++) {
        var item = aeExp_safe(function () {
            return app.project.item(i);
        }, null);
        if (item) model.items.push(aeExp_item(item, 0, scope));
    }
    return model;
}

function aeExportDump(path, scope) {
    var target = path || Folder.temp.fsName + '/ae-project.json';
    var text;
    try {
        text = aeExp_json(aeExp_build(scope || 'storyboard'));
    } catch (e) {
        text = aeExp_json({ error: String(e) });
    }
    var file = new File(target);
    file.encoding = 'UTF-8';
    file.open('w');
    file.write(text);
    file.close();
    return target;
}

function aeExportStoryboard(path) {
    return aeExportDump(path, 'storyboard');
}

function aeExportFull(path) {
    return aeExportDump(path, 'full');
}
