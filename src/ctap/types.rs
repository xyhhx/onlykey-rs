pub mod ctap_hid {
  pub const BROADCAST_CHANNEL_ID: u32 = 0xffffffff;

  pub enum Commands {
    Ping = 0x01,
    Msg = 0x03,
    Lock = 0x04,
    Init = 0x06,
    Wink = 0x08,
    Cbor = 0x10,
    Cancel = 0x11,

    Error = 0x3F,
    Keepalive = 0x3B,

    VendorFirst = 0x40,
  }

  pub enum CapabilitiesFlag {
    Wink = 0x01,
    Lock = 0x02,
    Cbor = 0x04,
    Nmsg = 0x08,
  }
}
