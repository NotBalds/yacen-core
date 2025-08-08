use crate::security::ed25519::ED25519_PUBLIC_KEY_LEN;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

pub const KEYPAIR_BYTES_LENGTH: usize = 83;

pub type PathToKey = HashMap<PathBuf, [u8; 32]>;

#[derive(Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub keypair: Vec<u8>,
    pub ptk: PathToKey,
}

#[derive(Serialize, Deserialize)]
pub struct Identity {
    pub local_id: String, // hex([32]bytes)
    pub name: String,
    pub public_key: [u8; ED25519_PUBLIC_KEY_LEN],
}

pub type KnownIdentities = Vec<Identity>;
