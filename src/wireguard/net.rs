pub use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use std::fmt;

use serde::{Deserialize, Serialize};

use rand::Rng;
use rand::rngs::ThreadRng;

// #[derive(Deserialize, Serialize)]
// pub struct Ipv4Addr {
//     pub octets: [u8; 4],
// }

// #[derive(Deserialize, Serialize)]
// pub struct Ipv6Addr {
//     pub octets: [u8; 16],
// }

// #[derive(Deserialize, Serialize)]
// pub enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }

#[derive(Deserialize, Serialize)]
pub struct Ipv4Prefix {
    pub addr: Ipv4Addr,
    pub len: u8,
}

#[derive(Deserialize, Serialize)]
pub struct Ipv6Prefix {
    pub addr: Ipv6Addr,
    pub len: u8,
}

#[derive(Deserialize, Serialize)]
pub enum IpPrefix {
    V4(Ipv4Prefix),
    V6(Ipv6Prefix),
}

// impl Ipv4Addr {
//     pub fn from(octets: [u8; 4]) -> Self {
//         Self { octets }
//     }
// }

// impl Ipv6Addr {
//     pub fn from(octets: [u8; 16]) -> Self {
//         Self { octets }
//     }
// }

impl Ipv4Prefix {
    pub fn new() -> Self {
        let octets = [0u8; 4];

        Self {
            addr: Ipv4Addr::from(octets),
            len: 0,
        }
    }

    // Set interface identifier.
    pub fn addr(&self, id: u16) -> Self {
        // todo: fill octets depending on len of prefix, 32, 24, 16, etc
        let mut octets = self.addr.octets();
        let interface = id.to_be_bytes(); // "be", "le", "ne" (https://en.wikipedia.org/wiki/Endianness)

        octets[2..].copy_from_slice(&interface);

        Self {
            addr: Ipv4Addr::from(octets),
            len: 32,
        }
    }
}

impl Ipv6Prefix {
    pub fn new() -> Self {
        let octets = [0u8; 16];

        Self {
            addr: Ipv6Addr::from(octets),
            len: 0,
        }
    }

    // Generate unique local address (ULA) prefix.
    pub fn generate_ula(&self) -> Self {
        let mut rng = rand::rng();
        let mut octets = self.addr.octets();

        rng.fill_bytes(&mut octets[..6]);
        octets[0] = 0xfd;

        Self {
            addr: Ipv6Addr::from(octets),
            len: 48,
        }
    }

    // Generate subnet ID.
    pub fn generate_network(&self) -> Self {
        let mut rng = rand::rng();
        let mut octets = self.addr.octets();

        rng.fill_bytes(&mut octets[6..8]);

        Self {
            addr: Ipv6Addr::from(octets),
            len: 64,
        }
    }

    // Set interface identifier.
    pub fn addr(&self, id: u64) -> Self {
        let mut octets = self.addr.octets();
        let interface = id.to_be_bytes(); // "be", "le", "ne" (https://en.wikipedia.org/wiki/Endianness)

        octets[8..].copy_from_slice(&interface);

        Self {
            addr: Ipv6Addr::from(octets),
            len: 128,
        }
    }
}

impl fmt::Display for Ipv4Prefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.addr, self.len)
    }
}

impl fmt::Display for Ipv6Prefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.addr, self.len)
    }
}
