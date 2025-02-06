use anyhow::Result;
use hidapi::HidDevice;
use log::debug;

use crate::ok::lib::{MessageType, OK_MESSAGE_HEADER, TIMEOUT};

pub struct OnlyKeyApi;

pub fn write(device: &HidDevice, message_type: u8, message: &[u8]) -> Result<()> {
    let mut payload: Vec<u8> = OK_MESSAGE_HEADER.into();

    payload.push(message_type);
    payload.extend_from_slice(message);
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
    let response_length = device.read_timeout(&mut buffer[..], TIMEOUT)?;
    debug!("Got a response {:?} bytes long", response_length);
    debug!("Buffer raw: {:x?}", &buffer[..]);
    buffer.resize(response_length, 0);
    debug!("Buffer padded: {:x?}", &buffer[..]);

    Ok(buffer)
}

pub fn get_key_labels(device: &HidDevice) -> Result<()> {
    debug!("Getting key labels");
    write(&device, MessageType::OkGetLabels as u8, &[107])?;

    Ok(())
}
