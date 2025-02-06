use anyhow::Result;
use clap::Parser;
use onlykey_rs::{
    cli::{Cli, Commands},
    ctap::cli::handle_ctap_command,
    onlykey::OnlyKey,
};

fn main() -> Result<()> {
    pretty_env_logger::init();

    let args = Cli::try_parse()?;

    let mut ok = OnlyKey::connect()?;
    match args.command {
        // Commands::Preferences {} => {}
        // Commands::KeyConfiguration(key_config) => {
        //     let key_config_command = key_config.command.unwrap_or_default();
        //     match key_config_command {
        //         KeyConfigurationCommands::GetKeyLabels => {
        //             let mut ok = OnlyKey::connect()?;
        //             ok.get_key_labels()?;
        //         }
        //     }
        // }
        // Commands::SSH {} => {}
        // Commands::GPG {} => {}
        Commands::CTAP(ctap) => handle_ctap_command(ctap, &ok)?,
    }

    ok.read_as_string()?;

    Ok(())
}
