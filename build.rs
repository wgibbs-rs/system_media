use std::env;
use std::process::Command;

#[cfg(target_os = "macos")]
fn main() {
    let swift_file = "src/macos/nowplaying.swift";
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
            "-framework",
            "AppKit",
            "-framework",
            "Cocoa",
            "-o",
            &lib_path,
            swift_file,
        ])
        .status()
        .expect("Failed to compile Swift code");

    if !status.success() {
        panic!("Swift compilation failed");
    }

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=swiftlib");
}
