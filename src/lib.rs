#[cfg(feature = "grpc-api")]
pub mod api {
    pub const VERSION: &str = "2";
    tonic::include_proto!("yacen_api.v1_0");
}

pub mod impls;
pub mod models;
#[cfg(feature = "security")]
pub mod security;
