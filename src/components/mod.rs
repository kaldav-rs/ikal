/**
 * See [3.6. Calendar Components](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6)
 */

mod vcalendar;
mod vevent;
mod vtodo;

pub use vcalendar::*;
pub use vevent::*;
pub use vtodo::*;

#[derive(Debug, PartialEq)]
pub enum Component {
    Event(crate::VEvent),
    Todo(crate::VTodo),
}

macro_rules! get {
    ($name:ident => $ty:ty) => {
        pub fn $name(&self) -> &$ty {
            match self {
                Self::Event(event) => &event.$name,
                Self::Todo(todo) => &todo.$name,
            }
        }

    }
}

impl Component {
    get!(dtstamp => crate::DateTime);
    get!(uid => String);
    get!(summary => Option<String>);

    pub fn into_event(self) -> crate::VEvent {
        match self {
            Self::Event(event) => event,
            _ => panic!(),
        }
    }

    pub fn into_todo(self) -> crate::VTodo {
        match self {
            Self::Todo(todo) => todo,
            _ => panic!(),
        }
    }
}
