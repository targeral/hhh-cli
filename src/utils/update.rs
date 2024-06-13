
use crate::pkgs::pkg_json;
use indicatif::{ProgressBar, ProgressStyle};
use std::{thread::sleep, time::Duration};

pub fn check_latest_version() -> Result<(), String> {
    let plugin_name = env!("CLI_NAME");
    let current_version = env!("CLI_VERSION");

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} Checking '{msg}' version...")
            .expect("Failed to set template")
            .tick_chars("/|\\- ")
    );
    spinner.set_message(plugin_name);

    spinner.enable_steady_tick(Duration::from_millis(100));
    let latest_pkg_json_metadata = pkg_json::get_package_json_from_rc("package-json", pkg_json::PackageJsonOptions {
        version: Some("latest".to_string()),
        ..pkg_json::PackageJsonOptions::default()
    });

    let latest_version = match latest_pkg_json_metadata {
        pkg_json::PackageJsonReturn::Version(latest_pkg_json_metadata) => {
            spinner.finish_and_clear();
            latest_pkg_json_metadata.version
        },
        _ => {
            spinner.finish_and_clear();
            return Err(format!(r#"Failed to get "latest" version"#));
        }
    };

    println!("{current_version}, {latest_version}");

    Ok(())
}