use std::process::Command;
use std::env;

fn main() {
    // Tell Cargo to rerun the build script if the build script or scripts/install_bootstrapcss.sh files change.
    println!("cargo:rerun-if-changed=$CARGO_MANIFEST_DIR/scripts/install_bootstrapcss.sh");
    println!("cargo:rerun-if-changed=$CARGO_MANIFEST_DIR/build.rs");

    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    Command::new("sh")
        .arg("-C")
        .arg(format!("{}/scripts/install_bootstrapcss.sh", cargo_manifest_dir))
        .output()
        .expect("sh command failed to start");
}
