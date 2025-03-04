use std::str::FromStr;
use std::{char, thread, time};

use anyhow::{Error, Result};
use hidapi::HidDevice;
use log::debug;

use crate::ok::lib::{MessageType, OK_MESSAGE_HEADER, TIMEOUT};

pub struct OnlyKeyApi;

pub fn write(device: &HidDevice, message_type: u8, message: &[u8]) -> Result<()> {
  let mut payload: Vec<u8> = [
    vec![0u8],
    OK_MESSAGE_HEADER.to_vec(),
    vec![message_type],
    message.to_vec(),
  ]
  .concat();

  payload.resize(64, 0);

  debug!("Writing {:?}", payload);
  debug!("{:x?}", payload);

  device.write(&payload)?;

  Ok(())
}

pub fn read(device: &HidDevice) -> Result<Vec<u8>> {
  device.set_blocking_mode(true)?;
  debug!("Reading from onlykey...");

  let mut buffer = vec![0; 64];
  let response_length = device.read_timeout(&mut buffer, TIMEOUT)?;
  debug!("Got a response {:?} bytes long", response_length);
  debug!("Buffer raw: {:x?}", &buffer[..]);
  buffer.resize(response_length, 0);
  debug!("Buffer padded: {:x?}", &buffer[..]);

  Ok(buffer)
}

pub fn parse_readout(bytes: Vec<u8>) -> Result<String> {
  let s = String::from_utf8(bytes.split(|&c| c == 0).next().unwrap_or_default().to_vec())
    .map_err(Error::from)?;
  Ok(s)
}

pub fn get_key_labels(device: &HidDevice) -> Result<()> {
  debug!("Getting key labels");
  write(device, MessageType::OkGetLabels as u8, &[107])?;
  thread::sleep(time::Duration::from_millis(100));

  for _ in 1..20 {
    let Ok(data) = parse_readout(read(device)?) else {
      continue;
    };
    let chunks = data.split_once("|").unwrap_or_default();
    let slot_number = char::from_str(chunks.0)? as i32;
    dbg!(slot_number);
    if (25..44).contains(&slot_number) {
      println!("Slot: {:?} {:?}", slot_number, chunks.1);
    }
  }

  Ok(())
}
