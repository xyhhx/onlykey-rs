use std::convert::Into;

use anyhow::Result;
use regex::Regex;

#[derive(Debug)]
pub struct Slip0013Identity {
  protocol: String,
  user: String,
  host: String,
  port: Option<String>,
  path: String,
}


impl Slip0013Identity {
    pub fn new(protocol: &str, user: &str, host: &str, port: Option<&str>, path: &str) -> Self {
        let protocol = String::from(protocol);
        let user = String::from(user);
        let host = String::from(host);
        let path = String::from(path);
        let port = port.map(|port| Some(String::from(port))).unwrap_or(None);
        
        Slip0013Identity {
            protocol,
            user,
            host,
            port,
            path,
        }
    }
}

impl From<String> for Slip0013Identity {
    fn from(identity_string: String) -> Self {
        let re = Regex::new(r"^(?:(?P<proto>.*)://)?(?:(?P<user>.*)@)?(?P<host>.*?)(?::(?P<port>\w*))?(?P<path>/.*)?$").unwrap();
        let matches = re.captures(&identity_string).unwrap();

        Slip0013Identity {
            protocol: matches["proto"].to_string(),
            user: matches["user"].to_string(),
            host: matches["host"].to_string(),
            port: matches.name("port").and(None),

            path: matches["path"].to_string(),

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
    use super::*;
    use std::any::{Any, TypeId};

    #[test]
    fn has_a_constructor() {
        let identity = Slip0013Identity::new("https", "satoshi", "bitcoin.org", None, "/");

        assert!(is_slip_0013_identity_instance(&identity));
    }

    #[test]
    fn can_parse_identity_string() {
        // URI  : https://satoshi@bitcoin.org/login
        let identity = Slip0013Identity::from("https://satoshi@bitcoin.org/login");
        assert!(is_slip_0013_identity_instance(&identity));
        assert_eq!(identity.protocol.to_string(), "https");
        assert_eq!(identity.user.to_string(), "satoshi");
        assert_eq!(identity.host.to_string(), "bitcoin.org");
        assert_eq!(identity.path.to_string(), "/login");
        assert_eq!(identity.port, None);
    }

    fn is_slip_0013_identity_instance<T: ?Sized + Any>(_s: &T) -> bool {
        TypeId::of::<Slip0013Identity>() == TypeId::of::<T>()
    }
}

// URI  : https://satoshi@bitcoin.org/login
// hash : d0e2389d4c8394a9f3e32de01104bf6e8db2d9e2bb0905d60fffa5a18fd696db
// path : m/2147483661/2637750992/2845082444/3761103859/4005495825

// NOTE: i dont fuck with cryptocurrencies these were just on the docs
// already
