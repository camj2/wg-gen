// TO-DO: Write own crypto generator instead of using external crate.
// https://www.reddit.com/r/WireGuard/comments/zsjrsx/technical_difference_between_psk_and_private_key/
use wireguard_keys::{Privkey as PrivateKey, Pubkey as PublicKey, Secret as PresharedKey};

// Asymmetric
pub struct KeyPair {
    pub private: String,
    pub public: String,
}

// Symmetric
pub struct KeySecret {
    pub secret: String,
}

// pub enum Key {
//     PrivateKey(String),
//     PublicKey(String),
//     PresharedKey(String),
// }

impl KeyPair {
    pub fn generate() -> Self {
        let private = PrivateKey::generate();
        let public = private.pubkey();

        Self {
            private: private.to_string(),
            public: public.to_string(),
        }
    }
}

impl KeySecret {
    pub fn generate() -> Self {
        let secret = PresharedKey::generate().to_string();

        Self {
            secret: secret.to_string(),
        }
    }
}
