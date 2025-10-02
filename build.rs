use std::env;
use std::fs::File;
use std::io::Write;
use std::panic;
use std::path::{Path, PathBuf};

fn main() {
    // Tell Cargo to rerun the build script if it changes.
    println!("cargo:rerun-if-changed=$CARGO_MANIFEST_DIR/scss");

    env::set_var("RUST_BACKTRACE", "1");

    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let cargo_manifest_dir = cargo_manifest_dir.as_str();

    // Install Bootstrap
    let bs_fetch_result = panic::catch_unwind(|| {
        // Download Bootstrap archive
        let mut bootstrap_zip = Vec::new();
        let mut curl_handle = curl::easy::Easy::new();
        curl_handle
            .url("https://github.com/twbs/bootstrap/archive/v5.3.0.zip")
            .unwrap();
        curl_handle.follow_location(true).unwrap();
        {
            let mut transfer = curl_handle.transfer();
            transfer
                .write_function(|new_data| {
                    bootstrap_zip.extend_from_slice(new_data);
                    Ok(new_data.len())
                })
                .unwrap();
            transfer.perform().unwrap();
        }
        let response_code = curl_handle.response_code().unwrap();
        let vec_len = bootstrap_zip.len();
        println!("response code: {response_code}, vec length: {vec_len}");

        // Extract Bootstrap archive
        let bootstrap_extract_target_dir: PathBuf =
            [cargo_manifest_dir, "bootstrap"].iter().collect();
        let mut archive =
            zip::ZipArchive::new(std::io::Cursor::new(bootstrap_zip.clone())).unwrap();
        archive.extract(&bootstrap_extract_target_dir).unwrap();

        // Copy Bootstrap JS files
        let bootstrap_js_filename = "bootstrap.min.js";
        let bootstrap_js_origin_path: PathBuf =
            ["dist", "js", bootstrap_js_filename].iter().collect();
        let bootstrap_js_origin_path = bootstrap_extract_target_dir.join(bootstrap_js_origin_path);

        let bootstrap_js_target_path: PathBuf =
            [cargo_manifest_dir, "public", "js", bootstrap_js_filename]
                .iter()
                .collect();

        // Create js path if it does not already exist
        create_dir_all(&bootstrap_js_target_path);
        std::fs::copy(bootstrap_js_origin_path, bootstrap_js_target_path).unwrap();
    });

    if let Err(err) = bs_fetch_result {
        println!("{:?}", err)
    }

    // Compile Sass
    {
        let grass_input_path: PathBuf = [cargo_manifest_dir, "scss", "main.scss"].iter().collect();

        let grass_output_path: PathBuf = [cargo_manifest_dir, "public", "style", "style.css"]
            .iter()
            .collect();

        // Create grass output path if it does not already exist
        create_dir_all(&grass_output_path);

        let mut grass_output_file = File::create(&grass_output_path).unwrap();

        // We want to compress the output CSS in release builds, but not in debug builds.
        let grass_output_style = cfg!(debug_assertions)
            .then(|| grass::OutputStyle::Expanded)
            .unwrap_or(grass::OutputStyle::Compressed);
        let grass_options = grass::Options::default().style(grass_output_style);
        grass_output_file
            .write_all(
                grass::from_path(grass_input_path, &grass_options)
                    .unwrap()
                    .as_bytes(),
            )
            .unwrap();
    }
}

fn create_dir_all(dir: &Path) {
    std::fs::create_dir_all(dir.parent().unwrap()).unwrap();
}
