//! Generates the Rust types of the schema files with `protox`, a pure-Rust compiler, so building
//! the workspace needs no external `protoc` (ADR-0004, rule 2). The snapshot schema also gets its
//! proto3 JSON mapping from `pbjson-build`, for the yata-snapshot file (ADR-0031). Its parser
//! skips unknown fields and reads an unknown enum name as the enum's zero value, because a file of
//! a newer minor version must still be read (`protocol-versions.md`); the conversion into the IR
//! refuses the zero value rather than reinterpreting it.

use prost::Message;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schemas = ["proto/core.proto", "proto/snapshot.proto"];
    for s in schemas {
        println!("cargo:rerun-if-changed={s}");
    }
    let descriptors = protox::compile(schemas, ["proto"])?;
    let encoded = descriptors.encode_to_vec();
    prost_build::Config::new().compile_fds(descriptors)?;
    pbjson_build::Builder::new()
        .register_descriptors(&encoded)?
        .ignore_unknown_fields()
        .ignore_unknown_enum_variants()
        .build(&[".yata.snapshot.v1"])?;
    Ok(())
}
