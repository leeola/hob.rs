use std::collections::BTreeMap;

pub type Events = BTreeMap<String, String>;

/// EventRequest is a series of values to be provided to an Action implementor.
///
/// The values of this struct serve to provide *(most of)* the data provided to
/// the event handler during the http request.
pub struct EventRequest {
    // TODO(leeola): Convert String to a Golang-like ReaderCloser so Requests can
    // be streamed.
    pub body: String,
}

pub struct EventResponse {
    // TODO(leeola): Convert String to a Golang-like ReaderCloser so Responses from
    // actions can be streamed.
    pub body: String,
}
