use crate::pkgs::registry_url::*;

pub struct AbbreviatedVersion {
    name: Option<String>,
}

pub struct PackageJsonOptions {
    pub version: Option<String>,
    pub full_metadata: bool,
    pub all_versions: bool,
    pub registry_url: Option<String>,
    pub omit_deprecated: bool,
}

impl Default for PackageJsonOptions {
    fn default() -> Self {
        PackageJsonOptions {
            version: Some("latest".to_string()),
            full_metadata: false,
            all_versions: false,
            registry_url: None,
            omit_deprecated: false
        }
    }
}

pub fn get_package_json_from_rc(package_name: &str, options: PackageJsonOptions) -> AbbreviatedVersion {
    let version = options.version.unwrap_or("latest".to_string());
    let scope  = package_name.split("/").next().unwrap();
    let registry_url = match options.registry_url {
        Some(registry_url) => registry_url,
        None => registry_url(scope)
    };
    println!("scope: {scope}, registry_url: {registry_url}");

    AbbreviatedVersion {
        name: Some("test".to_string())
    }
}