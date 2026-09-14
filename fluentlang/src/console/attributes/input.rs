use crate::console::attributes::args::set::SetArg;
use crate::console::attributes::args::show::ShowArgs;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Input {
    pub sentence: Option<String>,

    #[command(subcommand)]
    pub command: Option<MainCommands>,
}

#[derive(Subcommand, Debug)]
pub enum MainCommands {
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    Set {
        #[command(subcommand)]
        command: ConfigSubcommands,
    },
    Show(#[command(flatten)] ShowArgs),
}

#[derive(Subcommand, Debug)]
pub enum ConfigSubcommands {
    Keys(#[command(flatten)] SetArg),
}
