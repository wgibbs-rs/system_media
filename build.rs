use std::process::Command;
use std::env;

#[cfg(target_os = "macos")]
fn main() {

    // Path to your Swift source file
    let swift_file = "src/macos/nowplaying.swift";

    // Output directory for compiled library
    let out_dir = env::var("OUT_DIR").unwrap();
    let lib_path = format!("{}/libswiftlib.a", out_dir);

    // Compile Swift into a static library
    let status = Command::new("swiftc")
        .args(&[
            "-static",
            "-emit-library",
            "-framework",
            "MediaPlayer",
            "-framework",
            "AVFoundation",
            "-o",
            &lib_path,
            swift_file])
        .status()
        .expect("Failed to compile Swift code");

    if !status.success() {
        panic!("Swift compilation failed");
    }
    
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=swiftlib");

}
