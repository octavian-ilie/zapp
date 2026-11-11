// Modified by Project Zapp (2026): detects locally supplied PRL fixtures and
// reference voice codecs (cfgs zapp_prl_fixtures, zapp_reference_codecs).
// See SOURCE_ORIGIN.md.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Project Zapp: detect locally supplied third-party material.
    println!("cargo::rustc-check-cfg=cfg(zapp_prl_fixtures)");
    println!("cargo::rerun-if-changed=../cdma-otasp/tests");
    if std::path::Path::new("../cdma-otasp/tests/fixtures/verizon_50408.prl").is_file() {
        println!("cargo::rustc-cfg=zapp_prl_fixtures");
    }
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
                "../../proto/msc_management/v1/service.proto",
                "../../proto/base_station/v1/service.proto",
                "../../proto/voice_gateway/v1/service.proto",
                "../../proto/an/v1/service.proto",
                "../../proto/packet/v1/service.proto",
                "../../proto/bts_management/v1/service.proto",
                "../../proto/bsc_management/v1/service.proto",
                "../../proto/pcf_management/v1/service.proto",
                "../../proto/pcf/v1/service.proto",
                "../../proto/pdsn_management/v1/service.proto",
                "../../proto/management/v1/service.proto",
                "../../proto/mgmt/v1/service.proto",
            ],
            &["../../proto"],
        )?;
    Ok(())
}
