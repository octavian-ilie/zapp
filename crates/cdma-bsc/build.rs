// Modified by Project Zapp (2026): detects the locally supplied reference voice
// codecs (cfg zapp_reference_codecs). See SOURCE_ORIGIN.md.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Project Zapp: detect locally supplied third-party material.
    println!("cargo::rustc-check-cfg=cfg(zapp_reference_codecs)");
    println!("cargo::rerun-if-changed=../cdma-voice");
    if std::path::Path::new("../cdma-voice/csrc/include").is_dir() {
        println!("cargo::rustc-cfg=zapp_reference_codecs");
    }
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .type_attribute(
            "management.v1.ManagementEvent.Body",
            "#[allow(clippy::large_enum_variant)]",
        )
        .type_attribute(
            "management.v1.ManagementEvent.body",
            "#[allow(clippy::large_enum_variant)]",
        )
        .type_attribute(
            "management.v1.management_event.Body",
            "#[allow(clippy::large_enum_variant)]",
        )
        .compile_protos(
            &[
                "../../proto/bsc/v1/service.proto",
                "../../proto/events/v1/msc.proto",
                "../../proto/management/v1/service.proto",
                "../../proto/bts_management/v1/service.proto",
                "../../proto/bsc_management/v1/service.proto",
                "../../proto/base_station/v1/service.proto",
                "../../proto/msc_management/v1/service.proto",
                "../../proto/pcf_management/v1/service.proto",
                "../../proto/pdsn_management/v1/service.proto",
                "../../proto/hlr/v1/service.proto",
                "../../proto/smsc/v1/service.proto",
                "../../proto/packet/v1/service.proto",
            ],
            &["../../proto"],
        )?;
    Ok(())
}
