//! Writes the OpenAPI document to stdout, for the SDK codegen pipeline:
//! `cargo run -p osb-api --bin osbtk-openapi > bindings/openapi/osbtk.openapi.json`.

fn main() {
    println!("{}", osb_api::openapi_json());
}
