pub const WG_DEFAULT_LISTENING_PORT: u16 = 51820;
pub const WG_DEFAULT_KEEP_ALIVE: u16 = 25;
pub const WG_DEFAULT_MTU: u16 = 1420;

pub mod crypto;
pub mod net;

// use std::fmt::Write;

// use std::collections::HashMap;
use std::collections::BTreeMap;
// use std::fmt::Display;
use std::fmt;

use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
// use std::path::PathBuf;

use net::Ipv6Prefix;

use serde::{Deserialize, Serialize};
use serde_json;

// use crypto::{KeyPair, KeySecret};
use wireguard_keys::{Privkey as PrivateKey, Secret as PresharedKey};

use anyhow::{Context, Result};

#[derive(Serialize, Deserialize)]
pub struct Config {
    // Network
    prefix: Ipv6Prefix,

    // Peers
    peers: BTreeMap<u64, Peer>,
}

#[derive(Serialize, Deserialize)]
struct Peer {
    // Meta
    name: String,

    // Keys
    private_key: PrivateKey,
    preshared_key: PresharedKey,

    // Network
    endpoint: Option<String>,
    listen_port: Option<u16>,
    persistent_keepalive_interval: Option<u16>,
    fwmark: Option<u32>,
    mtu: Option<u16>,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;

        for (id, peer) in &self.peers {
            if !first {
                write!(f, "\n")?;
            }
            first = false;

            write!(f, "{}: {}", id, peer.name)?;
        }

        Ok(())
    }
}

impl Config {
    pub fn new() -> Self {
        let prefix = Ipv6Prefix::new().generate_ula().generate_network();

        let peers: BTreeMap<u64, Peer> = BTreeMap::new();

        Self { prefix, peers }
    }

    pub fn add<T: ToString>(&mut self, name: T) -> Result<()> {
        // Find lowest next available entry. Bail if none left.
        let id = (u64::MIN..u64::MAX)
            .into_iter()
            .find(|k| !self.peers.contains_key(k))
            .context("Too many peers. Whoa.")?;

        let private_key = PrivateKey::generate();
        let preshared_key = PresharedKey::generate();

        let peer = Peer {
            name: name.to_string(),
            private_key,
            preshared_key,
            endpoint: None,
            listen_port: None,
            persistent_keepalive_interval: None,
            fwmark: None,
            mtu: None,
        };
        self.peers.insert(id, peer);

        Ok(())
    }

    pub fn del<T: ToString>(&mut self, name: T) -> Result<()> {
        self.peers.retain(|_, peer| peer.name != name.to_string());

        Ok(())
    }

    pub fn from(path: &Path) -> Result<Self> {
        let file_contents = fs::read_to_string(&path)?;

        let config: Self = serde_json::from_str(&file_contents)?;

        Ok(config)
    }

    pub fn write(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(&self)?;

        fs::write(&path, &json)?;

        Ok(())
    }

    pub fn export(&self, path: &Path) -> Result<()> {
        // First peer is considered the "server" peer, remainder are "client" peers.
        let server = self
            .peers
            .get(&u64::MIN)
            .context("Peer with index of 0 not found.")?;
        // TO-DO: Check for duplicate names!

        // Create export directory
        fs::create_dir_all(&path)?;

        // Server WireGuard config file
        let mut server_config = File::create(path.join("wireguard.conf"))?;

        // [Interface]
        writeln!(server_config, "[Interface]")?;

        // PrivateKey
        writeln!(server_config, "PrivateKey = {}", server.private_key)?;

        // Address
        writeln!(server_config, "Address = {}", self.prefix.int(0))?; // TO-DO: get id rather than assuming zero!

        // ListenPort
        if let Some(v) = server.listen_port {
            writeln!(server_config, "ListenPort = {}", v)?;
        }

        // FwMark
        if let Some(v) = server.fwmark {
            writeln!(server_config, "FwMark = {}", v)?;
        }

        // MTU
        if let Some(v) = server.mtu {
            writeln!(server_config, "MTU = {}", v)?;
        }

        // Server Unbound config file
        let mut unbound_config = File::create(path.join("unbound.conf"))?;

        // Unbound
        writeln!(unbound_config, "# {}", server.name)?;

        // AAAA Record
        writeln!(
            unbound_config,
            "local-data: '{} AAAA {}'",
            server.name,
            self.prefix.int(0).addr
        )?;

        // PTR Record
        writeln!(
            unbound_config,
            "local-data-ptr: '{} {}'",
            self.prefix.int(0).addr,
            server.name,
        )?;

        for (id, peer) in &self.peers {
            if *id == 0 {
                continue;
            }

            let name = format!("{}-{}.conf", id, peer.name);

            // Client WireGuard config file
            let mut config = File::create(&path.join(name))?;

            // TO-DO:
            // Client WireGuard QR code file
            // let mut config = File::create(&path.join(name))?;

            // [Interface]
            writeln!(config, "[Interface]")?;

            // PrivateKey
            writeln!(config, "PrivateKey = {}", peer.private_key)?;

            // Address
            writeln!(config, "Address = {}", self.prefix.int(*id))?;

            // DNS
            writeln!(config, "DNS = {}", self.prefix.int(0).addr)?;

            // ListenPort
            if let Some(port) = peer.listen_port {
                writeln!(config, "ListenPort = {}", port)?;
            }

            // FwMark
            if let Some(mark) = peer.fwmark {
                writeln!(config, "FwMark = {}", mark)?;
            }

            // MTU
            if let Some(mtu) = peer.mtu {
                writeln!(config, "MTU = {}", mtu)?;
            }

            // [Peer]
            writeln!(config, "\n[Peer]")?;

            // PublicKey
            writeln!(config, "PublicKey = {}", server.private_key.pubkey())?;

            // PresharedKey
            writeln!(config, "PresharedKey = {}", peer.preshared_key)?;

            // AllowedIPs
            writeln!(config, "AllowedIPs = {}", self.prefix)?;

            // Endpoint
            if let Some(endpoint) = &server.endpoint {
                writeln!(config, "Endpoint = {}", endpoint)?;
            }

            // PersistentKeepalive
            if let Some(interval) = &peer.persistent_keepalive_interval {
                writeln!(config, "PersistentKeepalive = {}", interval)?;
            }

            // Server

            // [Peer]
            writeln!(server_config)?;
            writeln!(server_config, "# {}", peer.name)?;
            writeln!(server_config, "[Peer]")?;

            // PublicKey
            writeln!(server_config, "PublicKey = {}", peer.private_key.pubkey())?;

            // PresharedKey
            writeln!(server_config, "PresharedKey = {}", peer.preshared_key)?;

            // AllowedIPs
            writeln!(server_config, "AllowedIPs = {}", self.prefix.int(*id))?;

            // Endpoint
            if let Some(endpoint) = &peer.endpoint {
                writeln!(server_config, "Endpoint = {}", endpoint)?;
            }

            // Unbound
            writeln!(unbound_config, "\n# {}", peer.name)?;

            // AAAA Record
            writeln!(
                unbound_config,
                "local-data: '{} AAAA {}'",
                peer.name,
                self.prefix.int(*id).addr
            )?;

            // PTR Record
            writeln!(
                unbound_config,
                "local-data-ptr: '{} {}'",
                self.prefix.int(*id).addr,
                peer.name,
            )?;
        }
        Ok(())
    }
}
