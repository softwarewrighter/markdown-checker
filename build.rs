use chrono::Datelike;
use sw_cli::define_build_info;

fn main() {
    // Use sw-cli macro to define standard build info environment variables
    // Sets: BUILD_HOST, GIT_COMMIT_SHA, BUILD_TIMESTAMP
    define_build_info!();

    // Read version from VERSION file (project-specific)
    let version = std::fs::read_to_string("VERSION")
        .expect("Failed to read VERSION file")
        .trim()
        .to_string();
    println!("cargo:rustc-env=APP_VERSION={}", version);

    // Get current year for copyright (project-specific)
    let year = chrono::Utc::now().year();
    println!("cargo:rustc-env=BUILD_YEAR={}", year);

    // Tell cargo to re-run this script if these files change
    println!("cargo:rerun-if-changed=VERSION");
}
