#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate iron;
extern crate router;
extern crate mount;

mod config;
mod handlers;
mod routes;
mod server;

use clap::{Arg, App};

fn main() {
    // TODO(leeola): use a random 127.* range and random port
    let matches = App::new("hob")
        .version(crate_version!())
        .author("Lee Olayvar <http://leeo.la>")
        .about("A hob for your home")
        .arg(Arg::with_name("host")
            .long("host")
            .help("The host to bind to (default: localhost)")
            .value_name("HOST")
            .takes_value(true))
        .arg(Arg::with_name("port")
            .long("port")
            .help("The port to bind to (default: 4000)")
            .value_name("PORT")
            .takes_value(true))
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .get_matches();

    let host = matches.value_of("host").unwrap_or("127.0.0.1");
    let port = matches.value_of("port").unwrap_or("4000");

    server::listen(host, port);
}
