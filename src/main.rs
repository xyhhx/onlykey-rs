extern crate log;
extern crate pretty_env_logger;

use anyhow::Result;

use log::debug;
use onlykey_rs::onlykey::{MessageType, OnlyKey, CTAPHID_HEADER, OK_MESSAGE_HEADER};

fn main() -> Result<()> {
    pretty_env_logger::init();

    let mut ok = OnlyKey::connect()?;
    ok.get_key_labels()?;
    let res = ok.read_as_string()?;
    debug!("{:?}", res);

    Ok(())
}
