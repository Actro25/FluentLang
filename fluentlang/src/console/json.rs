use crate::console::attributes::args::set::SetArg;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct InputJson {
    pub keys: Option<SetArg>,
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct OutputJson {
    pub keys: SetArg,
}

pub struct Json {
    path: PathBuf,
}

impl Json {
    pub fn new() -> Self {
        let path = Self::get_path("config.json");
        Self { path }
    }

    pub fn set_config_data(&self, set_data: &InputJson) -> Result<(), String> {
        if !self.path.exists() {
            match self.create_config() {
                Ok(_) => {}
                Err(err) => return Err(err),
            };
        }

        let mut config_data = if let Ok(data) = self.get_config_data() {
            data
        } else {
            OutputJson::default()
        };

        if let Some(keys) = &set_data.keys {
            config_data.keys = keys.clone();
        }

        match File::create(&self.path) {
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

    pub fn get_config_data(&self) -> Result<OutputJson, String> {
        if !self.path.exists() {
            return Err("Config file does not exist.".to_string());
        }

        match File::open(&self.path) {
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

    fn create_config(&self) -> Result<(), String> {
        match File::create(&self.path) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }

    fn get_path(config: &str) -> PathBuf {
        let mut path = std::env::current_exe().expect("Can't get current directory");
        path.pop();
        path.push(config);
        path
    }
}
