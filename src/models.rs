use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    identity: Identity,
    known_identities: Vec<Identity>,
}

#[derive(Serialize, Deserialize)]
pub struct Identity {
    name: String,
    secret: [u8; 32],
}
