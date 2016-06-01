#[macro_use]
extern crate log;

extern crate rustc_serialize;
extern crate threadpool;
extern crate nanomsg;
extern crate num_cpus;

pub mod logger;
pub mod core;
pub mod gateway;
pub mod messages;
pub mod rpc;

/// unicorn version
pub const VERSION: [i32; 3] = [0, 0, 1];

/// Return version as a formatted string in semver format
pub fn get_version() -> String { format!("{:?}.{:?}.{:?}", VERSION[0], VERSION[1], VERSION[2]) }
