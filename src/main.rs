use anyhow::Result;
use clap::Parser;

use onlykey_rs::{
    cli::{Cli, Commands, CtapCommands},
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
        Commands::CTAP(ctap) => {
            let ctap_command = ctap.command.unwrap();
            match ctap_command {
                CtapCommands::Wink => {
                    ok.wink()?;
                }
                CtapCommands::Ping => {
                    // TODO:
                }
                CtapCommands::SetPin => {
                    // TODO:
                }
            }
        }
    }

    ok.read_as_string()?;

    Ok(())
}
