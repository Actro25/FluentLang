use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum AppErrors {
    CurrentlyUnavailable,
    ConfigFileDoesntExist,
    ConfigJsonProblem(serde_json::Error),
    ConfigIOProblem(std::io::Error),
    CantFindPathToConfigFile,
}

impl std::error::Error for AppErrors {}

impl From<std::io::Error> for AppErrors {
    fn from(value: std::io::Error) -> Self {
        AppErrors::ConfigIOProblem(value)
    }
}

impl From<serde_json::Error> for AppErrors {
    fn from(value: serde_json::Error) -> Self {
        AppErrors::ConfigJsonProblem(value)
    }
}

impl Display for AppErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            AppErrors::CurrentlyUnavailable => write!(f, "This feature is current unavailable. Please wait till we complete this feature."),
            AppErrors::ConfigFileDoesntExist => write!(f, "The config file doesn't exist. Please set you config data first."),
            AppErrors::ConfigJsonProblem(err) => write!(f, "There is json problem with the config file: {}", err),
            AppErrors::ConfigIOProblem(err) => write!(f, "There is io problems with the config file: {}", err),
            AppErrors::CantFindPathToConfigFile => write!(f, "Can't find path to the config file."),
        }
    }
}