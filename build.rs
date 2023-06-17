use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // Tell Cargo to rerun the build script if it changes.
    println!("cargo:rerun-if-changed=$CARGO_MANIFEST_DIR/build.rs");

    env::set_var("RUST_BACKTRACE", "1");

    // Install Bootstrap
    {
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
        let target_dir: PathBuf = [
            env::var("CARGO_MANIFEST_DIR").unwrap().as_str(),
            "bootstrap",
        ]
        .iter()
        .collect();
        zip_extract::extract(std::io::Cursor::new(bootstrap_zip), &target_dir, true).unwrap();
    }

    // Compile Sass
    {
        let grass_input_path: PathBuf = [
            env::var("CARGO_MANIFEST_DIR").unwrap().as_str(),
            "scss",
            "main.scss",
        ]
        .iter()
        .collect();

        let grass_output_path: PathBuf = [
            env::var("CARGO_MANIFEST_DIR").unwrap().as_str(),
            "style",
            "style.css",
        ]
        .iter()
        .collect();

        // Create grass output path if it does not already exist
        let grass_output_dir = grass_output_path.parent().unwrap();
        std::fs::create_dir_all(grass_output_dir).unwrap();

        let mut grass_output_file = File::create(&grass_output_path).unwrap();

        // We want to compress the output CSS in release builds, but not in debug builds.
        let grass_output_style = cfg!(debug_assertions)
            .then(|| grass::OutputStyle::Expanded)
            .unwrap_or(grass::OutputStyle::Compressed);
        let grass_options = grass::Options::default().style(grass_output_style);
        grass_output_file
            .write_all(
                grass::from_path(&grass_input_path, &grass_options)
                    .unwrap()
                    .as_bytes(),
            )
            .unwrap();
    }
}
