use std::process::Command;
use std::env;

// Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=$CARGO_MANIFEST_DIR/scripts/install_bootstrapcss.sh");
    println!("cargo:rerun-if-changed=$CARGO_MANIFEST_DIR/build.rs");
    println!("cargo:rerun-if-changed=$CARGO_MANIFEST_DIR/Cargo.lock");

    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    Command::new("sh")
        .arg("-C")
        .arg(format!("{}/scripts/install_bootstrapcss.sh", cargo_manifest_dir))
        .output()
        .expect("sh command failed to start");
}
