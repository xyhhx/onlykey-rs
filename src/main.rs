use anyhow::Result;
use clap::Parser;
use onlykey_rs::{
    cli::{Cli, Commands},
    ctap::cli::cli_handler as ctap_handler,
    ok::cli::cli_handler as ok_handler,
    onlykey::OnlyKey,
};

fn main() -> Result<()> {
    pretty_env_logger::init();

    let args = Cli::try_parse()?;

    let ok = OnlyKey::connect()?;
    match args.command {
        // Commands::Preferences {} => {}
        Commands::KeyConfiguration(args) => ok_handler(args, &ok)?,
        // Commands::SSH {} => {}
        // Commands::GPG {} => {}
        Commands::CTAP(args) => ctap_handler(args, &ok)?,
    }

    Ok(())
}
