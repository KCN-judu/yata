//! Generates the Rust types of the schema files with `protox`, a pure-Rust compiler, so building
//! the workspace needs no external `protoc` (ADR-0004, rule 2; ADR-0006, rule 2).

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schemas = ["proto/probe.proto"];
    for s in schemas {
        println!("cargo:rerun-if-changed={s}");
    }
    let descriptors = protox::compile(schemas, ["proto"])?;
    prost_build::Config::new().compile_fds(descriptors)?;
    Ok(())
}
