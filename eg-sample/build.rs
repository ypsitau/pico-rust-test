use std::env;
use std::path::PathBuf;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap();
    if target_os == "windows" && target_env == "msvc" {
        let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
        //println!("cargo:warning=CARGO_MANIFEST_DIR={}", manifest_dir.clone().into_string().unwrap());
        let library_dir = manifest_dir.join("lib/SDL2-2.32.8-VC/");
        let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
        //println!("cargo:warning=OUT_DIR={}", out_dir.display());
        let executable_dir = out_dir.ancestors().nth(3) // rise up three levels from the OUT_DIR
            .expect("Cargo OUT_DIR should be nested under the target profile directory")
            .to_path_buf();
        println!("cargo:rustc-link-search=native={}", library_dir.display());
        println!("cargo:rustc-link-lib=dylib=SDL2");
        std::fs::copy(&library_dir.join("SDL2.dll"), executable_dir.join("SDL2.dll"))
            .expect("failed to copy SDL2.dll to the target profile directory");
    }
}
