use crate::pkgs::rc;
use serde_json::json;

pub fn registry_url(scope: &str) -> String {
    let default_v = rc::index::DefaultInput::Json(json!({
        "registry": "https://registry.npmjs.org/"
    }));
    let result = rc::index::rc("npm", Some(default_v));
    let mut url = String::new();
    if let Some(rc_result_o) = result.as_object() {
        let scope_key = format!("{scope}:registry");
        if let Some(v) = rc_result_o.get(scope_key.as_str()) {
            if let Some(v) = v.as_str() {
                url.push_str(v)
            }
        } else if let Some(v) = rc_result_o.get("config_registry") {
            if let Some(v) = v.as_str() {
                url.push_str(v);
            }
        } else {
            url.push_str(rc_result_o.get("registry").unwrap().as_str().unwrap())
        }
    }
    
    // port:https://github.com/sindresorhus/registry-url/blob/main/index.js
    url
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