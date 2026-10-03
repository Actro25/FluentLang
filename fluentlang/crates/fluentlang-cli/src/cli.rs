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
    about = "CLI tool to analyze and explain sentences using AI providers",
    long_about = r#"A command-line application that connects to AI providers (Groq, Google AI Studio, OpenRouter, Cerebras) to generate detailed explanations of sentences.

Examples:
  fluentlang sentence "Break down this sentence for me."
  fluentlang config default groq-cloud
  fluentlang config set groq.private_key "gsk_..."
"#
)]
pub enum Cli {
    #[command(
        version,
        about = "Explain and break down a given sentence",
        long_about = r#"Sends a prompt to the currently configured AI provider to receive a detailed breakdown and explanation of the input text.

Example:
  fluentlang sentence "The quick brown fox jumps over the lazy dog."
"#
    )]
    Sentence {
        #[arg(value_name = "SENTENCE")]
        sentence: String,
    },

    #[command(
        version,
        about = "Manage configuration settings and API credentials",
        long_about = r#"Configure default AI providers, manage private/public API keys, and view stored application settings.

Examples:
  fluentlang config
  fluentlang config default groq-cloud
  fluentlang config set google.api_key "AIzaSy..."
"#
    )]
    Config {
        #[command(subcommand)]
        command: Option<ConfigCommands>,
    },
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    #[command(
        version,
        about = "Set an API key or config property",
        long_about = r#"Sets a configuration entry such as an API key for a specific AI provider.

Examples:
  fluentlang config set groq.private_key "gsk_..."
  fluentlang config set openrouter.public_key "pk_..."
"#
    )]
    Set { key: ConfigKey, value: String },

    #[command(
        version,
        about = "Set the default active AI provider",
        long_about = r#"Switches the primary AI provider used for analyzing sentences.

Example:
  fluentlang config default groq-cloud
"#
    )]
    Default { provider: Providers },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum ConfigKey {
    #[value(name = "groq.private_key")]
    GroqPrivateKey,

    #[value(name = "google.private_key")]
    GooglePrivateKey,
    #[value(name = "google.public_key")]
    GooglePublicKey,

    #[value(name = "openrouter.private_key")]
    OpenRouterPrivateKey,
    #[value(name = "openrouter.public_key")]
    OpenRouterPublicKey,

    #[value(name = "cerebras.private_key")]
    CerebrasPrivateKey,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Providers {
    #[value(alias = "groq")]
    GroqCloud,

    #[value(alias = "google-ai")]
    GoogleAiStudio,

    OpenRouter,

    CerebrasInference,
}

impl Cli {
    pub async fn process_command(self) -> Result<(), AppErrors> {
        match self {
            Cli::Sentence { sentence, .. } => cli_sentence_command(sentence).await?,
            Cli::Config { command, .. } => match command {
                None => show_config_data()?,
                Some(command) => cli_config_command(command)?,
            },
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
                ConfigKey::CerebrasPrivateKey => InputData::CerebrasInference(Key::Private(value)),
            };

            set_config_data(&get_path(CONFIG_NAME)?, input)?;
        }
        ConfigCommands::Default { provider } => {
            let input = match provider {
                Providers::GroqCloud => InputData::CurrentProvider(AvailableAiProviders::GroqCloud),
                Providers::GoogleAiStudio => {
                    InputData::CurrentProvider(AvailableAiProviders::GoogleAiStudio)
                }
                Providers::OpenRouter => {
                    InputData::CurrentProvider(AvailableAiProviders::OpenRouter)
                }
                Providers::CerebrasInference => {
                    InputData::CurrentProvider(AvailableAiProviders::CerebrasInference)
                }
            };

            set_config_data(&get_path(CONFIG_NAME)?, input)?;
        }
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