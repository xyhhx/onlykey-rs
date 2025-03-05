#[allow(dead_code)]
pub mod onlykey_interface {

  use serde::{Deserialize, Serialize};
  pub const MESSAGE_HEADER: [u8; 4] = [255u8; 4];

  #[derive(Debug, Deserialize, Serialize, Clone)]
  #[serde(untagged)]
  pub enum SlottedKey {
    StoredKey(StoredKeyInfo),
    DerivedKey(DerivedKeyInfo),
  }

  #[derive(PartialEq)]
  pub enum KeyAlgorithm {
    Rsa(usize),
    Ecc(EccCurveType),
  }

  impl KeySlot {
    pub fn algorithm(&self) -> KeyAlgorithm {
      match self {
        KeySlot::RSA1 | KeySlot::RSA2 | KeySlot::RSA3 | KeySlot::RSA4 => KeyAlgorithm::Rsa(0),
        KeySlot::ECC1
        | KeySlot::ECC2
        | KeySlot::ECC3
        | KeySlot::ECC4
        | KeySlot::ECC5
        | KeySlot::ECC6
        | KeySlot::ECC7
        | KeySlot::ECC8
        | KeySlot::ECC9
        | KeySlot::ECC10
        | KeySlot::ECC11
        | KeySlot::ECC12
        | KeySlot::ECC13
        | KeySlot::ECC14
        | KeySlot::ECC15
        | KeySlot::ECC16 => KeyAlgorithm::Ecc(EccCurveType::Unknown),
        KeySlot::HMAC1 | KeySlot::HMAC2 => KeyAlgorithm::Ecc(EccCurveType::Unknown),
      }
    }
  }

  impl SlottedKey {
    pub fn slot_number(&self) -> u8 {
      match self {
        SlottedKey::StoredKey(keyinfo) => keyinfo.slot_number(),
        SlottedKey::DerivedKey(keyinfo) => keyinfo.slot_number(),
      }
    }

    pub fn algorithm(&self) -> KeyAlgorithm {
      match self {
        SlottedKey::StoredKey(keyinfo) => keyinfo.algorithm(),
        SlottedKey::DerivedKey(keyinfo) => keyinfo.algorithm(),
      }
    }

    pub fn keygrip(&self) -> String {
      match self {
        SlottedKey::StoredKey(key) => key.keygrip.clone(),
        SlottedKey::DerivedKey(key) => key.keygrip.clone(),
      }
    }
  }

  #[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
  pub enum EccCurveType {
    Unknown,
    Ed25519,
    Cv25519,
    Nist256P1,
    Secp256K1,
  }

  #[derive(Debug, Deserialize, Serialize, Clone)]
  pub struct StoredKeyInfo {
    pub slot: KeySlot,
    pub keygrip: String,
    #[serde(default)]
    pub size: usize,
  }

  impl StoredKeyInfo {
    pub fn slot_number(&self) -> u8 {
      self.slot as u8
    }

    pub fn algorithm(&self) -> KeyAlgorithm {
      match self.slot {
        KeySlot::RSA1 | KeySlot::RSA2 | KeySlot::RSA3 | KeySlot::RSA4 => {
          KeyAlgorithm::Rsa(self.size)
        }
        KeySlot::ECC1
        | KeySlot::ECC2
        | KeySlot::ECC3
        | KeySlot::ECC4
        | KeySlot::ECC5
        | KeySlot::ECC6
        | KeySlot::ECC7
        | KeySlot::ECC8
        | KeySlot::ECC9
        | KeySlot::ECC10
        | KeySlot::ECC11
        | KeySlot::ECC12
        | KeySlot::ECC13
        | KeySlot::ECC14
        | KeySlot::ECC15
        | KeySlot::ECC16 => KeyAlgorithm::Ecc(EccCurveType::Unknown),
        KeySlot::HMAC1 | KeySlot::HMAC2 => KeyAlgorithm::Ecc(EccCurveType::Unknown),
      }
    }
  }

  #[derive(Debug, Deserialize, Serialize, Clone)]
  pub struct DerivedKeyInfo {
    pub identity: String,
    pub ecc_curve: EccCurveType,
    pub keygrip: String,
    #[serde(default)]
    pub validity: i64,
    #[serde(default)]
    pub creation: i64,
  }

  impl DerivedKeyInfo {
    pub fn slot_number(&self) -> u8 {
      match self.ecc_curve {
        EccCurveType::Unknown => 132,
        EccCurveType::Ed25519 => 201,
        EccCurveType::Nist256P1 => 202,
        EccCurveType::Secp256K1 => 203,
        EccCurveType::Cv25519 => 204,
      }
    }

    pub fn algorithm(&self) -> KeyAlgorithm {
      KeyAlgorithm::Ecc(self.ecc_curve.clone())
    }

    pub fn curve_type(&self) -> u8 {
      match self.ecc_curve {
        EccCurveType::Unknown => 0,
        EccCurveType::Ed25519 => 1,
        EccCurveType::Nist256P1 => 2,
        EccCurveType::Secp256K1 => 3,
        EccCurveType::Cv25519 => 4,
      }
    }
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

  #[derive(
    Debug,
    Deserialize,
    Serialize,
    Copy,
    Clone,
    PartialEq,
    strum::AsRefStr,
    strum::Display,
    strum::EnumIter,
  )]
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

    HMAC1 = 129,
    HMAC2 = 130,
  }

  #[derive(PartialEq)]
  pub enum CredentialSlotType {
    Duo,
    Standard,
  }

  #[derive(
    Debug,
    Deserialize,
    Serialize,
    Copy,
    Clone,
    PartialEq,
    strum::AsRefStr,
    strum::Display,
    strum::EnumIter,
  )]
  #[repr(u8)]
  pub enum StandardCredentialSlot {
    Slot1a = 1,
    Slot2a = 2,
    Slot3a = 3,
    Slot4a = 4,
    Slot5a = 5,
    Slot6a = 6,
    Slot1b = 7,
    Slot2b = 8,
    Slot3b = 9,
    Slot4b = 10,
    Slot5b = 11,
    Slot6b = 12,
  }

  #[derive(
    Debug,
    Deserialize,
    Serialize,
    Copy,
    Clone,
    PartialEq,
    strum::AsRefStr,
    strum::Display,
    strum::EnumIter,
  )]
  #[repr(u8)]
  pub enum DuoCredentialSlot {
    Green1a = 1,
    Green2a = 2,
    Green3a = 3,
    Green1b = 4,
    Green2b = 5,
    Green3b = 6,

    Blue1a = 7,
    Blue2a = 8,
    Blue3a = 9,
    Blue1b = 10,
    Blue2b = 11,
    Blue3b = 12,

    Yellow1a = 13,
    Yellow2a = 14,
    Yellow3a = 15,
    Yellow1b = 16,
    Yellow2b = 17,
    Yellow3b = 18,

    Purple1a = 19,
    Purple2a = 20,
    Purple3a = 21,
    Purple1b = 22,
    Purple2b = 23,
    Purple3b = 24,
  }
}
