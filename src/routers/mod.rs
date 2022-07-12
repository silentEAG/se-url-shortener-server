
mod default;
mod url_shorter;
mod jumper;
mod cors;

pub use self::default::default_handler;
pub use self::url_shorter::url_shorter_handler;
pub use self::jumper::jumper_handler;
pub use self::cors::cors_handler;