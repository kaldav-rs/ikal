/**
 * See [3.6. Calendar Components](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6)
 */
pub mod vtimezone;

mod vcalendar;
mod vevent;
mod vtodo;

pub use vcalendar::*;
pub use vevent::*;
pub use vtimezone::VTimezone;
pub use vtodo::*;

#[derive(Debug, PartialEq)]
pub(crate) enum Component {
    Event(crate::VEvent),
    Timezone(crate::VTimezone),
    Todo(crate::VTodo),
}
