pub mod subproc;

use std::collections::BTreeMap;
use iron::typemap;
use events::Events;
use error::Error;
use actions::subproc::SubprocAction;

pub enum Action {
    Subproc(SubprocAction),
}

pub type Actions = BTreeMap<String, Action>;


// TODO(leeola): Convert String to a Golang-like ReaderCloser so Responses from
// actions can be streamed.
pub type ActionResult = Result<String, Error>;

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

    pub fn trigger_event(&self, event_key: &str) -> ActionResult {
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

        self.handle_action(&action)
    }

    fn handle_action(&self, action: &Action) -> ActionResult {
        Err(Error::NotImplemented)
    }
}

impl typemap::Key for Linker {
    type Value = Linker;
}
