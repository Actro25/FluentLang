use crate::console::attributes::args::set::SetArg;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, ErrorKind};
use std::path::{Path, PathBuf};

pub struct Json {
    path: PathBuf,
}

impl Json {
    pub fn new() -> Self {
        let path = Self::get_path("config.json");
        Self { path }
    }

    pub fn set_config_data(&self, set_data: &SetArg) {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path);

        match file {
            Ok(file) => {
                let writer = BufWriter::new(file);
                match serde_json::to_writer_pretty(writer, &set_data) {
                    Err(err) => println!("Error serializing config: {}", err),
                    _ => {}
                };
            }
            Err(err) => match err.kind() {
                ErrorKind::PermissionDenied => {
                    eprintln!("There isn't enough permissions to write. Please try again.");
                }
                ErrorKind::NotFound => {
                    eprintln!("Path isn't found.");
                }
                _ => {
                    eprintln!("There is another IO error: {err}");
                }
            },
        }
    }

    //There should return Result
    pub fn get_config_data(&self) -> SetArg {
        let mut return_args = SetArg::new("".to_string(), "".to_string());
        if !self.path.exists() {
            println!("Config file does not exist.");
            return return_args;
        }

        match File::open(&self.path) {
            Ok(file) => {
                let reader = BufReader::new(file);

                match serde_json::from_reader::<_, SetArg>(reader) {
                    Ok(args) => return_args = args,
                    Err(err) => println!("Failed to parse JSON: {err}"),
                };
            }
            Err(err) => println!("Error opening config file: {}", err),
        }

        return_args
    }

    fn get_path(config: &str) -> PathBuf {
        let mut path = std::env::current_exe().expect("Can't get current directory");
        path.pop();
        path.push(config);
        path
    }
}
