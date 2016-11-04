#[macro_use]
extern crate hob;
use hob::prelude::*;

fn main() {
    hob::server::listen(Config {
        host: String::from("127.0.0.1"),
        port: 4001,
        events: events!{
            "hello" => "say_hello"
        },
        actions: actions!{
            "say_hello" => subproc!("echo", "hello world")
        },
    });
}
