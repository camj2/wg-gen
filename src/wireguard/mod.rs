pub const WG_DEFAULT_LISTENING_PORT: u16 = 51820;
pub const WG_DEFAULT_KEEP_ALIVE: u16 = 25;
pub const WG_DEFAULT_MTU: u16 = 1420;

pub mod crypto;
pub mod net;

use anyhow::{Context, bail};

use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use net::{IpPrefix, Ipv6Prefix};

use serde::{Deserialize, Serialize};
use serde_json;

use wireguard_keys::{Privkey as PrivateKey, Secret as PresharedKey};

#[derive(Serialize, Deserialize)]
pub struct Config {
    ip: IpPrefix,

    peers: BTreeMap<u64, Peer>,
}

#[derive(Serialize, Deserialize)]
struct Peer {
    name: String,

    private_key: PrivateKey,
    preshared_key: PresharedKey,

    endpoint: Option<String>,
    listen_port: Option<u16>,
    persistent_keepalive_interval: Option<u16>,
    fwmark: Option<u32>,
    mtu: Option<u16>,
    // Thought: add dns option
}

impl Config {
    pub fn new() -> Self {
        let ip = IpPrefix::from(Ipv6Prefix::new().generate_ula().generate_network());

        let peers: BTreeMap<u64, Peer> = BTreeMap::new();

        Self { ip, peers }
    }

    pub fn add<T: ToString>(&mut self, name: T) -> anyhow::Result<()> {
        // Find lowest available entry. Bail if none left.
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

    pub fn del<T: ToString>(&mut self, name: T) -> anyhow::Result<()> {
        self.peers.retain(|_, peer| peer.name != name.to_string());

        Ok(())
    }

    pub fn from(path: &Path) -> anyhow::Result<Self> {
        let file_contents = fs::read_to_string(&path)?;

        let config: Self = serde_json::from_str(&file_contents)?;

        Ok(config)
    }

    pub fn write(&self, path: &Path) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(&self)?;

        // TO-DO: Create file handle
        fs::write(&path, &json)?;

        Ok(())
    }

    // Build WireGuard config files.
    pub fn export(&self, path: &Path) -> anyhow::Result<()> {
        // First entry is considered the "server" peer, remainder are "client" peers.
        let server_entry = self
            .peers
            .first_key_value()
            .context("First peer should exist.")?;
        let server_id = *server_entry.0;
        let server = server_entry.1;

        let prefix = match &self.ip {
            IpPrefix::V4(ip) => bail!("IPv4: {}: not supported for now.", ip),
            IpPrefix::V6(ip) => ip,
        };

        // TO-DO: Check for duplicate names! Thought: Use BTreeMap for Unbound config? Reverse HashMap?

        // Create export directory
        fs::create_dir_all(&path)?;

        // Server WireGuard config file
        let mut server_config = File::create(path.join("wireguard.conf"))?;

        // [Interface]
        writeln!(server_config, "[Interface]")?;

        // PrivateKey
        writeln!(server_config, "PrivateKey = {}", server.private_key)?;

        // Address
        writeln!(server_config, "Address = {}", prefix.int(server_id))?;

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
        // TO-DO: rename to unbound_buf and write depending whether or not AAAA or A, IpPrefixDual ?

        // Unbound
        writeln!(unbound_config, "# {}", server.name)?;

        // AAAA Record
        writeln!(
            unbound_config,
            "local-data: '{} AAAA {}'",
            server.name,
            prefix.int(server_id).addr
        )?;

        // PTR Record
        writeln!(
            unbound_config,
            "local-data-ptr: '{} {}'",
            prefix.int(server_id).addr,
            server.name
        )?;

        for (id, peer) in &self.peers {
            let peer_id = *id;

            // Thought: Generate "server" peer here instead of above
            if peer_id == server_id {
                continue;
            }

            let name = format!("{}-{}.conf", peer_id, peer.name);

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
            writeln!(config, "Address = {}", prefix.int(peer_id))?;

            // DNS
            writeln!(config, "DNS = {}", prefix.int(server_id).addr)?;

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
            writeln!(config, "AllowedIPs = {}", prefix)?;

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
            writeln!(server_config, "AllowedIPs = {}", prefix.int(peer_id))?;

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
                prefix.int(peer_id).addr
            )?;

            // PTR Record
            writeln!(
                unbound_config,
                "local-data-ptr: '{} {}'",
                prefix.int(peer_id).addr,
                peer.name,
            )?;
        }
        Ok(())
    }
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
