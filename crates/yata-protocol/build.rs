//! Generates the Rust types of the schema files with `protox`, a pure-Rust compiler, so building
//! the workspace needs no external `protoc` (ADR-0004, rule 2; ADR-0006, rule 2). The probe
//! and snapshot schemas also get their proto3 JSON mapping from `pbjson-build`, for the export file
//! (ADR-0008) and the yata-snapshot file (ADR-0031).
//! Its parser skips unknown fields and reads an unknown enum name as the enum's zero value,
//! because a file of a newer minor version must still be accepted (`protocol-versions.md`).

use prost::Message;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schemas = [
        "proto/probe.proto",
        "proto/core.proto",
        "proto/snapshot.proto",
    ];
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
        .build(&[".yata.probe.v1", ".yata.snapshot.v1"])?;
    Ok(())
}
