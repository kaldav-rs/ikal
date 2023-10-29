/**
 * See [3.6. Calendar Components](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6)
 */

mod vevent;
mod vtodo;

pub use vevent::*;
pub use vtodo::*;

#[derive(Debug, PartialEq)]
pub enum Content {
    Empty,
    Event(crate::VEvent),
    Todo(crate::VTodo),
}
