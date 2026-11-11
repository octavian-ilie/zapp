// Modified by Project Zapp (2026): detects locally supplied carrier PRL
// fixtures (cfg zapp_prl_fixtures). See SOURCE_ORIGIN.md.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Project Zapp: detect locally supplied carrier PRL fixtures.
    println!("cargo::rustc-check-cfg=cfg(zapp_prl_fixtures)");
    println!("cargo::rerun-if-changed=../cdma-otasp/tests");
    if std::path::Path::new("../cdma-otasp/tests/fixtures/verizon_50408.prl").is_file() {
        println!("cargo::rustc-cfg=zapp_prl_fixtures");
    }
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(
            &[
                "../../proto/events/v1/msc.proto",
                "../../proto/hlr/v1/service.proto",
            ],
            &["../../proto"],
        )?;
    Ok(())
}
