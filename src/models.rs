use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    user: User,
    settings: Settings,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    secret: [u8; 32],
}

#[derive(Serialize, Deserialize)]
pub struct Settings {}
