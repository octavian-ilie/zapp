// Modified by Project Zapp (2026): detects locally supplied carrier PRL
// fixtures and reference voice codecs (cfgs zapp_prl_fixtures,
// zapp_reference_codecs). See SOURCE_ORIGIN.md.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo::rustc-check-cfg=cfg(zapp_reference_codecs)");
    println!("cargo::rerun-if-changed=../cdma-voice");
    if std::path::Path::new("../cdma-voice/csrc/include").is_dir() {
        println!("cargo::rustc-cfg=zapp_reference_codecs");
    }
    // Project Zapp: detect locally supplied carrier PRL fixtures.
    println!("cargo::rustc-check-cfg=cfg(zapp_prl_fixtures)");
    println!("cargo::rerun-if-changed=../cdma-otasp/tests");
    if std::path::Path::new("../cdma-otasp/tests/fixtures/verizon_50408.prl").is_file() {
        println!("cargo::rustc-cfg=zapp_prl_fixtures");
    }
    if std::env::var_os("CARGO_FEATURE_GRPC").is_some() {
        tonic_build::configure()
            .build_server(true)
            .build_client(true)
            .compile_protos(&["../../proto/ms/v1/service.proto"], &["../../proto"])?;
    }
    Ok(())
}
