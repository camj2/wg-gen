// https://doc.rust-lang.org/stable/std/io/
// https://doc.rust-lang.org/stable/std/path/
// https://doc.rust-lang.org/std/env/
// https://doc.rust-lang.org/std/fs/
// https://doc.rust-lang.org/std/fmt/trait.Display.html

// use crate::wireguard::prelude;
use crate::wireguard::net::*;
use crate::wireguard::{Config, Peer};

use wireguard_keys::{Privkey as PrivateKey, Pubkey as PublicKey, Secret as PresharedKey};

use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path;
use std::path::PathBuf;
use std::process;

use serde::{Deserialize, Serialize};
use serde_json;

impl Config {
    pub fn write(&self, config: PathBuf) {
        let json = serde_json::to_string_pretty(&self).unwrap();

        fs::write("wireguard.json", &json);

        // let mut buf = File::create("wireguard.json").unwrap();

        // buf.write_all(&json).unwrap();

        // eprintln!("{}", json);

        // fs::write("wireguard.json", &json).unwrap();
    }

    // pub fn export(&self, config: PathBuf, output: PathBuf) {
    //     if path.exists() {
    //         eprintln!("path already exists");
    //         process::exit(1);
    //     }

    //     for peer in self.peers {

    //     }
    // }
}
