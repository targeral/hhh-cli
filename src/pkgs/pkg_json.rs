use std::{collections::HashMap, fmt::{Debug, Display}};

use crate::pkgs::registry_url::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use urlencoding::encode;
// use super::registry_url;

// port:
// - https://github.com/sindresorhus/type-fest/blob/main/source/literal-union.d.ts
// - https://github.com/sindresorhus/package-json/blob/main/index.d.ts#L641
#[derive(Serialize, Deserialize, Debug)]
pub struct PersonDetail {
    name: String,
    url: Option<String>,
    email: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Person {
    Name(String),
    Detail(PersonDetail)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageJsonStandard {
    // The name of the package.
    pub name: Option<String>,

    pub version: Option<String>,

    pub description: Option<String>,

    pub keywords: Option<Vec<String>>,

    pub homepage: Option<String>,

    pub author: Option<Person>,

    #[serde(rename = "devDependencies")]
    pub dev_dependencies: HashMap<String, String>,
    pub dependencies: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DistTags {
    #[serde(flatten)]
    map: HashMap<String, String>,
}

impl DistTags {
    fn new(latest: String) -> Self {
        let mut map = HashMap::new();
        map.insert("latest".to_string(), latest);
        DistTags { map }
    }

    fn insert_tag(&mut self, tag_name: String, version: String) {
        self.map.insert(tag_name, version);
    }

    fn get_tag(&self, tag_name: &str) -> Option<&String> {
        self.map.get(tag_name)
    }

    fn latest(&self) -> Option<&String> {
        self.map.get("latest")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AbbreviatedMetadata {
    pub name: String,
    pub modified: String,
    #[serde(rename = "dist-tags")]
    pub dist_tags: DistTags,
    pub versions: HashMap<String, AbbreviatedVersion>,
}

pub struct FullVersion {
    _id: String,
    _shasum: String,
    _from: String,
    _npm_version: String,
    _node_version: String,
    _npm_user: String,
    license_text: Option<String>,
    git_head: Option<String>,
}

pub enum PackageJsonReturn {
    FullVersion(FullVersion),
    OnlyAllVersions(AbbreviatedMetadata),
    Version(AbbreviatedVersion),
    None
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AbbreviatedVersion {
    pub name: Option<String>,
    pub version: String,
}

impl AbbreviatedVersion {
    fn create(&self) -> Self {
        AbbreviatedVersion {
            name: self.name.clone(),
            version: self.version.clone()
        }
    }
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

#[tokio::main]
pub async fn get_package_json_from_rc(package_name: &str, options: PackageJsonOptions) -> PackageJsonReturn {
    let version = options.version.unwrap_or("latest".to_string());
    let scope  = package_name.split("/").next().unwrap();
    let registry_url = match options.registry_url {
        Some(registry_url) => registry_url,
        None => registry_url(scope)
    };
    let parsed_registry_url = match registry_url.ends_with("/") {
        true => {
            let last_char_index = registry_url.len() - 1;
            registry_url[..last_char_index].to_string()
        },
        false => registry_url
    };

    let reg = Regex::new(r"^%40").unwrap();
    let encode_package_name = encode(package_name);
    let package_name = reg.replace(&encode_package_name, "@");

    let package_url = format!("{parsed_registry_url}/{package_name}");
    println!("package_url: {package_url}");

    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::ACCEPT,
        "application/vnd.npm.install-v1+json; q=1.0, application/json; q=0.8, */*".parse().unwrap()
    );

    let mut data = json!({});
    let response = client.get(&package_url).headers(headers).send().await;
    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                let resp_json: Value = resp.json().await.expect("resp.json fail");
                data = resp_json;
            } else if resp.status().as_u16() == 404 {
                eprintln!("{}", package_name);
            }
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
        }
    }

    let meta_data: AbbreviatedMetadata = serde_json::from_value(data).expect("类型转换失败");

    if options.all_versions {
        return PackageJsonReturn::OnlyAllVersions(meta_data);
    }

    if let Some(version) = meta_data.dist_tags.get_tag(&version) {
        let version_meta_data = meta_data.versions.get(version).unwrap();
        return PackageJsonReturn::Version(version_meta_data.create());
    }

    PackageJsonReturn::None
}