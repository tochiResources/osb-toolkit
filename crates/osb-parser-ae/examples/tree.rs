//! Print a project's item tree (folders + items).

use osb_parser_ae::{Item, ItemType};

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../templates/after-effects/2026/AeToOsb/AeToOsb-Test-Project.aep")
            .to_string_lossy()
            .into_owned()
    });
    let project = osb_parser_ae::open(&path).expect("parse");
    for item in &project.root_folder.folder_contents {
        print_item(item, 0);
    }
}

fn print_item(item: &Item, depth: usize) {
    let pad = "  ".repeat(depth);
    let kind = match item.item_type {
        ItemType::Folder => "Folder",
        ItemType::Composition => "Comp",
        ItemType::Footage => "Footage",
        ItemType::Unknown => "?",
    };
    println!(
        "{pad}[{kind}] {:?} (id={}, children={}, layers={})",
        item.name,
        item.id,
        item.folder_contents.len(),
        item.composition_layers.len()
    );
    for child in &item.folder_contents {
        print_item(child, depth + 1);
    }
}
