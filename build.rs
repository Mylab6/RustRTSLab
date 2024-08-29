use std::process::Command;
use std::env;

fn main() {
    // Get the target directory from the environment variables
    let out_dir = env::var("OUT_DIR").unwrap();

    // Build for Linux
    println!("Building for Linux...");
    let linux_build = Command::new("cargo")
        .args(&["build", "--release", "--target=x86_64-unknown-linux-gnu"])
        .status()
        .expect("Failed to build for Linux");

    if !linux_build.success() {
        panic!("Linux build failed!");
    }

    // Build for Android
    /* 
    println!("Building for Android...");
    let android_build = Command::new("cargo")
        .args(&[
            "ndk",
            "--target", "arm64-v8a",
            "--target", "armeabi-v7a",
            "--target", "x86_64",
            "--target", "x86",
            "-o", &format!("{}/android", out_dir),
            "build", "--release"
        ])
        .status()
        .expect("Failed to build for Android");

    if !android_build.success() {
        panic!("Android build failed!");
    }
    */

    println!("Build completed successfully.");
}
