use actions::Action;
use actions::subproc::SubprocAction;

#[macro_export]
macro_rules! events {
    // match the trailing comma for a nice UX
    ( $($event:expr => $action:expr,)+ ) => {
        events!($($event => $action),+)
    };

    ( $($event:expr => $action:expr),+ ) => {
        {
            let mut _events = ::std::collections::BTreeMap::new();
            $(
                _events.insert(String::from($event), String::from($action));
            )*
            _events
        }
    };

    // NOTE(leeola): Old multi-action vector format.
    // Likely this syntax will be supported in the near future resulting in a
    // SyncMultiAction (fakename) or similar.
    //
    // // matching ], allows us to use a trailing comma in the macro.
    // ( $($event:expr => [ $($action:expr),* ], ),+ ) => {
    //     events!($($event => [$($action),*]),+)
    // };
    //
    // ( $($event:expr => [ $($action:expr),* ] ),+ ) => {
    //     {
    //         let mut _events = ::std::collections::BTreeMap::new();
    //         $(
    //             {
    //                 let mut _actions = Vec::new();
    //                 $(
    //                     _actions.push(String::from($action));
    //                 )*
    //                 _events.insert(String::from($event), Event { actions: _actions })
    //             };
    //         )*
    //         _events
    //     }
    // };
}

#[macro_export]
macro_rules! actions {
    // match the trailing comma for a nice UX
    ( $($key:expr => $value:expr,)+ ) => {
        actions!($($key => $value),+)
    };

    ( $($key:expr => $value:expr),* ) => {
        {
            let mut _map = ::std::collections::BTreeMap::new();
            $(
                _map.insert(String::from($key), $value);
            )*
            _map
        }
    };
}

#[macro_export]
macro_rules! subproc {
    ( $bin:expr, $( $arg:expr ),+ ) => {
        {
            let mut _args = Vec::new();

            $(
                _args.push(String::from($arg));
            )*

            Box::new(SubprocAction{
                bin: String::from($bin),
                args: _args,
            }) as Box<Action>
        }
    };
}
