pub mod subproc;

use std::collections::BTreeMap;
use iron::typemap;
use events::{Events, EventRequest, EventResponse};
use error::Error;

pub trait Action: Send + Sync {
    fn act(&self, EventRequest) -> ActionResult;
}

pub type Actions = BTreeMap<String, Box<Action>>;

pub type ActionResult = Result<EventResponse, Error>;

pub struct Linker {
    events: Events,
    actions: Actions,
}

impl Linker {
    pub fn new(events: Events, actions: Actions) -> Linker {
        Linker {
            events: events,
            actions: actions,
        }
    }

    pub fn trigger_event(&self, event_key: &str, req: EventRequest) -> ActionResult {
        let action_key = match self.events.get(event_key) {
            Some(action_key) => action_key,
            None => {
                debug!("requested event cannot be found. event:{}", event_key);
                return Err(Error::EventNotFound(String::from(event_key)));
            }
        };

        let action = match self.actions.get(&*action_key) {
            Some(action) => action,
            None => {
                debug!("requested action cannot be found. event:{}, action:{}",
                       event_key,
                       action_key);
                return Err(Error::ActionNotFound(action_key.clone()));
            }
        };

        action.act(req)
    }
}

// Implementing Key for the linker allows it to be accessed by the Iron extension.
impl typemap::Key for Linker {
    type Value = Linker;
}
