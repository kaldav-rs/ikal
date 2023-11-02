/**
 * See [3.6.1. Event Component](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6.1)
 */
#[derive(Clone, Debug, Default, PartialEq, crate::Component)]
pub struct VEvent {
    pub dtstamp: crate::DateTime,
    pub uid: String,
    pub dtstart: crate::DateTime,
    pub class: Option<crate::Class>,
    pub created: Option<crate::DateTime>,
    pub description: Option<String>,
    pub geo: Option<crate::Geo>,
    pub last_modified: Option<crate::DateTime>,
    pub location: Option<String>,
    pub organizer: Option<String>,
    pub priority: Option<u8>,
    pub sequence: Option<u32>,
    pub status: Option<crate::Status>,
    pub summary: Option<String>,
    pub transp: Option<crate::TimeTransparency>,
    pub url: Option<String>,
    pub recurid: Option<String>,
    pub rrule: Option<crate::Recur>,
    pub dtend: Option<crate::DateTime>,
    pub duration: Option<chrono::Duration>,
    pub attach: Vec<String>,
    pub attendee: Vec<String>,
    #[component(append)]
    pub categories: Vec<String>,
    pub comment: Vec<String>,
    pub contact: Vec<String>,
    #[component(append)]
    pub exdate: Vec<crate::DateTime>,
    pub rstatus: Vec<String>,
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

impl VEvent {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn parse() {
        crate::test::test_files::<crate::VEvent>("events");
    }
}
