use std::net::{Ipv4Addr, Ipv6Addr};

use std::fmt;

use rand::Rng;

use serde::{Deserialize, Serialize};

// TO-DO: write better ser/de methods instead of deriving traits
// Make serde a feature flag instead. Export only without serde support.
//
// https://serde.rs/custom-serialization.html
//
// https://github.com/serde-rs/serde/blob/aa7913ad0211fef960df68e7812a6211145a888c/serde_core/src/ser/impls.rs#L736
// https://github.com/serde-rs/serde/blob/aa7913ad0211fef960df68e7812a6211145a888c/serde_core/src/ser/impls.rs#L806
// https://github.com/serde-rs/serde/blob/aa7913ad0211fef960df68e7812a6211145a888c/serde_core/src/ser/impls.rs#L830

#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
pub struct Ipv4Len {
    len: u8,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
pub struct Ipv6Len {
    len: u8,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
pub struct Ipv4Prefix {
    addr: Ipv4Addr,
    len: Ipv4Len,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
pub struct Ipv6Prefix {
    addr: Ipv6Addr,
    len: Ipv6Len,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
pub enum IpPrefix {
    V4(Ipv4Prefix),
    V6(Ipv6Prefix),
}

impl From<Ipv4Prefix> for IpPrefix {
    fn from(ip: Ipv4Prefix) -> IpPrefix {
        IpPrefix::V4(ip)
    }
}

impl From<Ipv6Prefix> for IpPrefix {
    fn from(ip: Ipv6Prefix) -> IpPrefix {
        IpPrefix::V6(ip)
    }
}

impl From<u8> for Ipv4Len {
    fn from(len: u8) -> Self {
        let len: u8 = if len > 32 { 32 } else { len };
        Self { len }
    }
}

impl From<u8> for Ipv6Len {
    fn from(len: u8) -> Self {
        let len: u8 = if len > 128 { 128 } else { len };
        Self { len }
    }
}

impl Default for Ipv4Prefix {
    fn default() -> Self {
        let octets = [0u8; 4];

        Self {
            addr: Ipv4Addr::from(octets),
            len: Ipv4Len::from(0),
        }
    }
}

impl Default for Ipv6Prefix {
    fn default() -> Self {
        let octets = [0u8; 16];

        Self {
            addr: Ipv6Addr::from(octets),
            len: Ipv6Len::from(0),
        }
    }
}

impl Ipv4Len {
    pub fn len(&self) -> u8 {
        self.len
    }
}

impl Ipv6Len {
    pub fn len(&self) -> u8 {
        self.len
    }
}

impl Ipv4Prefix {
    pub fn new(addr: Ipv4Addr, len: Ipv4Len) -> Self {
        Self { addr, len }
    }

    pub fn addr(&self) -> Ipv4Addr {
        self.addr
    }

    pub fn len(&self) -> Ipv4Len {
        self.len
    }

    // Set interface identifier.
    pub fn int(&self, int: u16) -> Self {
        // TO-DO: fill octets depending on prefix length, 32, 24, 16, etc.
        let mut octets = self.addr.octets();

        // "be", "le", "ne" (https://en.wikipedia.org/wiki/Endianness)
        octets[2..].copy_from_slice(&int.to_be_bytes());

        Self {
            addr: Ipv4Addr::from(octets),
            len: Ipv4Len::from(32),
        }
    }
}

impl Ipv6Prefix {
    pub fn new(addr: Ipv6Addr, len: Ipv6Len) -> Self {
        Self { addr, len }
    }

    pub fn addr(&self) -> Ipv6Addr {
        self.addr
    }

    pub fn len(&self) -> Ipv6Len {
        self.len
    }

    // Generate unique local address (ULA) prefix.
    pub fn generate_ula(&self) -> Self {
        let mut rng = rand::rng();
        let mut octets = self.addr.octets();

        rng.fill_bytes(&mut octets[..6]);
        octets[0] = 0xfd;

        Self {
            addr: Ipv6Addr::from(octets),
            len: Ipv6Len::from(48),
        }
    }

    // Generate subnet ID.
    pub fn generate_network(&self) -> Self {
        let mut rng = rand::rng();
        let mut octets = self.addr.octets();

        rng.fill_bytes(&mut octets[6..8]);

        Self {
            addr: Ipv6Addr::from(octets),
            len: Ipv6Len::from(64),
        }
    }

    // Set interface identifier.
    pub fn int(&self, int: u64) -> Self {
        // TO-DO: fill octets depending on prefix length, 128, 64, 48, etc.
        let mut octets = self.addr.octets();

        // TO-DO: use PartialEq and or Eq to determine prefix length. Should be 64.
        // if self != 64 {
        //     panic!("!!!!!");
        // }

        // "be", "le", "ne" (https://en.wikipedia.org/wiki/Endianness)
        octets[8..].copy_from_slice(&int.to_be_bytes());

        Self {
            addr: Ipv6Addr::from(octets),
            len: Ipv6Len::from(128),
        }
    }
}

impl fmt::Display for Ipv4Len {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.len)
    }
}

impl fmt::Display for Ipv6Len {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.len)
    }
}

impl fmt::Display for Ipv4Prefix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.addr, self.len)
    }
}

impl fmt::Display for Ipv6Prefix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.addr, self.len)
    }
}
