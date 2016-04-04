extern crate rustc_serialize;

pub mod core;
pub mod gateway;
pub mod messages;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
