use std::str::FromStr;

use cargo_near_build::{bon, camino, extended};
use cargo_near_build::BuildOpts;

fn main() {
    let _e = env_logger::Builder::new().parse_default_env().try_init();

    // directory of target sub-contract's crate
    let workdir = "../product-donation";
    // unix path to target sub-contract's crate from root of the repo
    let nep330_contract_path = "workspace_root_folder/product-donation";

    let manifest = camino::Utf8PathBuf::from_str(workdir)
        .expect("pathbuf from str")
        .join("Cargo.toml");
    let build_opts = BuildOpts::builder()
        .manifest_path(manifest)
        .override_nep330_contract_path(nep330_contract_path)
        // a distinct target is needed to avoid deadlock during build
        .override_cargo_target_dir("../target/build-rs-product-donation")
        .build();


    let build_script_opts = extended::BuildScriptOpts::builder()
        .rerun_if_changed_list(bon::vec![workdir, "../Cargo.toml", "../Cargo.lock",])
        .build_skipped_when_env_is(vec![
            // shorter build for `cargo check`
            ("PROFILE", "debug"),
            (cargo_near_build::env_keys::BUILD_RS_ABI_STEP_HINT, "true"),
        ])
        .stub_path("../target/stub.bin")
        .result_env_key("BUILD_RS_SUB_BUILD_ARTIFACT_1")
        .build();

    let extended_opts = extended::BuildOptsExtended::builder()
        .build_opts(build_opts)
        .build_script_opts(build_script_opts)
        .build();

    cargo_near_build::extended::build(extended_opts).expect("sub-contract build error");
}
