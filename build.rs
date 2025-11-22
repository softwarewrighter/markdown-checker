use chrono::{Datelike, Utc};
use std::fs;
use std::process::Command;

fn main() {
    // Read version from VERSION file
    let version = fs::read_to_string("VERSION")
        .expect("Failed to read VERSION file")
        .trim()
        .to_string();
    println!("cargo:rustc-env=APP_VERSION={}", version);

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
    println!("cargo:rustc-env=GIT_COMMIT_SHA={}", git_sha);

    // Get build timestamp (milliseconds since epoch) for sw-cli
    let timestamp_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;
    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", timestamp_ms);

    // Get current year for copyright
    let year = Utc::now().year();
    println!("cargo:rustc-env=BUILD_YEAR={}", year);

    // Tell cargo to re-run this script if these files change
    println!("cargo:rerun-if-changed=VERSION");
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/index");
}
