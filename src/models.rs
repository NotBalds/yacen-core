use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const KEYPAIR_BYTES_LENGTH: usize = 83;

#[derive(Serialize, Deserialize)]
pub struct Identity {
    pub name: String,
    pub secret: Vec<u8>,
    pub known_identities: HashMap<[u8; 32], String>,
}
