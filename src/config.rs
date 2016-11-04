use std::collections::BTreeMap;
use events::Events;
use actions::Actions;

pub struct Config {
    // default_events: bool
    // default_actions: bool
    pub host: String,
    pub port: u16,
    pub events: Events,
    pub actions: Actions,
}
