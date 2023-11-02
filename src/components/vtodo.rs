/**
 * See [3.6.2. To-Do Component](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6.2)
 */
#[derive(Clone, Default, Debug, PartialEq, crate::Component)]
pub struct VTodo {
    pub dtstamp: crate::DateTime,
    pub uid: String,
    pub class: Option<crate::Class>,
    pub completed: Option<crate::DateTime>,
    pub created: Option<crate::DateTime>,
    pub dtstart: Option<crate::DateTime>,
    pub geo: Option<crate::Geo>,
    pub last_modified: Option<crate::DateTime>,
    pub location: Option<String>,
    pub organizer: Option<String>,
    pub percent_complete: Option<u8>,
    pub priority: Option<u8>,
    pub recurid: Option<String>,
    pub sequence: Option<u32>,
    pub status: Option<crate::Status>,
    pub summary: Option<String>,
    pub url: Option<String>,
    pub rrule: Option<crate::Recur>,
    pub due: Option<crate::DateTime>,
    pub duration: Option<chrono::Duration>,
    pub attach: Vec<String>,
    pub attendee: Vec<String>,
    #[component(append)]
    pub categories: Vec<String>,
    pub comment: Vec<String>,
    pub contact: Vec<String>,
    #[component(append)]
    pub exdate: Vec<crate::DateTime>,
    pub rstatus: Vec<crate::RequestStatus>,
    pub related_to: Vec<String>,
    #[component(append)]
    pub resources: Vec<String>,
    #[component(append)]
    pub rdate: Vec<String>,
    #[component(ignore)]
    pub x_prop: std::collections::BTreeMap<String, String>,
    #[component(ignore)]
    pub iana_prop: std::collections::BTreeMap<String, String>,
}

impl VTodo {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
