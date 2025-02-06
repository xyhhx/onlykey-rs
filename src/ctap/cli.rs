use anyhow::Result;
use clap::{Args, Subcommand};

use crate::onlykey::OnlyKey;

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct CtapArgs {
    #[command(subcommand)]
    pub command: Option<CtapCommands>,
}

#[derive(Debug, Subcommand)]
pub enum CtapCommands {
    Ping,
    Wink,
    SetPin,
}

pub fn handle_ctap_command(args: CtapArgs, ok: &OnlyKey) -> Result<()> {
    let ctap_command = args.command.unwrap();
    match ctap_command {
        CtapCommands::Wink => ok.wink()?,

        CtapCommands::Ping => {
            // TODO:
        }
        CtapCommands::SetPin => {
            // TODO:
        }
    }

    Ok(())
}
