#[macro_use]
extern crate hob;
use hob::prelude::*;

fn main() {
    hob::server::listen(Config {
        host: String::from("127.0.0.1"),
        port: 4001,
        events: events!{
            "greeting" => [ "hello" ],
        },
        actions: actions!{
            "hello" => subproc!("echo", "hello world"),
        },
    });
}
