#[derive(Debug)]
pub enum Error {
    NotImplemented,
    EventNotFound(String),
    ActionNotFound(String),
}
