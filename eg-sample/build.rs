use std::env;
use std::path::PathBuf;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap();

    if target_os == "windows" && target_env == "msvc" {
        let library_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("lib");
        println!("cargo:rustc-link-search=native={}", library_dir.display());
        println!("cargo:rustc-link-lib=dylib=SDL2");
        println!("cargo:rerun-if-changed=lib/SDL2.lib");
    }
}
