use crate::pkgs::registry_url::*;
use regex::Regex;
use serde_json::{json, Value};
use urlencoding::encode;

// port:
// - https://github.com/sindresorhus/type-fest/blob/main/source/literal-union.d.ts
// - https://github.com/sindresorhus/package-json/blob/main/index.d.ts#L641
struct PersonDetail {
    name: String,
    url: Option<String>,
    email: Option<String>
}

enum Person {
    Name(String),
    Detail(PersonDetail)
}

pub struct PackageJson {
    // The name of the package.
    name: Option<String>,

    version: Option<String>,

    description: Option<String>,

    keywords: Option<Vec<String>>,

    // homepage: Option<>

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
}

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

#[tokio::main]
pub async fn get_package_json_from_rc(package_name: &str, options: PackageJsonOptions) -> AbbreviatedVersion {
    let version = options.version.unwrap_or("latest".to_string());
    let scope  = package_name.split("/").next().unwrap();
    let registry_url = match options.registry_url {
        Some(registry_url) => registry_url,
        None => registry_url(scope)
    };

    let reg = Regex::new(r"^%40").unwrap();
    let encode_package_name = encode(package_name);
    let package_name = reg.replace(&encode_package_name, "@");

    let package_url = format!("{registry_url}/{package_name}");
    
    println!("package_url: {package_url}");
    println!("version: {:?}", version);
    println!("scope: {scope}, registry_url: {registry_url}");

    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::ACCEPT,
        "application/vnd.npm.install-v1+json; q=1.0, application/json; q=0.8, */*".parse().unwrap()
    );

    let response = client.get(&package_url).headers(headers).send().await;

    let mut data = json!({});
    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                let resp_json: Value = resp.json().await.expect("resp.json fail");
                data = resp_json;
                // let dist_tag = data.as_object().unwrap().get("dist-tags").expect("no dist-tags").get(&version).expect(format!("no {:?}", version).as_str());
                // println!("{:?}", dist_tag);
            } else if resp.status().as_u16() == 404 {
                eprintln!("{}", package_name);
            }
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
        }
    }

    if options.all_versions {
        println!("{:#}", data);
        
    }
    AbbreviatedVersion {
        name: Some("test".to_string())
    }
}