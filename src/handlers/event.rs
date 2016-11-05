use std::io::Read as IoRead;
// use iron::prelude::{IronResult, Request, Response};
use iron::prelude::*;
use iron::status;
use router::Router;
use persistent::Read;
use error::Error;
use actions::Linker;
use events::EventRequest;

pub fn handler(req: &mut Request) -> IronResult<Response> {
    // TODO(leeola): remove unwraps asap. But atm, it's a late night and i'm
    // hacking :)
    let arc = req.get::<Read<Linker>>().unwrap();
    let mut linker = arc.as_ref();

    let router = req.extensions.get::<Router>().unwrap();
    let event = router.find("event").unwrap();

    // compose our event request
    let mut body = String::new();
    req.body.read_to_string(&mut body);
    let event_req = EventRequest { body: body };

    match linker.trigger_event(event, event_req) {
        Ok(res) => Ok(Response::with((status::Ok, res.body))),
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
