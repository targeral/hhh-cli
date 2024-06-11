use std::collections::HashMap;
use std::env;

fn env(prefix: &str, env_vars: Option<HashMap<String, String>>) -> HashMap<String, serde_json::Value> {
    let env_vars = env_vars.unwrap_or_else(|| env::vars().collect());
    let mut obj = HashMap::new();
    let l = prefix.len();

    for (k, v) in env_vars.iter() {
        if k.to_lowercase().starts_with(&prefix.to_lowercase()) {
            let mut keypath: Vec<&str> = k[l..].split("__").collect();

            // Trim empty strings from keypath array
            keypath.retain(|&s| !s.is_empty());

            let mut cursor = &mut obj;
            for (i, subkey) in keypath.iter().enumerate() {
                if subkey.is_empty() || !cursor.is_object() {
                    break;
                }

                if i == keypath.len() - 1 {
                    cursor.insert(subkey.to_string(), serde_json::Value::String(v.clone()));
                } else {
                    cursor = cursor
                        .entry(subkey.to_string())
                        .or_insert_with(|| serde_json::Value::Object(HashMap::new()))
                        .as_object_mut()
                        .unwrap();
                }
            }
        }
    }

    obj
}

fn main() {
    // Example usage
    let prefix = "MYAPP_";
    let env_vars: HashMap<String, String> = [
        ("MYAPP_DB_HOST".to_string(), "localhost".to_string()),
        ("MYAPP_DB_PORT".to_string(), "5432".to_string()),
    ].iter().cloned().collect();

    let result = env(prefix, Some(env_vars));
    println!("{:?}", result);
}