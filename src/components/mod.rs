/**
 * See [3.6. Calendar Components](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6)
 */
mod vcalendar;
mod vevent;
mod vfreebusy;
mod vjournal;
mod vtodo;

pub mod valarm;
pub mod vtimezone;

pub use valarm::VAlarm;
pub use vcalendar::*;
pub use vevent::*;
pub use vfreebusy::*;
pub use vjournal::*;
pub use vtimezone::VTimezone;
pub use vtodo::*;

#[derive(Debug, PartialEq)]
pub(crate) enum Component {
    Alarm(crate::VAlarm),
    Event(crate::VEvent),
    Freebusy(crate::VFreebusy),
    Journal(crate::VJournal),
    Timezone(crate::VTimezone),
    Todo(crate::VTodo),
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Components {
    Alarm,
    Event,
    Freebusy,
    Journal,
    Timezone,
    Todo,
}

impl std::fmt::Display for Components {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Alarm => "VALARM",
            Self::Event => "VEVENT",
            Self::Freebusy => "VFREEBUSY",
            Self::Journal => "VJOURNAL",
            Self::Timezone => "VTIMEZONE",
            Self::Todo => "VTODO",
        };

        f.write_str(s)
    }
}

impl From<Component> for Components {
    fn from(value: Component) -> Self {
        match value {
            Component::Alarm(_) => Self::Alarm,
            Component::Event(_) => Self::Event,
            Component::Freebusy(_) => Self::Freebusy,
            Component::Journal(_) => Self::Journal,
            Component::Timezone(_) => Self::Timezone,
            Component::Todo(_) => Self::Todo,
        }
    }
}
