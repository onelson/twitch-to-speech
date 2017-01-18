//use std::env;
//use std::process::Command;
//use std::path::{Path, PathBuf};
//
//fn main() {
//
//    let out_dir = env::var("OUT_DIR").unwrap();
//
//    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
//
//    let mut archive = PathBuf::from(manifest_dir);
//    archive.push("flite-2.0.0-release.tar.bz2");
//
//    let mut src_dir = PathBuf::from(&out_dir);
//    src_dir.push("flite-2.0.0-release");
//
//    let mut lib_dir = PathBuf::from(&src_dir);
//    lib_dir.push("build/x86_64-linux-gnu/lib");
//
//    Command::new("tar").args(&["xf", &archive.as_path().to_str().unwrap()])
//        .current_dir(&out_dir)
//        .status().unwrap();
//    Command::new("sh").args(&[
//        "configure",
//        "--disable-shared",
//        "--with-pic",
//        "--with-langvox=default",
//        "--with-audio=linux"])
//        .current_dir(&src_dir)
//        .status().unwrap();
//
//    Command::new("make")
//        .current_dir(&src_dir)
//        .status().unwrap();
//
//    println!("cargo:rustc-link-search=native={}", lib_dir.as_path().display());
//    println!("cargo:rustc-link-lib=static=flite");
//    println!("cargo:rustc-link-lib=static=flite_cmu_grapheme_lang");
//    println!("cargo:rustc-link-lib=static=flite_cmu_grapheme_lex");
//    println!("cargo:rustc-link-lib=static=flite_cmu_indic_lang");
//    println!("cargo:rustc-link-lib=static=flite_cmu_indic_lex");
//    println!("cargo:rustc-link-lib=static=flite_cmu_time_awb");
//    println!("cargo:rustc-link-lib=static=flite_cmu_us_awb");
//    println!("cargo:rustc-link-lib=static=flite_cmu_us_kal");
//    println!("cargo:rustc-link-lib=static=flite_cmu_us_kal16");
//    println!("cargo:rustc-link-lib=static=flite_cmu_us_rms");
//    println!("cargo:rustc-link-lib=static=flite_cmu_us_slt");
//    println!("cargo:rustc-link-lib=static=flite_cmulex");
//    println!("cargo:rustc-link-lib=static=flite_usenglish");
//
//}


extern crate pkg_config;

fn main() {
//    println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu");
//    println!("cargo:rustc-link-search=static=/usr/lib/x86_64-linux-gnu");
//    println!("cargo:libdir=/usr/lib/x86_64-linux-gnu");
//    println!("cargo:include=/usr/include");
//    println!("cargo:include=/usr/include/flite");
//    println!("cargo:rustc-link-lib=static=flite");
//    println!("cargo:rustc-link-lib=static=flite_cmu_grapheme_lang");
//    println!("cargo:rustc-link-lib=static=flite_cmu_grapheme_lex");
//    println!("cargo:rustc-link-lib=static=flite_cmu_indic_lang");
//    println!("cargo:rustc-link-lib=static=flite_cmu_indic_lex");
//    println!("cargo:rustc-link-lib=static=flite_cmulex");
//    println!("cargo:rustc-link-lib=static=flite_cmu_time_awb");
//    println!("cargo:rustc-link-lib=static=flite_cmu_us_awb");
//    println!("cargo:rustc-link-lib=static=flite_cmu_us_kal16");
//    println!("cargo:rustc-link-lib=static=flite_cmu_us_kal");
//    println!("cargo:rustc-link-lib=static=flite_cmu_us_rms");
//    println!("cargo:rustc-link-lib=static=flite_cmu_us_slt");
//    println!("cargo:rustc-link-lib=static=flite_usenglish");

    pkg_config::Config::new()
        .atleast_version("2.0.0").probe("flite").unwrap();

}