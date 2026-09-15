use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct InputJson {
    pub public: Option<String>,
    pub private: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct OutputJson {
    pub public: String,
    pub private: String,
}

pub fn set_config_data(path: &PathBuf, set_data: &InputJson) -> Result<(), String> {
    let mut config_data = if let Ok(data) = get_config_data(path) {
        data
    } else {
        OutputJson::default()
    };

    if let Some(key) = &set_data.public {
        config_data.public = key.clone();
    }
    if let Some(key) = &set_data.private {
        config_data.private = key.clone();
    }

    if let Some(parent) = path.parent() {
        match fs::create_dir_all(parent) {
            Ok(_) => {}
            Err(err) => return Err(err.to_string()),
        }
    }

    match File::create(path) {
        Ok(file) => {
            let writer = BufWriter::new(file);

            match serde_json::to_writer_pretty(writer, &config_data) {
                Err(err) => Err(err.to_string()),
                _ => Ok(()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

pub fn get_config_data(path: &PathBuf) -> Result<OutputJson, String> {
    if !path.exists() {
        return Err("Config file does not exist.".into());
    }

    match File::open(path) {
        Ok(file) => {
            let reader = BufReader::new(file);

            match serde_json::from_reader::<_, OutputJson>(reader) {
                Ok(args) => Ok(args),
                Err(err) => Err(format!("Failed to parse JSON: {}", err)),
            }
        }
        Err(err) => Err(format!("Error opening config file: {}", err)),
    }
}

pub fn get_path(config_name: &str) -> Result<PathBuf, String> {
    let path = dirs::config_dir();
    if let Some(mut path) = path {
        path.push("FluentLang");
        path.push(config_name);
        Ok(path)
    } else {
        Err("Can't find path for config folder.".into())
    }
}