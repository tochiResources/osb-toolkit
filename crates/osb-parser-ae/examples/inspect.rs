//! Print a summary of an After Effects project.
//!
//! ```text
//! cargo run -p osb-parser-ae --example inspect -- path/to/project.aep
//! ```

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(default_path);
    let project = match osb_parser_ae::open(&path) {
        Ok(project) => project,
        Err(error) => {
            eprintln!("failed to parse {path}: {error}");
            std::process::exit(1);
        }
    };

    println!("expression engine : {}", project.expression_engine);
    println!("bit depth         : {:?}", project.depth);
    println!("items             : {}", project.items().count());
    println!("compositions      : {}", project.compositions().count());

    for comp in project.compositions() {
        println!(
            "\n• {} — {}x{} @ {:.3} fps, {:.3}s, {} layer(s), bg #{:02x}{:02x}{:02x}",
            comp.name,
            comp.footage_dimensions[0],
            comp.footage_dimensions[1],
            comp.footage_framerate,
            comp.footage_seconds,
            comp.composition_layers.len(),
            comp.background_color[0],
            comp.background_color[1],
            comp.background_color[2],
        );
        for layer in &comp.composition_layers {
            println!(
                "    #{:<3} {:<40} src={:<6} in={:.3} out={:.3} start={:.3} stretch={:.1}% effects={:<2} text={} 3d={}",
                layer.index,
                layer.name,
                layer.source_id,
                layer.in_point,
                layer.out_point,
                layer.start_time,
                layer.stretch,
                layer.effects.len(),
                layer.text.is_some(),
                layer.three_d_enabled,
            );
        }
    }
}

fn default_path() -> String {
    let manifest = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .join("../../templates/after-effects/2026/AeToOsb/AeToOsb-Test-Project.aep")
        .to_string_lossy()
        .into_owned()
}
