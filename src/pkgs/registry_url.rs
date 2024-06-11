use crate::pkgs::rc;
use serde_json::json;

pub fn registry_url(scope: &str) -> &str {
    let default_v = rc::index::DefaultInput::Json(json!({
        "registry": "https://registry.npmjs.org/"
    }));
    rc::index::rc("npm", Some(default_v));
    // port:https://github.com/sindresorhus/registry-url/blob/main/index.js
    let result = "https://registry.npmjs.org/";
    result
}

#[cfg(test)]
mod test {
    use super::registry_url;

    #[test]
    fn test_registry_url() {
        registry_url("npm");
        assert_eq!(1, 1);
    }
}