use anyhow::{bail, Result};
use hidapi::{HidApi, HidDevice};
use log::debug;

const DEVICE_IDS: [(u16, u16); 2] = [
    // OnlyKey
    (0x16C0, 0x0486),
    // OnlyKey Duo
    (0x1d50, 0x60fc),
];
const MESSAGE_HEADER: [u8; 4] = [255u8, 255, 255, 255];
const TIMEOUT: i32 = 5000;

#[repr(u8)]
pub enum MessageType {
    OkDecrypt = 240,
    OkGetLabels = 229,
    OkGetPubKey = 236,
    OkRestore = 241,
    OkSetPdPin = 227,
    OkSetPin = 225,
    OkSetPriv = 239,
    OkSetSdPin = 226,
    OkSetSlot = 230,
    OkSetTime = 228,
    OkSetU2FCert = 234,
    OkSetU2FPriv = 232,
    OkSign = 237,
    OkWipePriv = 238,
    OkWipeSlot = 231,
    OkWipeU2FCert = 235,
    OkWipeU2FPriv = 233,
    OkWink = 0x08,
}

#[repr(u8)]
enum MessageField {
    Label = 1,
    Url = 15,
    Delay1 = 17,
    NextKey4 = 18,
    Username = 2,
    NextKey1 = 16,
    NextKey2 = 3,
    Delay2 = 4,
    Password = 5,
    NextKey3 = 6,
    Delay3 = 7,
    NextKey5 = 19,
    TFAType = 8,
    TOTPKey = 9,
    YubiAuth = 10,
    IdleTimeout = 11,
    WipeMode = 12,
    KeyTypeSpeed = 13,
    KeyLayout = 14,
    LEDBrightness = 24,
    LockButton = 25,
    HMACMode = 26,
    SysAdminMode = 27,
    SecProfileMode = 23,
    PGPChallengeMode = 22,
    SSHChallengeMode = 21,
    BackupMode = 20,
    TouchSense = 28,
}

#[repr(u8)]
enum KeyType {
    Ed25519,
    P256,
    SecP256K1,
}

pub struct OnlyKey {
    device: HidDevice,
}

impl OnlyKey {
    pub fn new(device: HidDevice) -> Result<Self> {
        let _ = device.read_timeout(&mut [], 0).is_ok();
        let ok = OnlyKey { device };
        Ok(ok)
    }

    pub fn connect() -> Result<OnlyKey> {
        debug!("Attempting to connect...");
        let api = HidApi::new()?;
        let device_info = api
            .device_list()
            .find(|dev| DEVICE_IDS.contains(&(dev.vendor_id(), dev.product_id())));

        let device = match device_info {
            Some(d) => {
                debug!("Found a device at {:?} {:?}", d.vendor_id(), d.product_id());
                d.open_device(&api)?
            }
            None => bail!("No onlykeys found"),
        };

        let ok = OnlyKey::new(device)?;

        Ok(ok)
    }

    pub fn write(&mut self, message_type: u8, message: &[u8]) -> Result<()> {
        let mut payload: Vec<u8> = MESSAGE_HEADER.into();
        payload.push(message_type);
        payload.extend_from_slice(message);
        payload.resize(64, 0);
        debug!("Writing {:?}", payload);
        debug!("{:x?}", payload);

        self.device.write(&payload)?;

        Ok(())
    }

    pub fn read(&mut self) -> Result<Vec<u8>> {
        self.device.set_blocking_mode(true)?;
        debug!("Reading from onlykey...");

        let mut buffer = vec![0; 64];
        let response_length = self.device.read_timeout(&mut buffer, TIMEOUT)?;
        buffer.resize(response_length, 0);

        Ok(buffer)
    }

    pub fn read_as_string(&mut self) -> Result<String> {
        let s = String::from_utf8(
            self.read()?
                .split(|&c| c == 0)
                .next()
                .unwrap_or_default()
                .to_vec(),
        )?;

        Ok(s)
    }
}
