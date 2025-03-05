use std::{thread, time};

use anyhow::{Error, Result};
use hidapi::HidDevice;
use log::debug;
use strum::IntoEnumIterator;

use crate::ok::lib::{KeySlot, MessageType, OK_MESSAGE_HEADER, TIMEOUT};
use crate::onlykey::OnlyKey;

pub struct OnlyKeyApi;
pub fn read(device: &HidDevice) -> Result<Vec<u8>> {
  device.set_blocking_mode(true)?;
  debug!("Reading from onlykey...");

  let mut buffer = vec![0; 64];
  let response_length = device.read_timeout(&mut buffer, TIMEOUT)?;
  debug!("Got a response {:?} bytes long", response_length);
  debug!("Buffer raw: {:x?}", &buffer[..]);
  buffer.resize(response_length, 0);
  debug!("Buffer padded: {:x?}", &buffer[..]);

  device.set_blocking_mode(false)?;

  Ok(buffer)
}

pub fn parse_readout(bytes: Vec<u8>) -> Result<String> {
  let s = String::from_utf8(bytes.split(|&c| c == 0).next().unwrap_or_default().to_vec())
    .map_err(Error::from)?;
  Ok(s)
}

pub fn get_key_labels(ok: &OnlyKey) -> Result<()> {
  debug!("Getting key labels");

  let mut payload: Vec<u8> = [
    vec![0u8],
    OK_MESSAGE_HEADER.to_vec(),
    vec![MessageType::OkGetLabels as u8],
    vec![107],
  ]
  .concat();

  payload.resize(64, 0);

  ok.write(&mut payload)?;
  thread::sleep(time::Duration::from_millis(100));

  let mut slots: Vec<(u32, String)> = Vec::new();
  for _ in 1..20 {
    let Ok(data) = parse_readout(ok.read()?) else {
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
