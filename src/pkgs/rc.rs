mod utils {
    use std::{fs, path::{Path, PathBuf}};

    use anyhow::Result;
    use json_comments::StripComments;
    use configparser::ini::Ini;
    use serde_json::{Value, json};
    use std::collections::HashMap;

    pub trait ToStrVec {
        fn to_str_vec(&self) -> Vec<&str>;
    }

    impl ToStrVec for Vec<&str> {
        fn to_str_vec(&self) -> Vec<&str> {
            // can return *self?
            self.clone()
        }
    }

    impl ToStrVec for Vec<Option<&str>> {
        fn to_str_vec(&self) -> Vec<&str> {
            self.iter().filter_map(|&arg| arg).collect()
        }
    }


    pub fn parse(content: &str) -> Result<Value> {
        let re = regex::Regex::new(r"^\s*\{")?;

        if re.is_match(content) {
            let stripped = StripComments::new(content.as_bytes());
            let json_value: Value = serde_json::from_reader(stripped)?;
            return Ok(json_value);
        }

        let mut config = Ini::new();
        config.read(String::from(content)).expect("read ini fail");
        let ini_map = config.get_map_ref();
        let mut json_value = serde_json::Map::new();

        for (section_name, properties) in ini_map {
            let mut section_map = serde_json::Map::new();
            for (k, v) in properties.iter() {
                // ??
                let v_default = String::from("");
                let v_ = v.as_ref().unwrap_or(&v_default);
                section_map.insert(k.to_string(), Value::String(v_.to_string()));
            }
            if section_name == "default" {
                json_value = section_map;
            } else {
                json_value.insert(
                section_name.to_string(),
                Value::Object(section_map)
                );
            }
        }

        return Ok(Value::Object(json_value));
    }

    fn path_join<T: ToStrVec>(args: T) -> PathBuf {
        let args: Vec<&str> = args.to_str_vec();

        let path = args.iter().fold(PathBuf::new(), |mut acc, arg| {
            acc.push(arg);
            acc
        });

        path
    }

    pub fn file<T: ToStrVec>(args: T) -> Option<String> {
        let path = path_join(args);

        match fs::read_to_string(&path) {
            Ok(content) => Some(content),
            Err(_) => None,
        }
    }

    pub fn json<T: ToStrVec>(args: T) -> Option<Value> {
        let result = match file(args) {
            Some(content) => parse(&content),
            None => return None
        };

        match result {
            Ok(json_content) => Some(json_content),
            Err(_) => None,
        }
    }

    pub fn env(prefix: &str, env_vars: Option<HashMap<String, String>>) -> Value {
        let env_vars = env_vars.unwrap_or_else(|| std::env::vars().collect());
        let mut obj: Value = json!({});
        
        for (k, v) in env_vars.iter() {
            let k_lower_case = k.to_lowercase();
            let prefix_lower_case = prefix.to_lowercase();
            if let Some(sub_k) = k_lower_case.strip_prefix(&prefix_lower_case) {
                let mut keypath: Vec<&str> = sub_k.split("__").collect();
                keypath.retain(|&k| !k.is_empty());

                let mut cursor = &mut obj;
                for (i, sub_key) in keypath.iter().enumerate() {
                    if sub_key.is_empty() || !cursor.is_object() {
                        continue;
                    }

                    if i == keypath.len() - 1 {
                        cursor[sub_key.to_string()]= Value::String(v.clone());
                    }

                    if cursor.get(sub_key.to_string()).is_none() {
                        cursor[sub_key.to_string()] = json!({});
                    }

                    cursor = cursor.get_mut(sub_key.to_string()).unwrap();
                }
            }
        }

        obj
    }

    pub fn find<T: ToStrVec>(args: T) -> Option<PathBuf> {
        fn find_(start: &Path, rel: &Path) -> Option<PathBuf> {
            let file = start.join(rel);
            
            if file.exists() {
                Some(file)
            } else {
                let parent = start.parent();
                match parent {
                    Some(parent) => find_(parent, rel),
                    _ => None
                }
            }
            // match fs::metadata(file) {
            //     Ok(_) => {
            //         let a = file.exists()
            //         return Some(String::from(file.to_str().clone().unwrap()));
            //     },
            //     Err(__) => {
            //         if let Some(dirname) = start.parent() {
            //             let dirname = dirname.to_path_buf();
            //             return find_(dirname, rel);
            //         } else {
            //             return None;
            //         }
            //     }
            // }
        }

        let rel = path_join(args);
        let cwd = std::env::current_dir().unwrap();
        find_(&cwd, &rel)
    }

}

