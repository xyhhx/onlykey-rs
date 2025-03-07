use anyhow::Result;
use clap::{Args, Subcommand};

use crate::onlykey::OnlyKey;

#[derive(Debug, Args)]
#[command(name = "keys", bin_name = "keys")]
#[command(arg_required_else_help = true)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct KeyConfigurationArgs {
  #[command(subcommand)]
  command: Option<KeyConfigurationCommands>,
}

#[derive(Debug, Subcommand)]
pub enum KeyConfigurationCommands {
  GetKeyLabels,
  GetPublicKey { identity: String },
}

pub fn cli_handler(args: KeyConfigurationArgs, ok: &OnlyKey) -> Result<()> {
  let key_config_command = args.command.unwrap();
  match key_config_command {
    KeyConfigurationCommands::GetKeyLabels => ok.get_key_labels()?,
    KeyConfigurationCommands::GetPublicKey { identity } => ok.get_pubkey(identity)?,
  }

  Ok(())
}
