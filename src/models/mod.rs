mod matrix;
mod miniflux;
mod telegram;
mod model;

pub use telegram::TelegramClient;
pub use matrix::MatrixClient;
pub use miniflux::MinifluxClient;
pub use model::Model;
pub type CustomError = Box<dyn std::error::Error>;
