pub mod api {
    pub const VERSION: &str = "v2_1";
    tonic::include_proto!("yacen_api.v2_1");
}

pub mod impls;
pub mod models;
pub mod security;
