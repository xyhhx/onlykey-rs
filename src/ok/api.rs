use std::{thread, time};

use eyre::Result;
use log::debug;
use strum::IntoEnumIterator;

use crate::ok::types::onlykey_interface::{KeySlot, MessageType, MESSAGE_HEADER};
use crate::onlykey::OnlyKey;

pub struct OnlyKeyApi;

pub fn get_key_labels(ok: &OnlyKey) -> Result<()> {
  debug!("Getting key labels");

  let mut payload: Vec<u8> = [
    vec![0u8],
    MESSAGE_HEADER.to_vec(),
    vec![MessageType::OkGetLabels as u8],
    vec![107],
  ]
  .concat();

  payload.resize(64, 0);

  ok.write(&mut payload)?;
  thread::sleep(time::Duration::from_millis(100));

  let mut slots: Vec<(u32, String)> = Vec::new();
  for _ in 1..20 {
    let Ok(data) = ok.parse_readout(ok.read()?) else {
      continue;
    };
    let (slot_number, slot_label) = data.split_once("|").unwrap_or_default();
    let slot_number = slot_number.as_bytes()[0] as u32;

    if (25..44).contains(&slot_number) {
      let slot_id = match slot_number >= 29 {
        true => slot_number + 72,
        false => slot_number - 24,
      };

      debug!(
        "found key label:\nslot_label={} slot_number={} slot_id={} ",
        slot_label, slot_number, slot_id,
      );

      slots.push((slot_id, String::from(slot_label)));
    }
  }

  for slot in KeySlot::iter() {
    println!(
      "Slot #{} ({}) - {}",
      slot as u32,
      slot,
      slots
        .iter()
        .find(|s| s.0 == slot as u32)
        .unwrap_or(&(0, String::from("")))
        .1
    );
  }

  Ok(())
}

pub fn get_pubkey(_ok: &OnlyKey, _identity: String, _ecdh: bool) -> Result<()> {
  todo!();
  // let slip_0013_id = Slip0013Identity::from(identity);
  // let bip32_address = slip_0013_id.as_bip32_address();

  // Ok(())
}
