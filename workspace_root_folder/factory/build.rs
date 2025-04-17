use std::path::Path;
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

fn override_cargo_target_dir() -> std::path::PathBuf {
    let out_dir_env = std::env::var("OUT_DIR").expect("OUT_DIR is always set in build scripts");
    let out_dir = Path::new(&out_dir_env);

    let dir = out_dir.join(format!("target-{}-for-{}", "product", "factory"));

    std::fs::create_dir_all(&dir).expect("create dir");
    dir
}

fn passed_env_logic(args: &mut Vec<String>, passed_env_keys: Vec<&str>) {
    let env_key_assingments = passed_env_keys
        .iter()
        .filter_map(|key| {
            std::env::var(key)
                .ok()
                .map(|value| [key, value.as_str()].join("="))
        })
        .flat_map(|env_key_assingment| vec!["--env".to_string(), env_key_assingment]);

    args.extend(env_key_assingments);
}

fn main() {
    // directory of target sub-contract's crate
    let workdir = "../product-donation";
    // unix path to `workdir` from root of the repo
    let nep330_contract_path = "workspace_root_folder/product-donation";

    // this is wasm result path of `cargo near build non-reproducible-wasm`
    // for target sub-contract, where repo's root is replaced with `/home/near/code`
    let nep330_output_wasm_path: &str = "/home/near/code/workspace_root_folder/target/near/simple_factory_product/simple_factory_product.wasm";

    let override_cargo_target_dir = override_cargo_target_dir();

    let passed_env_keys = vec!["KEY", "GOOGLE_QUERY"];

    let command = {
        let mut cmd = std::process::Command::new("cargo");
        let args = {
            let mut args = vec!["near", "build", "non-reproducible-wasm", "--locked"]
                .into_iter()
                .map(String::from)
                .collect::<Vec<_>>();

            passed_env_logic(&mut args, passed_env_keys);

            args
        };
        cmd.args(args);

        cmd.current_dir(workdir);
        cmd.env("CARGO_TARGET_DIR", &override_cargo_target_dir);
        cmd.env("NEP330_BUILD_INFO_CONTRACT_PATH", nep330_contract_path);
        cmd.env(
            "NEP330_BUILD_INFO_OUTPUT_WASM_PATH",
            nep330_output_wasm_path,
        );
        cmd.env("NO_COLOR", "true");
        cmd.stdout(std::process::Stdio::piped());
        cmd.stderr(std::process::Stdio::piped());
        cmd
    };

    let is_abi_or_cargo_check = is_abi_build_step_or_debug_profile();

    let out_path = run_build::step(command, is_abi_or_cargo_check, &override_cargo_target_dir);

    post_build::step(
        is_abi_or_cargo_check,
        vec![workdir, "../Cargo.toml", "../Cargo.lock"],
        out_path,
        "BUILD_RS_SUB_BUILD_ARTIFACT_1",
    );
}

mod run_build {

    use std::path::PathBuf;
    const RESULT_PREFIX: &str = "     -                Binary: ";

    pub fn step(
        mut command: std::process::Command,
        is_abi_or_cargo_check: bool,
        override_cargo_target_dir: &PathBuf,
    ) -> PathBuf {
        if is_abi_or_cargo_check {
            let out_path = override_cargo_target_dir.join("empty_subcontract_stub.wasm");
            std::fs::write(&out_path, b"").expect("success write");
            out_path
        } else {
            let process = command.spawn().expect("could not spawn cargo-near");
            let output = process
                .wait_with_output()
                .expect("waiting for cargo-near to finish");
            if !output.status.success() {
                panic!(
                    "build with cargo-near failure: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }

            let stderr = String::from_utf8_lossy(&output.stderr);
            let result_line = stderr
                .lines()
                .filter(|x| x.starts_with(RESULT_PREFIX))
                .last();

            let out_path = result_line
                .expect("no output line find in cargo-near output")
                .strip_prefix(RESULT_PREFIX);
            PathBuf::from(out_path.expect("starts with expected prefix").to_string())
        }
    }
}

mod post_build {
    use std::path::PathBuf;

    use sha2::{Digest, Sha256};
    pub fn step(
        is_abi_or_cargo_check: bool,
        watched_paths: Vec<&str>,
        out_path: PathBuf,
        result_env_var: &str,
    ) {
        for in_path in watched_paths {
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
                    "subcontract empty stub is `{}`",
                    out_path.to_str().expect("valid utf8")
                )
            );
        } else {
            println!(
                "cargo::warning={}",
                format!(
                    "subcontract out path is `{}`",
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
