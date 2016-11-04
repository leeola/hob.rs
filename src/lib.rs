#[macro_use]
extern crate log;
extern crate iron;
extern crate mount;
extern crate persistent;
extern crate router;

mod macros;
pub mod actions;
pub mod config;
pub mod error;
pub mod events;
pub mod handlers;
pub mod routes;
pub mod server;

pub mod prelude {
    pub use super::server;
    pub use super::config::Config;
    pub use super::actions::subproc::SubprocAction;
    pub use super::events::Events;
    pub use super::actions::{Action, Actions};
    pub use super::error::Error;
}
