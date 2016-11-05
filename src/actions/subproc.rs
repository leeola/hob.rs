use error::Error;
use actions::{Action, ActionResult};
use events::EventRequest;

pub struct SubprocAction {
    pub bin: String,
    pub args: Vec<String>,
}

impl Action for SubprocAction {
    fn act(&self, _: EventRequest) -> ActionResult {
        Err(Error::NotImplemented)
    }
}
