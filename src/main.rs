extern crate log;
extern crate pretty_env_logger;

use anyhow::Result;

use log::debug;
use onlykey_rs::onlykey::{MessageType, OnlyKey};

fn main() -> Result<()> {
    pretty_env_logger::init();

    let mut ok = OnlyKey::connect()?;
    ok.write(MessageType::OkGetLabels as u8, &[])?;
    let res = ok.read_as_string()?;
    debug!("{:?}", res);

    Ok(())
}
