use chrono::Utc;
use std::fs;
use std::process::Command;

fn main() {
    // Read version from VERSION file
    let version = fs::read_to_string("VERSION")
        .expect("Failed to read VERSION file")
        .trim()
        .to_string();
    println!("cargo:rustc-env=APP_VERSION={}", version);

    // Get the current timestamp in ISO 8601 format
    let build_timestamp = Utc::now().to_rfc3339();
    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", build_timestamp);

    // Get the build host (hostname)
    let build_host = if let Ok(output) = Command::new("hostname").output() {
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    } else {
        "unknown".to_string()
    };
    println!("cargo:rustc-env=BUILD_HOST={}", build_host);

    // Get the git commit SHA
    let git_sha = if let Ok(output) = Command::new("git").args(["rev-parse", "HEAD"]).output() {
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    } else {
        "unknown".to_string()
    };
    println!("cargo:rustc-env=GIT_SHA={}", git_sha);

    // Get the short git commit SHA (first 7 characters)
    let git_sha_short = if git_sha.len() >= 7 {
        git_sha[..7].to_string()
    } else {
        git_sha.clone()
    };
    println!("cargo:rustc-env=GIT_SHA_SHORT={}", git_sha_short);

    // Tell cargo to re-run this script if these files change
    println!("cargo:rerun-if-changed=VERSION");
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/index");
}
