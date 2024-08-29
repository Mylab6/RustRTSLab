use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Initialize logging
    println!("cargo:rerun-if-changed=build.rs");
    println!("Starting the build process...");

    // Print out some environment variables
    println!("cargo:warning=BUILD_PROFILE: {:?}", env::var("PROFILE").unwrap_or_else(|_| "unknown".into()));
    println!("cargo:warning=OUT_DIR: {:?}", env::var("OUT_DIR").unwrap_or_else(|_| "unknown".into()));
    println!("cargo:warning=TARGET: {:?}", env::var("TARGET").unwrap_or_else(|_| "unknown".into()));

    // Example of checking for dependencies
    check_dependency("cargo");

    // Generate some files or include some assets
    generate_assets();

    // Final message
    println!("cargo:warning=Build process completed.");
}

fn check_dependency(dep: &str) {
    println!("cargo:warning=Checking for dependency: {}", dep);
    match Command::new(dep).output() {
        Ok(output) => {
            if output.status.success() {
                println!("cargo:warning=Dependency `{}` found.", dep);
            } else {
                println!("cargo:warning=Dependency `{}` not found!", dep);
            }
        }
        Err(e) => {
            println!("cargo:warning=Failed to execute command `{}`: {:?}", dep, e);
        }
    }
}

fn generate_assets() {
    println!("cargo:warning=Generating assets...");

    // Example of generating a file
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let dest_path = PathBuf::from(out_dir).join("generated_asset.txt");
    if let Err(e) = fs::write(&dest_path, "This is a generated asset.") {
        println!("cargo:warning=Failed to write generated asset: {:?}", e);
    } else {
        println!("cargo:warning=Generated asset saved to {:?}", dest_path);
    }

    println!("cargo:rerun-if-changed=assets/");
    println!("cargo:warning=Asset generation completed.");
}