pub mod index {
    use serde_json::{Value, json};
    use super::utils::{self as cc, json};
    use std::path::{Path};

    pub enum DefaultInput {
        String(String),
        Json(Value),
    }

    impl DefaultInput {
        fn to_value(self) -> Value {
            match self {
                DefaultInput::String(s) => serde_json::from_str(&s).expect("Failed to parse JSON string"),
                DefaultInput::Json(v) => v,
            }
        }
    }

    fn deep_clone_array(arr: &Value) -> Value {
        let mut clone = json!([]);
        let clone_arr = clone.as_array_mut().unwrap();
        if arr.is_array() {
            for (index, item) in arr.as_array().unwrap().iter().enumerate() {
                if item.is_object() && !item.is_null() {
                    if item.is_array() {
                        clone_arr.insert(index, deep_clone_array(item))
                    } else {
                        clone_arr.insert(index, deep_extend(vec![json!({}), item.clone()]))
                    }
                } else {
                    clone_arr.insert(index, item.clone());
                }
            }
        }

        clone
    }

    fn deep_extend(values: Vec<Value>) -> Value {
        let result = values.iter().fold(json!({}), |mut acc, x| {
            let acc_map = acc.as_object_mut().unwrap();
            if let Value::Object(o) = x {
                for (k, v) in o {
                    let src = acc_map.get(k);
                    let val = x.get(k).unwrap();
                    if !val.is_object() || val.is_null() {
                        acc_map.insert(k.clone(), val.clone());
                    } else if val.is_array() {
                        acc_map.insert(k.clone(), deep_clone_array(val));
                    } else if src.is_none() || src.is_some_and(|s| !s.is_object() || s.is_null() || s.is_array()) {
                        acc_map.insert(k.clone(), deep_extend(vec![json!({}), val.clone()]));
                    } else {
                        acc_map.insert(k.clone(), deep_extend(vec![src.unwrap().clone(), val.clone()]));
                    }
                }

            }
            acc
        });
        result
    }

    pub fn rc(name: &str, defaults: Option<DefaultInput>) -> Value {
        let defaults_: Value;
        if let Some(defaults) = defaults {
            defaults_ = defaults.to_value();
        } else {
            defaults_ = json!({});
        }
        let prefix = String::from("") + name + "_";
        let env = cc::env(&prefix, None);
        let mut configs = vec![defaults_];
        let mut config_files: Vec<String> = vec![];

        let mut add_config_file = |file: &str| {
            if config_files.iter().find(|&f| f == file) != None {
                return
            }

            if let Some(file_config) = cc::file(vec![file]) {
                if let Ok(config) = cc::parse(&file_config) {
                    configs.push(config);
                }
                config_files.push(String::from(file));
            }
        };

        let is_win = cfg!(target_os = "windows");

        if !is_win {
            let etc = Path::new("/etc");
            let p1 = etc.join(name).join("config");
            let p2 = etc.join(format!("{name}rc"));
            let paths = vec![p1, p2];
            paths.iter().for_each(|p| {
                if let Some(p) = p.to_str() {
                    add_config_file(p);
                }
            });
        }

        let env_vars = std::env::vars();
        let home = if is_win {
            std::env::var("USERPROFILE")
        } else {
            std::env::var("HOME")
        };

        if let Ok(home) = home {
            let home = Path::new(&home);
            vec![
                home.join(".config").join(name).join("config"),
                home.join(".config").join(name),
                home.join(format!(".{name}")).join("config"),
                home.join(format!(".{name}rc")),
            ].iter().for_each(|p| {
                if let Some(p) = p.to_str() {
                    add_config_file(p);
                }
            });
        }

        let cwd_rc_str = String::from(".") + name + "rc";
        let cwd_rc_file = cwd_rc_str.as_str();
        if let Some(cwd_rc_file) = cc::find(vec![cwd_rc_file]) {
            add_config_file(cwd_rc_file.to_str().unwrap());
        }

        if let Some(config) = env.get("config") {
            add_config_file(config.to_string().as_str())
        }

        configs.push(env);
        if config_files.len() > 0 {
            configs.push(json!({
                "configs": config_files,
                "config": config_files[config_files.len() - 1]
            }));
        }

        deep_extend(configs)
    }


