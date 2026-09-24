//! Generates the Rust types of the fact schema with `protox`, a pure-Rust compiler, so building
//! the daemon needs no external `protoc` (ADR-0002, § The fact schema).

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schemas = ["proto/fact.proto"];
    for s in schemas {
        println!("cargo:rerun-if-changed={s}");
    }
    let descriptors = protox::compile(schemas, ["proto"])?;
    prost_build::Config::new().compile_fds(descriptors)?;
    Ok(())
}
