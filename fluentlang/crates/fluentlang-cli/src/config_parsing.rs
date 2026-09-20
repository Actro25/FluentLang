use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use fluentlang_core::error::AppErrors;

#[derive(Debug)]
pub enum InputData {
    Private(String),
    Public(String)
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct OutputData{
    pub public: String,
    pub private: String,
}

///This function set data into JSON config by path.
///Initially, this function takes existed data (OutputData) in JSON file to not lost it.
///Then the function set data into the struct (OutputData) by InputData struct.
///This function also creates config.json
pub fn set_config_data(path: impl AsRef<Path>, set_data: InputData) -> Result<(), AppErrors> {
    let path = path.as_ref();
    /*
     If the get_config_data return Ok than return the data that was returned.
     If the function returns that ConfigFileDoesntExist it means that config should be created because
     it could be the first user's launch of the program.
     If there are any others problems it returns the errors upward.
     */
    let mut config_data = match get_config_data(path) {
        Ok(data) => data,
        Err(AppErrors::ConfigFileDoesntExist) => OutputData::default(),
        Err(err) => return Err(err)
    };

    //If the parameter from user was entered then clone it in setting struct.
    match set_data {
        InputData::Private(key) => config_data.public = key,
        InputData::Public(key) => config_data.private = key,
    }

    //If the config dir isn't exist then we create it
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    //There we create file if it doesn't exist.
    let file = File::create(path)?;

    //Creating writer, because to_writer_pretty is supposed to have writer as parameter.
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &config_data)?;

    Ok(())
}

///This function will return Error::ConfigFileDoesntExist if path returns false from function .exists().
///If path exists it will return OutputData struct if the path exists and there isn't any io error.
pub fn get_config_data(path: impl AsRef<Path>,) -> Result<OutputData, AppErrors> {
    let path = path.as_ref();

    if !path.exists() {
        return Err(AppErrors::ConfigFileDoesntExist);
    }

    match File::open(path) {
        Ok(file) => {
            let reader = BufReader::new(file);

            match serde_json::from_reader::<_, OutputData>(reader) {
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