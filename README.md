
# Hob (unstable)

Hob is a self hosted automation manager which facilitates listening for
generic webhook events and propagating them to configured actions and events.

The exact Developer and end-user experience *(api or otherwise)* is still
very much in Flux, so feel free to open any desired issues to influence the
design.

With that said, the first versions mainly focus on overall API design and will
not have much *(if any)* ability to be configured via the Web UI.

## API Design

Hob is implemented as a series of events and actions tied together by
a configuration. Events are triggered by sending an HTTP request to an endpoint
*(example: /events/restart_machine)*. Once triggered, the event will call one or
more actions based on how they are configured.

Both Events and Actions will *(as development progresses)* be flexible to support
multiple methods of communication for simple and advanced event triggers and action
handlers. Eg, an action handler can be written by curling
*/actions/restart_machine/longpoll* on a loop. No complicated RPC framework is
needed.
