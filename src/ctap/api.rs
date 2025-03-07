use eyre::Result;
use log::debug;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

use crate::ctap::types::ctap_hid::BROADCAST_CHANNEL_ID;
use crate::onlykey::OnlyKey;

pub fn init(ok: &OnlyKey) -> Result<()> {
  let mut rng = ChaCha20Rng::from_os_rng();
  let nonce: [u8; 8] = rng.random();

  let mut payload: Vec<u8> = [
    [0u8].to_vec(),
    [BROADCAST_CHANNEL_ID as u8].to_vec(),
    nonce.to_vec(),
  ]
  .concat();
  payload.resize(64, 0u8);

  debug!("Writing {:?}", payload);
  debug!("{:x?}", payload);

  ok.write(&mut payload)?;

  Ok(())
}

pub fn wink(ok: &OnlyKey) -> Result<()> {
  debug!("\n\nRunning wink");

  let mut payload = vec![0u8, 17, 0, 0, 0, 136];
  payload.resize(64, 0u8);

  ok.write(&mut payload)?;

  Ok(())
}
