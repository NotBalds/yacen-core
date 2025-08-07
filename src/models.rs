use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const KEYPAIR_BYTES_LENGTH: usize = 83;

#[derive(Serialize, Deserialize)]
pub struct Identity {
    name: String,
    secret: [u8; KEYPAIR_BYTES_LENGTH],
    known_identities: HashMap<[u8; 32], String>,
}
