use iron::prelude::{IronResult, Request, Response};
use iron::status;
use router::Router;

pub fn handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::NotImplemented, "not implemented")))
}
