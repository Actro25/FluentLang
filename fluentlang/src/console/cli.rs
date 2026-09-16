use crate::console::config_parsing::{InputJson, get_config_data, get_path, set_config_data};
use crate::console::error::AppErrors;
use clap::{Parser, Subcommand};

//This is base struct for cli.
//For example:
//fluentlang "This is my first sentence!"
//     ^                  ^
//This is key work        |
//                 This is an Option<String>
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    pub sentence: Option<String>,
    #[command(subcommand)]
    pub command: Option<MainCommands>,
}

//This is a subcommand struct for the base struct.
//For example: 
//fluentlang config set private "YOUR-PRIVATE-KEY"
//              ^
//          This is subcommand.
#[derive(Subcommand, Debug)]
pub enum MainCommands {
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
}

//This is also subcommand struct but for MainCommands struct
//For example:
//fluentlang config set private "YOUR-PRIVATE-KEY"
//                   ^
//This is subcomand for config. There's also get command but it only for showing all config data.
#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    Set {
        #[command(subcommand)]
        command: SetArguments,
    },
    Get,
}

//This is arguments struct for config data that is also subcommand but.
//For example:
//fluentlang config set private "YOUR-PRIVATE-KEY"
//                         ^
//This is a config parameters that can contain a value.
#[derive(Subcommand, Debug)]
pub enum SetArguments {
    Private { key_value: String },
    Public { key_value: String },
}

impl Cli {
    pub fn process_command(&self) -> Result<(), AppErrors> {
        //If the firs argument isn't a sentence then return CurrentlyUnavailable.
        if let Some(_) = self.sentence.as_ref() {
            return Err(AppErrors::CurrentlyUnavailable);
        }

        if let Some(main_command) = &self.command {
            match main_command {
                MainCommands::Config { command, .. } => match command {
                    ConfigCommands::Set { command, .. } => match command {
                        SetArguments::Private { key_value, .. } => {
                            //Creating input data for json setting
                            let mut input = InputJson::default();
                            //Input data that I want to save in config file
                            input.private = Some(key_value.clone());
                            //Call set function with path where we want to save config data.
                            set_config_data(&get_path("config.json")?, &input)?;
                        }
                        SetArguments::Public { key_value, .. } => {
                            let mut input = InputJson::default();
                            input.public = Some(key_value.clone());
                            set_config_data(&get_path("config.json")?, &input)?;
                        }
                    },
                    //It shows in console all config data.
                    ConfigCommands::Get => show_config_data()?,
                },
            }
        } else {
            println!("No command specified.")
        }

        Ok(())
    }
}

pub fn show_config_data() -> Result<(), AppErrors> {
    //Getting config data to show
    let output = get_config_data(&get_path("config.json")?)?;

    println!("Public key: {}", output.public);
    println!("Private key: {}", output.private);

    Ok(())
}