use anyhow::Result;
use clap::{Args, Subcommand};

use crate::onlykey::OnlyKey;

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct PgpArgs {
  #[command(subcommand)]
  command: Option<PGP>,
}

#[derive(Debug, Subcommand)]
pub enum PGP {}

pub fn cli_handler(args: PgpArgs, _ok: &OnlyKey) -> Result<()> {
  let _key_config_command = args.command.unwrap();
  todo!();
}
