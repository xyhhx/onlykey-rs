use clap::Parser;
use onlykey_rs::cli::{Cli, Commands};
use onlykey_rs::ctap::cli::cli_handler as ctap_handler;
use onlykey_rs::ok::cli::cli_handler as ok_handler;
use onlykey_rs::onlykey::OnlyKey;
use onlykey_rs::pgp::cli::cli_handler as pgp_handler;
use onlykey_rs::ssh::cli::cli_handler as ssh_handler;

fn main() -> eyre::Result<()> {
  color_eyre::install()?;
  pretty_env_logger::init();
  let args = Cli::try_parse()?;
  let ok = OnlyKey::connect()?;

  match args.command {
    // Commands::Preferences {} => {}
    Commands::KeyConfiguration(args) => ok_handler(args, &ok)?,

    Commands::SSH(args) => ssh_handler(args, &ok)?,
    Commands::PGP(args) => pgp_handler(args, &ok)?,
    Commands::CTAP(args) => ctap_handler(args, &ok)?,
  }

  Ok(())
}
