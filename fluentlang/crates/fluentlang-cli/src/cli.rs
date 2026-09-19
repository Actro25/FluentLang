use clap::{Parser, Subcommand};
use fluentlang_core::config_parsing::{get_config_data, get_path, set_config_data, InputData};
use fluentlang_core::error::AppErrors;

const CONFIG_NAME: &str = "config.json";

#[derive(Parser, Debug)]
#[command(
    name = "fluentlang",
    version,
    long_about = r#"
This is base struct for cli.
For example:
fluentlang "This is my first sentence!"
    ^                  ^
This is key word       |
           This is a sentence that you want to understand"#
)]
pub enum Cli {
    #[command(
        version,
        about = "An attribute that returns explained sentence",
        long_about = r#"
This is subcommand for key work "fluentlang"
For example:
fluentlang sentence "This is my first sentence!"
               ^        ^
This is subcommand      |
           This is a sentence that you want to understand"#
    )]
    Sentence {
        #[arg(value_name = "SENTENCE")]
        sentence: String,
    },
    #[command(
        version,
        about = "An attribute that helps to set up the config file",
        long_about = r#"
This is subcommand for key work "fluentlang"
For example:
fluentlang config set public "PUBLIC-KEY"
               ^   ^
This is subcommand |
           This is also a subcommand but for "config""#
    )]
    Config{
        #[command(subcommand)]
        command: ConfigCommands,
    }
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    #[command(
        version,
        about = "An attribute that setting up the config file by values",
        long_about = r#"
This is also subcommand struct but for MainCommands struct
For example:
fluentlang config set private "YOUR-PRIVATE-KEY"
                  ^
This is subcommand for config. This will help you to set data by parameters."#
    )]
    Set {
        #[command(subcommand)]
        command: SetArguments,
    },

    #[command(
        version,
        about = "An attribute that gets and shows all the config data",
        long_about = r#"
This is also subcommand struct but for MainCommands struct
For example:
fluentlang config get
                  ^
This is subcommand for config. It'll show config data."#
    )]
    Get,
}

#[derive(Subcommand, Debug)]
pub enum SetArguments {
    #[command(
        version,
        about = "A private key parameter",
        long_about = r#"
This is arguments struct for config data that is also subcommand.
For example:
fluentlang config set private "YOUR-PRIVATE-KEY"
                        ^
This is a config parameters that contains a value."#
    )]
    Private { key_value: String },

    #[command(
        version,
        about = "A public key parameter",
        long_about = r#"
This is arguments struct for config data that is also subcommand.
For example:
fluentlang config set public "YOUR-PRIVATE-KEY"
                        ^
This is a config parameters that contains a value."#
    )]
    Public { key_value: String },
}

impl Cli {
    pub fn process_command(&self) -> Result<(), AppErrors> {
        //If the firs argument isn't a sentence then return CurrentlyUnavailable.
        match &self {
            Cli::Sentence { .. } => return Err(AppErrors::CurrentlyUnavailable),
            Cli::Config { command, .. } => match command {
                ConfigCommands::Set { command, .. } => match command {
                    SetArguments::Private { key_value, .. } => {
                        //Creating input data for the JSON setting
                        //Input data that I want to save in config file
                        let input = InputData::Private(key_value);
                        //Call set function with path where we want to save config data.
                        set_config_data(&get_path(CONFIG_NAME)?, &input)?;
                    }
                    SetArguments::Public { key_value, .. } => {
                        let input = InputData::Public(key_value);
                        set_config_data(&get_path(CONFIG_NAME)?, &input)?;
                    }
                },
                //It shows in console all config data.
                ConfigCommands::Get => show_config_data()?,
            }
        };

        Ok(())
    }
}

pub fn show_config_data() -> Result<(), AppErrors> {
    //Getting config data to show
    let output = get_config_data(&get_path(CONFIG_NAME)?)?;

    println!("Public key: {}", output.public);
    println!("Private key: {}", output.private);

    Ok(())
}