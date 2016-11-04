// use iron::prelude::{IronResult, Request, Response};
use iron::prelude::*;
use iron::status;
use router::Router;
use persistent::Read;
use error::Error;
use actions::Linker;

pub fn handler(req: &mut Request) -> IronResult<Response> {
    // TODO(leeola): remove unwraps asap. But atm, it's a late night and i'm
    // hacking :)
    let arc = req.get::<Read<Linker>>().unwrap();
    let mut linker = arc.as_ref();

    let router = req.extensions.get::<Router>().unwrap();
    let event = router.find("event").unwrap();

    match linker.trigger_event(event) {
        Ok(res) => Ok(Response::with((status::Ok, res))),
        Err(Error::NotImplemented) => Ok(Response::with((status::NotFound, "not implemented"))),
        Err(Error::EventNotFound(event)) => {
            Ok(Response::with((status::NotFound, format!("event '{}' not found", event))))
        }
        Err(Error::ActionNotFound(action)) => {
            Ok(Response::with((status::InternalServerError,
                               format!("for event '{}', action '{}' is not defined", event, action))))
        }
    }
}
