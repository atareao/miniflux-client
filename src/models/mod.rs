mod matrix;
mod miniflux;
mod model;

pub use matrix::MatrixClient;
pub use miniflux::MinifluxClient;
pub use model::Model;
pub type CustomError = Box<dyn std::error::Error>;
