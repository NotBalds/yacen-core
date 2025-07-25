#[cfg(feature = "grpc-api")]
pub mod api {
    pub const VERSION: &str = "v1_1";
    tonic::include_proto!("yacen_api.v1_1");
}

pub mod impls;
pub mod models;
#[cfg(feature = "security")]
pub mod security;
