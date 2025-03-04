use clap::{Parser, Subcommand};

use crate::ctap::cli::CtapArgs;
use crate::ok::cli::KeyConfigurationArgs;

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
  KeyConfiguration(KeyConfigurationArgs),
  //
  // #[command(arg_required_else_help = true)]
  // SSH {},
  //
  // #[command(arg_required_else_help = true)]
  // GPG {},
  CTAP(CtapArgs),
}
