use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use crate::error::AppErrors;

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

pub fn set_config_data(path: &PathBuf, set_data: &InputJson) -> Result<(), AppErrors> {
    //If get_config_data return Ok than return the data that was returned else return error.
    let mut config_data = get_config_data(path)?;

    //If the parameter from user was entered then clon it in setting struct.
    if let Some(key) = &set_data.public {
        config_data.public = key.clone();
    }
    if let Some(key) = &set_data.private {
        config_data.private = key.clone();
    }

    //If the config dir isn't exist then we create it
    if let Some(parent) = path.parent() {
        match fs::create_dir_all(parent) {
            Ok(_) => {}
            Err(err) => return Err(AppErrors::ConfigIOProblem(err)),
        }
    }

    //There we create file if it doesn't exist.
    match File::create(path) {
        Ok(file) => {
            //Creating writer, because to_writer_pretty is supposed to have writer as parameter.
            let writer = BufWriter::new(file);

            match serde_json::to_writer_pretty(writer, &config_data) {
                Err(err) => Err(AppErrors::ConfigJsonProblem(err)),
                _ => Ok(()),
            }
        }
        Err(err) => Err(AppErrors::ConfigIOProblem(err)),
    }
}

pub fn get_config_data(path: &PathBuf) -> Result<OutputJson, AppErrors> {
    if !path.exists() {
        return Err(AppErrors::ConfigFileDoesntExist);
    }

    match File::open(path) {
        Ok(file) => {
            let reader = BufReader::new(file);

            match serde_json::from_reader::<_, OutputJson>(reader) {
                Ok(args) => Ok(args),
                Err(err) => Err(AppErrors::ConfigJsonProblem(err)),
            }
        }
        Err(err) => Err(AppErrors::ConfigIOProblem(err)),
    }
}

pub fn get_path(config_name: &str) -> Result<PathBuf, AppErrors> {
    //Getting path from dits library.
    let path = dirs::config_dir();
    //If path exists then return path as FluentLang/config_name then return error.
    if let Some(mut path) = path {
        path.push("FluentLang");
        path.push(config_name);
        Ok(path)
    } else {
        Err(AppErrors::CantFindPathToConfigFile)
    }
}