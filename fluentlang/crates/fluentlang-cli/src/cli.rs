use clap::{Parser, Subcommand, ValueEnum};
use fluentlang_api::api::GroqAPI;
use fluentlang_core::config_parsing::{
    AvailableAiProviders, InputData, Key, Keys, OutputData, get_config_data, get_path,
    set_config_data,
};
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
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
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
        key: ConfigKey,
        value: String,
    },

    #[command(version, about = "", long_about = "")]
    Default {
        provider: Providers,
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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum ConfigKey {
    // Groq
    #[value(name = "groq.private_key")]
    GroqPrivateKey,

    // Google AI Studio
    #[value(name = "google.private_key")]
    GooglePrivateKey,
    #[value(name = "google.public_key")]
    GooglePublicKey,

    // OpenRouter
    #[value(name = "openrouter.private_key")]
    OpenRouterPrivateKey,
    #[value(name = "openrouter.public_key")]
    OpenRouterPublicKey,

    // Cerebras
    #[value(name = "cerebras.private_key")]
    CerebrasPrivateKey,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Providers {
    GroqCloud,
    GoogleAiStudio,
    OpenRouter,
    CerebrasInference,
}

#[derive(Subcommand, Debug)]
pub enum SetKey {
    #[command(
        version,
        about = "A private key parameter",
        long_about = r#"
This is arguments struct for config data that is also subcommand.
For example:
fluentlang config set API_PROVIDER private "YOUR-PRIVATE-KEY"
                                      ^
            This is a config parameters that contains a value."#
    )]
    Private { key_value: String },
}

#[derive(Subcommand, Debug)]
pub enum SetKeys {
    #[command(
        version,
        about = "A private key parameter",
        long_about = r#"
This is arguments struct for config data that is also subcommand.
For example:
fluentlang config set API_PROVIDER private "YOUR-PRIVATE-KEY"
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
fluentlang config set API_PROVIDER public "YOUR-PRIVATE-KEY"
                                      ^
            This is a config parameters that contains a value."#
    )]
    Public { key_value: String },
}

impl Cli {
    pub async fn process_command(self) -> Result<(), AppErrors> {
        match self {
            Cli::Sentence { sentence, .. } => cli_sentence_command(sentence).await?,
            Cli::Config { command, .. } => cli_config_command(command)?,
        }

        Ok(())
    }
}

async fn cli_sentence_command(sentence: String) -> Result<(), AppErrors> {
    //Firstly, we have to get what the current provider is.
    let config_data = match get_config_data(&get_path(CONFIG_NAME)?) {
        Ok(data) => data,
        Err(AppErrors::ConfigFileDoesntExist) => OutputData::default(),
        Err(err) => return Err(err),
    };

    //When we have config data we send request to the corresponding provider.
    match config_data.current_provider {
        AvailableAiProviders::GroqCloud => {
            GroqAPI::send_request(sentence, config_data.groq_cloud.private).await?;
        }
        AvailableAiProviders::GoogleAiStudio => {
            return Err(AppErrors::CurrentlyUnavailable);
        }
        AvailableAiProviders::OpenRouter => {
            return Err(AppErrors::CurrentlyUnavailable);
        }
        AvailableAiProviders::CerebrasInference => {
            return Err(AppErrors::CurrentlyUnavailable);
        }
        AvailableAiProviders::Unknow => return Err(AppErrors::UnknowProvider),
        AvailableAiProviders::ProviderIsNotChosen => {
            return Err(AppErrors::ProviderIsNotChosen);
        }
    };

    Ok(())
}

fn cli_config_command(command: ConfigCommands) -> Result<(), AppErrors> {
    match command {
        ConfigCommands::Set { key, value } => {
            let input = match key {
                ConfigKey::GroqPrivateKey => InputData::GroqCloud(Key::Private(value)),
                ConfigKey::GooglePrivateKey => InputData::GoogleAiStudio(Keys::Private(value)),
                ConfigKey::GooglePublicKey => InputData::GoogleAiStudio(Keys::Public(value)),
                ConfigKey::OpenRouterPrivateKey => InputData::OpenRouter(Keys::Private(value)),
                ConfigKey::OpenRouterPublicKey => InputData::OpenRouter(Keys::Public(value)),
                ConfigKey::CerebrasPrivateKey => InputData::CerebrasInference(Key::Private(value))
            };

            set_config_data(&get_path(CONFIG_NAME)?, input)?;
        }
        ConfigCommands::Get => show_config_data()?,
        ConfigCommands::Default { provider } => {
            let input = match provider {
                Providers::GroqCloud => InputData::CurrentProvider(AvailableAiProviders::GroqCloud),
                Providers::GoogleAiStudio => InputData::CurrentProvider(AvailableAiProviders::GoogleAiStudio),
                Providers::OpenRouter => InputData::CurrentProvider(AvailableAiProviders::OpenRouter),
                Providers::CerebrasInference => InputData::CurrentProvider(AvailableAiProviders::CerebrasInference),
            };

            set_config_data(&get_path(CONFIG_NAME)?, input)?;
        },
    };

    Ok(())
}

pub fn show_config_data() -> Result<(), AppErrors> {
    //Getting config data to show
    let output = get_config_data(&get_path(CONFIG_NAME)?)?;

    println!("=== Current Configuration ===");
    println!("Current Active Provider: {:?}\n", output.current_provider);

    println!("--- Groq Cloud ---");
    println!("  Private: {}", output.groq_cloud.private);

    println!("\n--- Google AI Studio ---");
    println!("  Public:  {}", output.google_ai_studio.public);
    println!("  Private: {}", output.google_ai_studio.private);

    println!("\n--- Open Router ---");
    println!("  Public:  {}", output.open_router.public);
    println!("  Private: {}", output.open_router.private);

    println!("\n--- Cerebras Inference ---");
    println!("  Private: {}", output.cerebras_inference.private);

    Ok(())
}