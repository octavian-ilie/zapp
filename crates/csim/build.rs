// Copyright 2026 Octavian Ilie
// SPDX-License-Identifier: Apache-2.0
//
// Project Zapp: detects locally supplied carrier PRL fixtures, which are not
// distributed with Project Zapp (cfg zapp_prl_fixtures). See SOURCE_ORIGIN.md.

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rustc-check-cfg=cfg(zapp_prl_fixtures)");
    println!("cargo::rerun-if-changed=../cdma-otasp/tests");
    if std::path::Path::new("../cdma-otasp/tests/fixtures/verizon_50408.prl").is_file() {
        println!("cargo::rustc-cfg=zapp_prl_fixtures");
    }
}
