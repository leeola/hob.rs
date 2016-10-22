use std::collections::BTreeMap;

pub struct Config {
    // default_events: bool
    // default_actions: bool
    pub host: String,
    pub port: u16,
    pub actions: Actions,
    pub events: Events,
}

type Actions = BTreeMap<String, Action>;

type Events = BTreeMap<String, Event>;

pub struct Event {
    pub actions: Vec<String>,
}

pub enum Action {
    Subproc(SubprocAction),
}

pub struct SubprocAction {
    pub bin: String,
    pub args: Vec<String>,
}
