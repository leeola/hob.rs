#[macro_use]
extern crate log;
extern crate iron;
extern crate router;
extern crate mount;

mod macros;
pub mod config;
pub mod handlers;
pub mod routes;
pub mod server;

pub mod prelude {
    pub use super::server;
    pub use super::config::{Config, Event, Action, SubprocAction};
}
