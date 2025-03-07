use clap::{Parser, Subcommand};

use crate::ctap::cli::CtapArgs;
use crate::ok::cli::KeyConfigurationArgs;
use crate::pgp::cli::PgpArgs;
use crate::ssh::cli::SshArgs;

#[derive(Parser, Debug)]
#[command(name = "onlykey-rs", author, version, about, long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
  // Preferences,
  #[command(name = "keys")]
  #[command(about = "Manage the keys (derived and stored) on your Onlykey")]
  KeyConfiguration(KeyConfigurationArgs),
  SSH(SshArgs),
  PGP(PgpArgs),
  CTAP(CtapArgs),
}

#[cfg(test)]
mod tests {
  use clap::CommandFactory;

  use super::*;
  #[test]
  fn verify_cli() {
    Cli::command().debug_assert();
  }
}
