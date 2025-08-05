use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Identity {
    name: String,
    secret: [u8; 32],
    known_identities: HashMap<[u8; 32], String>,
}
