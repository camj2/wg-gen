// #[derive(Serialize, Deserialize)]
// pub struct Peer {
//     // Config (crate)
//     peer: u64,
//     name: String,

//     // Interface (wg)
//     private_key: String,        // PrivateKey
//     listen_port: Option<u16>,   // ListenPort
//     firewall_mark: Option<u32>, // FwMark

//     // Interface (wg-quick)
//     address: Ipv6Prefix,   // Address
//     dns: Ipv6Addr,         // DNS
//     mtu: Option<u16>,      // MTU
//     table: Option<String>, // Table (possible values: "off", "auto", "<table>")
//     hooks: Option<Hooks>,  //
//     save: Option<bool>,    // SaveConfig

//     // Peer (wg)
//     public_key: String,       // PublicKey
//     preshared_key: String,    // PresharedKey
//     allowed_ips: Ipv6Prefix,  // AllowedIPs
//     endpoint: Option<String>, // Endpoint
//     keep_alive: Option<u16>,  // PersistentKeepalive
// }

// #[derive(Serialize, Deserialize)]
// pub struct Hooks {
//     pre_up: Option<Command>,    // PreUp
//     post_up: Option<Command>,   // PostUp
//     pre_down: Option<Command>,  // PreDown
//     post_down: Option<Command>, // PostDown
// }
