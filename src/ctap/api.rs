use anyhow::Result;
use hidapi::HidDevice;
use log::debug;

const CTAPHID_HEADER: [u8; 7] = [255u8, 255, 255, 255, 0x86, 0, 8];

pub fn wink(device: &HidDevice) -> Result<()> {
    debug!("Running wink");
    // self.device.write(&[255, 255, 255, 255, 134, 0, 8])?;
    device.write(&[
        0, 17, 0, 0, 0, 136, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ])?;

    Ok(())
}
