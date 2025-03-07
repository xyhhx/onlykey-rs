use std::time::Duration;

use eyre::{Error, Result};
use hidapi::{HidApi, HidDevice};
use log::{debug, info};

const TIMEOUT: Duration = Duration::from_secs(5);

const OK_DEVICE_IDS: [(u16, u16); 2] = [
  // OnlyKey
  (0x16C0, 0x0486),
  // OnlyKey Duo
  (0x1d50, 0x60fc),
];

pub struct OnlyKey {
  device: HidDevice,
}

impl OnlyKey {
  pub fn new(device: HidDevice) -> Result<Self> {
    let _ = device.read_timeout(&mut [], 0).is_ok();
    let ok = OnlyKey { device };
    Ok(ok)
  }

  fn _get_report_descriptor(_device: &HidDevice) -> Result<()> {
    todo!()
  }

  pub fn connect() -> Result<OnlyKey> {
    debug!("Attempting to connect...");
    let api = HidApi::new()?;
    let device = api
      .device_list()
      .filter(|dev| OK_DEVICE_IDS.contains(&(dev.vendor_id(), dev.product_id())))
      .find(|dev| {
        if dev.serial_number() == Some("1000000000") {
          if dev.usage_page() == 0xFFAB || dev.interface_number() == 2 {
            info!(
              "Found OK device at {}:{}",
              dev.vendor_id(),
              dev.product_id()
            );
            return true;
          }
        } else if dev.usage_page() == 0xF1D0 || dev.interface_number() == 1 {
          info!(
            "Found OK device at {}:{}",
            dev.vendor_id(),
            dev.product_id()
          );
          return true;
        }
        false
      })
      .expect("No onlykeys found!")
      .open_device(&api)?;

    debug!(
      "device: {} {} {} {:#?}",
      device.get_device_info()?.serial_number().unwrap(),
      device.get_device_info()?.manufacturer_string().unwrap(),
      device.get_device_info()?.product_string().unwrap(),
      device.get_device_info()?.path(),
      // device.get_device_info()?
    );

    let ok = OnlyKey::new(device)?;
    ok.device.set_blocking_mode(false)?;

    Ok(ok)
  }

  pub fn write(&self, payload: &mut Vec<u8>) -> Result<()> {
    debug!("Writing {:?}", &payload);
    debug!("{:x?}", &payload);

    self.device.write(payload)?;

    Ok(())
  }

  pub fn read(&self) -> Result<Vec<u8>> {
    self.device.set_blocking_mode(true)?;
    debug!("Reading from onlykey...");

    let mut buffer = vec![0; 64];
    let response_length = self
      .device
      .read_timeout(&mut buffer, TIMEOUT.as_nanos() as i32)?;
    debug!("Got a response {:?} bytes long", response_length);
    debug!("Buffer raw: {:x?}", &buffer[..]);

    buffer.resize(response_length, 0);
    debug!("Buffer padded: {:x?}", &buffer[..]);

    self.device.set_blocking_mode(false)?;

    Ok(buffer)
  }

  pub fn parse_readout(&self, bytes: Vec<u8>) -> Result<String> {
    let s = String::from_utf8(bytes.split(|&c| c == 0).next().unwrap_or_default().to_vec())
      .map_err(Error::from)?;
    Ok(s)
  }

  pub fn get_key_labels(&self) -> Result<()> {
    crate::ok::api::get_key_labels(self)?;

    Ok(())
  }

  pub fn wink(&self) -> Result<()> {
    crate::ctap::api::wink(self)?;

    Ok(())
  }

  pub fn init_ctap(&self) -> Result<()> {
    crate::ctap::api::init(self)?;

    Ok(())
  }

  pub fn get_pubkey(&self, identity: String) -> Result<()> {
    crate::ok::api::get_pubkey(self, identity, true)?;

    Ok(())
  }
}
