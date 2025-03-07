use eyre::Result;
use clap::{Args, Subcommand};

use crate::onlykey::OnlyKey;

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct SshArgs {
  #[command(subcommand)]
  command: Option<SSH>,
}

#[derive(Debug, Subcommand)]
pub enum SSH {}

pub fn cli_handler(args: SshArgs, _ok: &OnlyKey) -> Result<()> {
  let _key_config_command = args.command.unwrap();
  todo!();
}
