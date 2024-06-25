use std::default;
use std::io::{self, BufReader};
use std::path::Path;
use std::fs::{self, File};

use serde::Deserialize;

use super::constants::HHH_FILE_NAME;

#[derive(Deserialize)]
pub struct HHHConfig {
    name: Option<String>,
}

impl Default for HHHConfig {
    fn default() -> Self {
        HHHConfig {
            name: None,
        }
    }
}

pub fn check_config_file_if_exist(cwd: &Path) -> Result<bool, io::Error> {
    let hhh_config_path =  cwd.join(HHH_FILE_NAME);
    let result = fs::metadata(hhh_config_path)?.is_file();
    Ok(result)
}

pub fn read_hhh_config(cwd: &Path) -> Result<HHHConfig, io::Error> {
    let hhh_path = cwd.join(HHH_FILE_NAME);
    let file = File::open(hhh_path)?;
    let reader = BufReader::new(file);
    let result = serde_json::from_reader::<_, HHHConfig>(reader)?;
    Ok(result)
}
