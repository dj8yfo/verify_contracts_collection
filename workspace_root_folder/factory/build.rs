use std::path::Path;

fn main() {
    // directory of target sub-contract's crate
    let workdir = "../product-donation";
    // unix path to target sub-contract's crate from root of the repo
    let nep330_contract_path = "workspace_root_folder/product-donation";

    let override_cargo_target_dir = {
        let out_dir_env = std::env::var("OUT_DIR").expect("OUT_DIR is always set in build scripts");
        let out_dir = Path::new(&out_dir_env);

        let dir = out_dir.join(format!("target-{}-for-{}", "product", "factory"));

        std::fs::create_dir_all(&dir).expect("create dir");
        dir
    };

    let mut command = {
        let mut cmd = std::process::Command::new("cargo");
        cmd.args(["near", "build", "non-reproducible-wasm", "--locked"]);

        cmd.current_dir(workdir);
        cmd.env("CARGO_TARGET_DIR", &override_cargo_target_dir);
        cmd.env("NEP330_BUILD_INFO_CONTRACT_PATH", nep330_contract_path);
        cmd
    };

    let is_abi_or_cargo_check = is_abi_build_step_or_debug_profile();
    let out_path = if is_abi_or_cargo_check {
        override_cargo_target_dir.join("empty_subcontract_stub.wasm")
    } else {
        override_cargo_target_dir.join("near/simple_factory_product/simple_factory_product.wasm")
    };

    if is_abi_or_cargo_check {
        std::fs::write(&out_path, b"").expect("success write");
    } else {
        command.status().expect("no problems in sub-build");
    }

    post_build::step(
        is_abi_or_cargo_check,
        workdir,
        out_path,
        "BUILD_RS_SUB_BUILD_ARTIFACT_1",
    );
}

mod post_build {
    use std::path::PathBuf;

    use sha2::{Digest, Sha256};
    pub fn step(
        is_abi_or_cargo_check: bool,
        workdir: &str,
        out_path: PathBuf,
        result_env_var: &str,
    ) {
        for in_path in vec![workdir, "../Cargo.toml", "../Cargo.lock"] {
            println!("cargo::rerun-if-changed={}", in_path);
        }
        println!(
            "cargo::rustc-env={}={}",
            result_env_var,
            out_path.to_str().expect("valid utf8")
        );
        if is_abi_or_cargo_check {
            println!(
                "cargo::warning={}",
                format!(
                    "subcontract empty stub is {}",
                    out_path.to_str().expect("valid utf8")
                )
            );
        } else {
            println!(
                "cargo::warning={}",
                format!(
                    "subcontract out path is {}",
                    out_path.to_str().expect("valid utf8")
                )
            );
        }

        if !is_abi_or_cargo_check {
            println!(
                "cargo::warning={}",
                format!("subcontract sha256 is {}", compute_hash(&out_path))
            );
        }
    }

    fn compute_hash(path: &PathBuf) -> String {
        let mut hasher = Sha256::new();
        hasher.update(std::fs::read(&path).expect("path read"));
        let hash = hasher.finalize();
        let hash: &[u8] = hash.as_ref();
        bs58::encode(&hash).into_string()
    }
}

pub fn is_abi_build_step_or_debug_profile() -> bool {
    if let Ok(value) = std::env::var("CARGO_NEAR_ABI_GENERATION") {
        if value == "true" {
            return true;
        }
    }
    if let Ok(value) = std::env::var("PROFILE") {
        if value == "debug" {
            return true;
        }
    }
    false
}
