// Modified by Project Zapp (2026): detects the locally supplied reference voice
// codecs and EV-DO capture (cfgs zapp_reference_codecs,
// zapp_evdo_downlink_capture). See SOURCE_ORIGIN.md.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Project Zapp: detect locally supplied third-party material.
    println!("cargo::rustc-check-cfg=cfg(zapp_reference_codecs)");
    println!("cargo::rerun-if-changed=../cdma-voice");
    if std::path::Path::new("../cdma-voice/csrc/include").is_dir() {
        println!("cargo::rustc-cfg=zapp_reference_codecs");
    }
    println!("cargo::rustc-check-cfg=cfg(zapp_evdo_downlink_capture)");
    println!("cargo::rerun-if-changed=../../test/iq");
    if std::path::Path::new("../../test/iq/evdo_downlink_884490_4x_chip.wav").is_file() {
        println!("cargo::rustc-cfg=zapp_evdo_downlink_capture");
    }
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile_protos(
            &[
                "../../proto/bsc/v1/service.proto",
                "../../proto/bts_management/v1/service.proto",
            ],
            &["../../proto"],
        )?;
    Ok(())
}
