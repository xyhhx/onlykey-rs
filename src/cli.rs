use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "onlykey-rs", author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    // #[command(arg_required_else_help = true)]
    // Preferences {},
    //
    // KeyConfiguration(KeyConfigurationArgs),
    //
    // #[command(arg_required_else_help = true)]
    // SSH {},
    //
    // #[command(arg_required_else_help = true)]
    // GPG {},
    CTAP(CtapArgs),
}

// #[derive(Debug, Args)]
// #[command(args_conflicts_with_subcommands = true)]
// #[command(flatten_help = true)]
// pub struct KeyConfigurationArgs {
//     #[command(subcommand)]
//     command: Option<KeyConfigurationCommands>,
// }
//
// #[derive(Debug, Subcommand)]
// pub enum KeyConfigurationCommands {
//     GetKeyLabels,
// }
//
#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct CtapArgs {
    #[command(subcommand)]
    pub command: Option<CtapCommands>,
}

#[derive(Debug, Subcommand)]
pub enum CtapCommands {
    Ping,
    Wink,
    SetPin,
}
