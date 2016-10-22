use iron::prelude::*;
use iron::status;
use router::Router;
use mount::Mount;
use routes;
use config::Config;

pub fn listen(config: Config) {
    trace!("constructing routes for the api versions");
    let mut router = Mount::new();
    router.mount("/", routes::v0::routes());
    router.mount("/v0", routes::v0::routes());

    trace!("running server");
    Iron::new(router).http(&*format!("{}:{}", config.host, config.port));
}
