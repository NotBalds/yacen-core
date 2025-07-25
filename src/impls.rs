use super::models::Config;

use bincode::{config::standard, error::DecodeError, serde::*};
impl Config {
    pub fn to_bytes(self) -> Vec<u8> {
        bincode::serde::encode_to_vec(self, bincode::config::standard()).unwrap()
    }

    pub fn from_bytes(data: &[u8]) -> Result<(Self, usize), DecodeError> {
        decode_from_slice(data, standard())
    }
}
