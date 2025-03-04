use anyhow::Result;
use hidapi::{HidApi, HidDevice, MAX_REPORT_DESCRIPTOR_SIZE};
use log::{debug, error};

use crate::utils::read_string_from_bytes;

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
    let _ = Self::get_report_descriptor(&device);
    let ok = OnlyKey { device };
    Ok(ok)
  }

  fn get_report_descriptor(device: &HidDevice) -> Result<()> {
    debug!("Getting descriptor details");
    let mut buf = [0u8; MAX_REPORT_DESCRIPTOR_SIZE];
    match device.get_report_descriptor(&mut buf) {
      Ok(length) => debug!(
        "\tDescriptor:\n\tLength = {:?}\n\tData = {:?}",
        length,
        &mut buf[..length]
      ),
      Err(error) => error!("\tFailed to retrieve the descriptor: {:?}", error),
    }

    Ok(())
  }

  pub fn connect() -> Result<OnlyKey> {
    debug!("Attempting to connect...");
    let api = HidApi::new()?;
    let device = api
      .device_list()
      .filter(|dev| OK_DEVICE_IDS.contains(&(dev.vendor_id(), dev.product_id())))
      .find(|dev| {
        dev.serial_number() == Some("1000000000")
          && (dev.usage_page() == 0xFFAB || dev.interface_number() == 2)
          || (dev.usage_page() == 0xF1D0 || dev.interface_number() == 1)
      })
      .expect("No onlykeys found!")
      .open_device(&api)?;

    debug!(
      "device: {} {} {} {:#?} {:#?}",
      device.get_device_info()?.serial_number().unwrap(),
      device.get_device_info()?.manufacturer_string().unwrap(),
      device.get_device_info()?.product_string().unwrap(),
      device.get_device_info()?.path(),
      device.get_device_info()?
    );
    let ok = OnlyKey::new(device)?;
    ok.device.set_blocking_mode(false)?;

    Ok(ok)
  }

  pub fn get_key_labels(&self) -> Result<()> {
    crate::ok::api::get_key_labels(&self.device)?;

    Ok(())
  }

  pub fn wink(&self) -> Result<()> {
    crate::ctap::api::wink(&self.device)?;

    Ok(())
  }
}
