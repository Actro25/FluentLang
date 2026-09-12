use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Input{
    sentence: Option<String>,

    #[command(subcommand)]
    command: Option<MainCommands>,
}

#[derive(Subcommand, Debug)]
enum MainCommands {

    Config {
        #[command(subcommand)]
        command: ConfigSubcommands
    }
}

#[derive(Subcommand, Debug)]
enum ConfigSubcommands {
    Set {
        public: String,
        private: String,
    },
    Show,
}