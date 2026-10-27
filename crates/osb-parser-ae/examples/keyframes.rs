//! Print the animated properties (keyframes, easing, expressions) of a project's
//! compositions.
//!
//! ```text
//! cargo run -p osb-parser-ae --example keyframes -- path/to/project.aep [comp-substring]
//! ```

use osb_parser_ae::{Property, Stream};

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../templates/after-effects/2026/AeToOsb/AeToOsb-Test-Project.aep")
            .to_string_lossy()
            .into_owned()
    });
    let filter = std::env::args().nth(2);
    let project = osb_parser_ae::open(&path).expect("parse project");

    for comp in project.compositions() {
        if let Some(filter) = &filter {
            if !comp.name.contains(filter.as_str()) {
                continue;
            }
        }
        println!(
            "\n=== {} ({}x{} @ {:.3}fps) ===",
            comp.name,
            comp.footage_dimensions[0],
            comp.footage_dimensions[1],
            comp.footage_framerate
        );
        for layer in &comp.composition_layers {
            let animated: Vec<&Property> = layer
                .properties
                .iter()
                .flat_map(leaf_properties)
                .filter(|prop| prop.stream.as_ref().is_some_and(is_interesting))
                .collect();
            if animated.is_empty() {
                continue;
            }
            println!("\n  layer #{} '{}'", layer.index, layer.name);
            for prop in animated {
                print_property(prop, "    ");
            }
        }
    }
}

fn leaf_properties(property: &Property) -> Vec<&Property> {
    if property.stream.is_some() {
        return vec![property];
    }
    property
        .properties
        .iter()
        .flat_map(leaf_properties)
        .collect()
}

fn is_interesting(stream: &Stream) -> bool {
    !stream.keyframes.is_empty() || stream.expression.is_some()
}

fn print_property(property: &Property, indent: &str) {
    let Some(stream) = &property.stream else {
        return;
    };
    println!(
        "{indent}{} [{}] animated={} dims={}",
        property.match_name, property.name, stream.animated, stream.dimensions
    );
    if let Some(expression) = &stream.expression {
        println!(
            "{indent}  expression(enabled={}): {}",
            stream.expression_enabled,
            expression.replace(['\r', '\n'], " ")
        );
    }
    for keyframe in &stream.keyframes {
        println!(
            "{indent}  t={:.4}s ({}) value={:?} in={}/{:?} out={}/{:?} spatial={:?}",
            keyframe.time_seconds,
            keyframe.time_units,
            round(&keyframe.value),
            keyframe.in_interpolation.name(),
            round(&keyframe.in_speed),
            keyframe.out_interpolation.name(),
            round(&keyframe.out_speed),
            round(&keyframe.out_tangent),
        );
    }
}

fn round(values: &[f64]) -> Vec<f64> {
    values
        .iter()
        .map(|value| (value * 1000.0).round() / 1000.0)
        .collect()
}
