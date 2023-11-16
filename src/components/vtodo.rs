/**
 * See [3.6.2. To-Do Component](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6.2)
 */
#[derive(Clone, Default, Debug, PartialEq, crate::Component)]
pub struct VTodo {
    pub dtstamp: crate::DateTime,
    pub uid: crate::Text,
    pub class: Option<crate::Class>,
    pub completed: Option<crate::DateTime>,
    pub created: Option<crate::DateTime>,
    pub dtstart: Option<crate::Date>,
    pub geo: Option<crate::Geo>,
    pub last_modified: Option<crate::DateTime>,
    pub location: Option<crate::Text>,
    pub organizer: Option<crate::Uri>,
    pub percent_complete: Option<u8>,
    pub priority: Option<u8>,
    pub recurid: Option<crate::Date>,
    pub sequence: Option<u32>,
    pub status: Option<crate::Status>,
    pub summary: Option<crate::Text>,
    pub url: Option<crate::Uri>,
    pub rrule: Option<crate::Recur>,
    pub due: Option<crate::Date>,
    pub duration: Option<chrono::Duration>,
    pub attach: Vec<crate::Text>,
    pub attendee: Vec<crate::Uri>,
    #[component(append)]
    pub categories: Vec<crate::Text>,
    pub comment: Vec<crate::Text>,
    pub contact: Vec<crate::Text>,
    #[component(append)]
    pub exdate: Vec<crate::Date>,
    pub rstatus: Vec<crate::RequestStatus>,
    pub related_to: Vec<crate::Text>,
    #[component(append)]
    pub resources: Vec<crate::Text>,
    pub rdate: Vec<crate::RDate>,
    #[component(ignore)]
    pub x_prop: std::collections::BTreeMap<String, crate::ContentLine>,
    #[component(ignore)]
    pub iana_prop: std::collections::BTreeMap<String, crate::ContentLine>,
}

impl VTodo {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
