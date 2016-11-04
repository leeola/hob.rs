use iron::prelude::*;
use iron::status;
use router::Router;
use mount::Mount;
use routes;
use persistent;
use config::Config;
use actions;

pub fn listen(config: Config) {
    trace!("constructing routes for the api versions");
    let mut router = Mount::new();
    router.mount("/", routes::v0::routes());
    router.mount("/v0", routes::v0::routes());

    let mut chain = Chain::new(router);
    let linker = actions::Linker::new(config.events, config.actions);
    chain.link(persistent::Read::<actions::Linker>::both(linker));

    trace!("running server");
    Iron::new(chain).http(&*format!("{}:{}", config.host, config.port));
}