    #[cfg(test)]
    mod tests {
        use serde_json::json;
        use crate::pkgs::rc::utils::json;

        use super::deep_extend;

        #[test]
        fn test_deep_extend() -> Result<(), String> {
            let v1 = json!({"a": "1", "b": {"b_1": "b_1"}, "c": ["c_1", "c_2"], "d": 1});
            let v2 = json!({"a": "2"});
            let v3 = json!({"b": {"b_1": "B_1", "b_2": "B_2"}});
            let v4 = json!({"c": ["c_3"]});
            let result = deep_extend(vec![v1, v2, v3, v4]);
            let expect = json!({
                "a": "2",
                "b": {"b_1": "B_1", "b_2": "B_2"},
                "c": ["c_3"],
                "d": 1
            });
            if result == expect {
                Ok(())
            } else {
                println!("{:#}", result);
                Err(format!("{:#}", result))
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use super::index;
    use serde_json::json;
    use std::{collections::HashMap, path::Path};


    #[test]
    fn parse_json_file() -> Result<(), String> {
        let json_content = r#"
        {
            "key": "value",
            "number": 42,
            "array": [1, 2, 3]
        }
        "#;
        let result = utils::parse(json_content);
        let parsed_value = match result {
            Ok(parsed_value) => parsed_value,
            Err(e) => return Err(format!("parse fail: {:?}", e))
        };
        let expected = json!({
            "key": "value",
            "number": 42,
            "array": [1, 2, 3]
        });
        
        if parsed_value == expected {
            Ok(())
        } else {
            Err(String::from("parsed value is wrong"))
        }
    }

    #[test]
    fn parse_ini_file() -> Result<(), String> {
        let ini_content = r#"
        //registry.npmjs.org/:_authToken=xxx
        access=public
        "#;
        let result = utils::parse(ini_content);
        let parsed_value = match result {
            Ok(parsed_value) => parsed_value,
            Err(e) => return Err(format!("parse fail: {:?}", e))
        };
        println!("parsed_value: {parsed_value}");

        let expected = json!({
            "//registry.npmjs.org/": "_authToken=xxx",
            "access": "public"
        });
        
        if expected == parsed_value {
            Ok(())
        } else {
            Err(String::from("parsed value is wrong"))
        }
    }

    #[test]
    fn file() -> Result<(), String> {
        // 获取当前文件目录
        // file! 是一个编译时宏，它会返回当前文件的路径（相对于项目根目录）。
        let current_file_path = Path::new(file!());
        let cwd = current_file_path.parent().unwrap().to_str().unwrap();

        let content = match utils::file(vec![cwd, "rc.rs"]) {
            Some(content) => content,
            None => return Err(String::from("can`t read file")),
        };

        println!("content: {content}");

        Ok(())
    }

    #[test]
    fn json() -> Result<(), String> {
        let cwd_path_buf = std::env::current_dir().unwrap();
        let cwd = cwd_path_buf.to_str().unwrap();
        match utils::json(vec![cwd, "fixtures", "rc.json"]) {
            Some(json_value) => {
                println!("json_value: {}", json_value.to_string());
                Ok(())
            },
            None => Err(format!("json function fail"))
        }
    }

    #[test]
    fn env() -> Result<(), String> {
        let n = String::from("rc");
        let env_str = String::from("_someOpt__a__b__c");
        let env_key = String::from("") + &n + &env_str;
        let mut env_vars_map: HashMap<String, String> = HashMap::new();
        env_vars_map.insert(env_key, String::from("243"));
        let env_vars: Option<HashMap<String, String>> = Some(env_vars_map);

        let prefix = String::from("") + &n + "_";

        let result = utils::env(&prefix, env_vars);
        let expect_json = json!({
            "someopt": {
                "a": {
                    "b": {
                        "c": "243"
                    }
                }
            }
        });

        if result == expect_json {
            Ok(())
        } else {
            Err(format!("fail: result is {:?}\n, expect is {:?}", result.to_string(), expect_json.to_string()))
        }
    }

    #[test]
    fn find() -> Result<(), String> {
        let args = vec![".npmrc"];
        match utils::find(args) {
            Some(p) => {
                println!(".npmrc in: {:?}", p);
                Ok(())
            },
            None => {
                Err(format!("find fail"))
            }
        }
    }

}