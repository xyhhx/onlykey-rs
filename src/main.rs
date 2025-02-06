use anyhow::Result;
use clap::Parser;
use onlykey_rs::{
    cli::{Cli, Commands},
    ctap::cli::handle_ctap_command,
    ok::cli::cli_handler,
    onlykey::OnlyKey,
};

fn main() -> Result<()> {
    pretty_env_logger::init();

    let args = Cli::try_parse()?;

    let ok = OnlyKey::connect()?;
    match args.command {
        // Commands::Preferences {} => {}
        Commands::KeyConfiguration(args) => cli_handler(args, &ok)?,
        // Commands::SSH {} => {}
        // Commands::GPG {} => {}
        Commands::CTAP(args) => handle_ctap_command(args, &ok)?,
    }

    Ok(())
}
