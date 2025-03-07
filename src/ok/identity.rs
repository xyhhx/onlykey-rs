use eyre::{Ok, Result};
use libcrux_sha2::sha256;
use log::debug;
use regex::Regex;

const HARD_BYTES_OWO: u32 = 0x80000000u32;

#[derive(Debug)]
pub struct Slip0013Identity {
  protocol: String,
  user: Option<String>,
  host: String,
  port: Option<String>,
  path: Option<String>,
}

impl Slip0013Identity {
  #[allow(dead_code)]
  pub fn new(
    protocol: &str,
    user: Option<&str>,
    host: &str,
    port: Option<&str>,
    path: Option<&str>,
  ) -> Self {
    let protocol = String::from(protocol);
    let user = user.map(String::from);
    let host = String::from(host);
    let path = path.map(String::from);
    let port = port.map(String::from);

    Slip0013Identity {
      protocol,
      user,
      host,
      port,
      path,
    }
  }
}

pub trait Bip32Address {
  #[allow(dead_code)]
  fn as_bip32_address(&self) -> Result<String>;
  fn into_bip32_address(identity_string: &str) -> Result<String>;
}

impl Bip32Address for Slip0013Identity {
  #[allow(dead_code)]
  fn as_bip32_address(&self) -> Result<String> {
    debug!("trying to get bip32 address for {:?}", self);
    let user = self.user.clone().unwrap_or(String::from(""));
    let port = self.port.clone().unwrap_or(String::from(""));
    let path = self.path.clone().unwrap_or(String::from(""));
    let identity_string = format!(
      "{}://{}@{}{}{}",
      self.protocol, &user, self.host, &port, &path
    );

    Ok(Self::into_bip32_address(&identity_string)?)
  }

  fn into_bip32_address(id_str: &str) -> Result<String> {
    debug!("hashing 0{}", id_str);
    let id_bytes: Vec<u8> = [vec![0u8; 4], Vec::<u8>::from(id_str)].concat();
    let sha256_hash = sha256(&id_bytes);
    let hash_128 = &sha256_hash[..16];
    let mut address_n: Vec<u32> = vec![13u32 | HARD_BYTES_OWO];
    for chunk in hash_128.chunks_exact(4) {
      let c: [u8; 4] = chunk.try_into()?;
      address_n.push(u32::from_le_bytes(c) | HARD_BYTES_OWO);
    }

    Ok(format!(
      "m/{}/{}/{}/{}/{}",
      address_n[0], address_n[1], address_n[2], address_n[3], address_n[4]
    ))
  }
}

impl From<String> for Slip0013Identity {
  fn from(identity_string: String) -> Self {
    debug!("parsing {:?}", &identity_string);
    let re = Regex::new(
      r"^(?:(?P<proto>.*)://)?(?:(?P<user>.*)@)?(?P<host>.*?)(?::(?P<port>\w*))?(?P<path>/.*)?$",
    )
    .unwrap();
    let props = re.captures(&identity_string).unwrap();

    Slip0013Identity {
      protocol: props.name("proto").map_or("", |m| m.as_str()).to_string(),
      user: props.name("user").map(|m| m.as_str().to_string()),
      host: props.name("host").map_or("", |m| m.as_str()).to_string(),
      port: props.name("port").map(|m| m.as_str().to_string()),
      path: props.name("path").map(|m| m.as_str().to_string()),
    }
  }
}

impl From<&str> for Slip0013Identity {
  fn from(identity_string: &str) -> Self {
    Slip0013Identity::from(identity_string.to_string())
  }
}

#[cfg(test)]
mod tests {
  use std::any::{Any, TypeId};

  use super::*;

  const URI: &str = "https://satoshi@bitcoin.org/login";
  // const SHA256_HASH: &str = "d0e2389d4c8394a9f3e32de01104bf6e8db2d9e2bb0905d60fffa5a18fd696db";
  const BIP32_ADDR: &str = "m/2147483661/2637750992/2845082444/3761103859/4005495825";

  #[test]
  fn has_a_constructor() {
    let identity = Slip0013Identity::new("https", Some("satoshi"), "bitcoin.org", None, Some("/"));

    assert!(is_slip_0013_identity_instance(&identity));
  }

  #[test]
  fn can_parse_identity_string() {
    let identity = Slip0013Identity::from(URI);

    assert!(is_slip_0013_identity_instance(&identity));
    assert_eq!(identity.protocol.to_string(), "https");
    assert_eq!(identity.user.unwrap().to_string(), "satoshi");
    assert_eq!(identity.host.to_string(), "bitcoin.org");
    assert_eq!(identity.path.unwrap().to_string(), "/login");
    assert_eq!(identity.port, None);
  }

  #[test]
  fn implements_into_bip32() {
    let addr = Slip0013Identity::into_bip32_address(URI).unwrap();

    assert_eq!(format!("{}", addr), BIP32_ADDR);
  }

  #[test]
  fn can_make_bip32_address_from_self() {
    let identity = Slip0013Identity::from(URI);
    let bip32_address = identity.as_bip32_address();

    assert_eq!(format!("{}", bip32_address.unwrap()), BIP32_ADDR);
  }

  fn is_slip_0013_identity_instance<T: ?Sized + Any>(_s: &T) -> bool {
    TypeId::of::<Slip0013Identity>() == TypeId::of::<T>()
  }
}

// NOTE: i dont fuck with cryptocurrencies these were just on the docs already
