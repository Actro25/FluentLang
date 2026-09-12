use clap::{Parser, Subcommand};
use crate::console::attributes::args::set::SetArg;
use crate::console::attributes::args::show::ShowArgs;

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
        command: ConfigSubcommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum ConfigSubcommands {
    Set(#[command(flatten)] SetArg),
    Show(#[command(flatten)] ShowArgs),
}
