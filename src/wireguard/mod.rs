pub const WG_DEFAULT_LISTENING_PORT: u16 = 51820;
pub const WG_DEFAULT_KEEP_ALIVE: u16 = 25;
pub const WG_DEFAULT_MTU: u16 = 1420;

pub mod crypto;
// pub mod fs;
pub mod net;

// use std::fmt::Write;

use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use net::{IpAddr, IpPrefix, Ipv4Addr, Ipv4Prefix, Ipv6Addr, Ipv6Prefix};

use serde::{Deserialize, Serialize};
use serde_json;

// use crypto::{KeyPair, KeySecret};
use wireguard_keys::{Privkey as PrivateKey, Pubkey as PublicKey, Secret as PresharedKey};

// #[derive(Serialize, Deserialize)]
// pub struct WireGuardConfig {
//     // This is all we need for a basic interconnected roaming network. See `man 8 wg wg-quick`.

//     // Meta
//     // Thought: List peers with '-l' and add/delete with '--add' '--delete'.
//     pub id: u64,
//     pub peer: String,

//     // [Interface]
//     interface: Interface,

//     // [Peer]
//     peers: Vec<Peer>,
// }

// #[derive(Serialize, Deserialize)]
// struct Interface {
//     private_key: String,         // PrivateKey (Fix: Use byte slice array.)
//     address: IpPrefix,           // Address
//     dns: Option<IpAddr>,         // DNS (Fix: Only include octets, not len.)
//     listening_port: Option<u16>, // ListenPort
//     mtu: Option<u16>,            // MTU
//     firewall_mark: Option<u32>,  // FwMark
// }

// #[derive(Serialize, Deserialize)]
// struct Peer {
//     public_key: String,       // PublicKey
//     preshared_key: String,    // PresharedKey
//     allowed_ips: IpPrefix,    // AllowedIPs
//     endpoint: Option<String>, // Endpoint
//     keep_alive: Option<u16>,  // PersistentKeepalive
// }

#[derive(Serialize, Deserialize)]
pub struct Config {
    prefix: Ipv6Prefix,
    peers: Vec<Peer>,
}

#[derive(Serialize, Deserialize)]
struct Peer {
    meta: Meta,
    access: Access,
    network: Network,
}

#[derive(Serialize, Deserialize)]
struct Meta {
    name: String,
    id: u64,
}

#[derive(Serialize, Deserialize)]
struct Access {
    key: PrivateKey,
    psk: PresharedKey,
}

#[derive(Serialize, Deserialize)]
struct Network {
    endpoint: Option<String>,
    listen_port: Option<u16>,
    keep_alive: Option<u16>,
    fw_mark: Option<u32>,
    mtu: Option<u16>,
}

impl Config {
    // Generate basic config from list of pseudo names. Good starting point.
    pub fn from(names: Vec<String>) -> Self {
        let mut peers: Vec<Peer> = Vec::new();

        let mut id: u64 = 0;

        for name in names {
            let key = PrivateKey::generate();
            let psk = PresharedKey::generate();

            let peer = Peer {
                meta: Meta { name, id },
                access: Access { key, psk },
                network: Network {
                    endpoint: None,
                    listen_port: None,
                    keep_alive: None,
                    fw_mark: None,
                    mtu: None,
                },
            };
            peers.push(peer);

            id += 1;
        }

        let prefix = Ipv6Prefix::new().generate_ula().generate_network();

        // let address = prefix.address(1);

        Self {
            // prefix: IpPrefix::V6(prefix),
            prefix,
            peers,
        }
    }

    pub fn from_path(path: PathBuf) -> Self {
        let contents = fs::read_to_string(&path).unwrap();

        let config: Self = serde_json::from_str(&contents).unwrap();

        config
    }

    pub fn write(&self, path: PathBuf) {
        let json = serde_json::to_string_pretty(&self).unwrap();

        fs::write(&path, &json).expect("file should exist");
    }

    // todo: use Display trait instead?
    pub fn export(&self, path: PathBuf) {
        fs::create_dir_all(&path).unwrap();

        // Unbound WireGuard config
        let mut unbound_config = File::create(path.join("unbound.conf")).unwrap();

        // First peer is considered the "server" peer, remainder are "client" peers.
        let mut server_config = File::create(path.join("wireguard.conf")).unwrap();

        for peer in &self.peers {
            if peer.meta.id == 0 {
                // [Interface]
                writeln!(server_config, "[Interface]").expect("write to string");

                // PrivateKey
                writeln!(server_config, "PrivateKey = {}", peer.access.key)
                    .expect("write to string");

                // Address
                let prefix = self.prefix.addr(peer.meta.id);
                // let prefix = match self.prefix {
                //     IpPrefix::V4(prefix) => prefix.address(peer.meta.id),
                //     IpPrefix::V6(prefix) => prefix.address(peer.meta.id),
                // };
                writeln!(server_config, "Address = {}", prefix).expect("write to string");

                // ListenPort
                if let Some(port) = peer.network.listen_port {
                    writeln!(server_config, "ListenPort = {}", port).expect("write to string");
                }

                // FwMark
                if let Some(mark) = peer.network.fw_mark {
                    writeln!(server_config, "FwMark = {}", mark).expect("write to string");
                }

                // MTU
                if let Some(mtu) = peer.network.mtu {
                    writeln!(server_config, "MTU = {}", mtu).expect("write to string");
                }

                continue;
            }

            let name = format!("{}-{}.conf", peer.meta.id, peer.meta.name);
            let mut config = File::create(&path.join(name)).unwrap();

            // [Interface]
            writeln!(config, "[Interface]").expect("write to string");

            // PrivateKey
            writeln!(config, "PrivateKey = {}", peer.access.key).expect("write to string");

            // Address
            writeln!(config, "Address = {}", self.prefix.addr(peer.meta.id)).expect("write to string");
        }
    }

    // pub fn write(&self) {
    //     // Serialize Config struct to JSON file
    // }
    // pub fn export(&self) {
    //     // First peer is considered the "server" peer, remainder are "client" peers.
    // }
}

struct WireGuardConfig {
    interface: WireGuardInterface,

    peers: Vec<WireGuardPeer>,
}
struct WireGuardInterface {
    private_key: String,
    listen_port: Option<u16>,
    fw_mark: u32,
}
struct WireGuardPeer {
    public_key: PublicKey,
    secret_key: PresharedKey,
    allowed_ips: IpPrefix,
    endpoint: Option<String>,
    keep_alive: Option<u16>,
}
