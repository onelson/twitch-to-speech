use std::env;
use std::process::Command;
use std::path::{Path, PathBuf};

fn main() {

    let out_dir = env::var("OUT_DIR").unwrap();

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let mut archive = PathBuf::from(manifest_dir);
    archive.push("flite-2.0.0-release.tar.bz2");

    let mut src_dir = PathBuf::from(&out_dir);
    src_dir.push("flite-2.0.0-release");

    let mut lib_dir = PathBuf::from(&src_dir);
    lib_dir.push("build/x86_64-linux-gnu/lib");

    Command::new("tar").args(&["xf", &archive.as_path().to_str().unwrap()])
        .current_dir(&out_dir)
        .status().unwrap();
    Command::new("sh").args(&["configure", "--with-pic"])
        .current_dir(&src_dir)
        .status().unwrap();
    Command::new("make")
        .current_dir(&src_dir)
        .status().unwrap();

    println!("cargo:rustc-link-search=native={}", lib_dir.as_path().display());
    println!("cargo:rustc-link-lib=static=flite");
}
