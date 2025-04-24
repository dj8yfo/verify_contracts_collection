use cargo_near_build::{bon, camino};

fn main() {
    let build_opts = cargo_near_build::BuildOpts::builder()
        .manifest_path(camino::Utf8PathBuf::from("../community").join("Cargo.toml"))
        .build();

    let extended_build_opts = cargo_near_build::extended::BuildOptsExtended::builder()
        .build_opts(build_opts)
        .rerun_if_changed_list(bon::vec![
            "../discussions", // transitive dependecy of `devhub-community` contract
            "Cargo.toml",
            "../Cargo.lock",
        ])
        .result_file_path_env_key("BUILD_RS_SUB_BUILD_DEVHUB-COMMUNITY")
        .prepare()
        .expect("no error in auto-compute of params");

    let _wasm_path =
        cargo_near_build::extended::build_with_cli(extended_build_opts).expect("no build error");
}
