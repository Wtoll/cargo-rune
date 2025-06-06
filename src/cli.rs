use clap::{Parser, Subcommand, Args};
use clio::Input;

pub const CLAP_STYLING: clap::builder::styling::Styles = clap::builder::styling::Styles::styled()
    .header(clap_cargo::style::HEADER)
    .usage(clap_cargo::style::USAGE)
    .literal(clap_cargo::style::LITERAL)
    .placeholder(clap_cargo::style::PLACEHOLDER)
    .error(clap_cargo::style::ERROR)
    .valid(clap_cargo::style::VALID)
    .invalid(clap_cargo::style::INVALID);

#[derive(Parser, Debug)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
#[command(styles = CLAP_STYLING)]
pub struct CargoCli {
    #[command(subcommand)]
    pub command: CargoCommands
}

#[derive(Subcommand, Debug)]
pub enum CargoCommands {
    Rune(CargoRune)
}

#[derive(Args, Debug)]
#[command(args_conflicts_with_subcommands = true)]
pub struct CargoRune {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[command(flatten)]
    pub run_args: Option<RunArgs>
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Runs a rune script
    Run(RunArgs),
}

#[derive(Debug, clap::Args)]
pub struct RunArgs {
    /// The rune script to be run
    pub input: Input
}