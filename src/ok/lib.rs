use serde::{Deserialize, Serialize};

pub const OK_MESSAGE_HEADER: [u8; 4] = [255u8; 4];
pub const TIMEOUT: i32 = 5000;

#[repr(u8)]
pub enum KeyType {
  Ed25519,
  P256,
  SecP256K1,
}

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

#[derive(Debug, Deserialize, Serialize, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum KeySlot {
  RSA1 = 1,
  RSA2 = 2,
  RSA3 = 3,
  RSA4 = 4,

  ECC1 = 101,
  ECC2 = 102,
  ECC3 = 103,
  ECC4 = 104,
  ECC5 = 105,
  ECC6 = 106,
  ECC7 = 107,
  ECC8 = 108,
  ECC9 = 109,
  ECC10 = 110,
  ECC11 = 111,
  ECC12 = 112,
  ECC13 = 113,
  ECC14 = 114,
  ECC15 = 115,
  ECC16 = 116,
}
