use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

fn main() {
    // directory of target sub-contract's crate
    let workdir = "../product-donation";
    // unix path to target sub-contract's crate from root of the repo
    let nep330_contract_path = "workspace_root_folder/product-donation";

    let override_cargo_target_dir = {
        let out_dir_env = std::env::var("OUT_DIR").expect("OUT_DIR is always set in build scripts");
        let out_dir = Path::new(&out_dir_env);

        out_dir.join(format!("target-{}-for-{}", "product", "factory"))
    };

    let mut cmd = std::process::Command::new("cargo");

    cmd.args(["near", "build", "non-reproducible-wasm", "--locked"]);

    cmd.current_dir(workdir);
    cmd.env("CARGO_TARGET_DIR", &override_cargo_target_dir);
    cmd.env("NEP330_BUILD_INFO_CONTRACT_PATH", nep330_contract_path);

    cmd.status().expect("no problems in sub-build");

    let out_path =
        override_cargo_target_dir.join("near/simple_factory_product/simple_factory_product.wasm");

    for in_path in vec![workdir, "../Cargo.toml", "../Cargo.lock"] {
        println!("cargo::rerun-if-changed={}", in_path);
    }
    println!(
        "cargo::rustc-env={}={}",
        "BUILD_RS_SUB_BUILD_ARTIFACT_1",
        out_path.to_str().expect("valid utf8")
    );
    println!(
        "cargo::warning={}",
        format!("out path is {}", out_path.to_str().expect("valid utf8"))
    );
    println!(
        "cargo::warning={}",
        format!("sha256 is {}", compute_hash(&out_path))
    );
}

pub fn compute_hash(path: &PathBuf) -> String {
    let mut hasher = Sha256::new();
    hasher.update(std::fs::read(&path).expect("path read"));
    let hash = hasher.finalize();
    let hash: &[u8] = hash.as_ref();
    bs58::encode(&hash).into_string()
}
