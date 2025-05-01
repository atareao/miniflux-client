mod matrix;
mod miniflux;

pub use matrix::MatrixClient;
pub use miniflux::MinifluxClient;
pub type CustomError = Box<dyn std::error::Error>;
