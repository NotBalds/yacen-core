pub mod api {
    pub const VERSION: &str = "v2_2";
    tonic::include_proto!("yacen_api.v2_2");
}

pub mod impls;
pub mod models;
pub mod security;
