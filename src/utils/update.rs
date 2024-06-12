
use crate::pkgs::pkg_json::*;

pub fn check_latest_version() {

    let pkg_json = get_package_json_from_rc("webpack", PackageJsonOptions {
        version: Some("latest".to_string()),
        all_versions: true,
        ..PackageJsonOptions::default()
    });
}