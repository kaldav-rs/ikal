/**
 * See [3.6.3. Journal Component](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6.3)
 */
#[derive(Debug, Default, PartialEq, crate::Component)]
pub struct VJournal {
    pub dtstamp: crate::DateTime,
    pub uid: String,
    pub class: Option<crate::Class>,
    pub created: Option<crate::DateTime>,
    pub dtstart: crate::DateTime,
    pub last_modified: Option<crate::DateTime>,
    pub organizer: Option<String>,
    pub recurid: Option<crate::DateTime>,
    pub sequence: Option<u32>,
    pub status: Option<crate::Status>,
    pub summary: Option<String>,
    pub url: Option<String>,
    pub rrule: Option<crate::Recur>,
    pub attach: Vec<String>,
    pub attendee: Vec<String>,
    #[component(append)]
    pub categories: Vec<String>,
    pub comment: Vec<String>,
    pub contact: Vec<String>,
    pub description: Vec<String>,
    #[component(append)]
    pub exdate: Vec<crate::DateTime>,
    pub related_to: Vec<String>,
    #[component(append)]
    pub rdate: Vec<crate::DateTime>,
    pub rstatus: Vec<crate::RequestStatus>,
    #[component(ignore)]
    pub x_prop: std::collections::BTreeMap<String, String>,
    #[component(ignore)]
    pub iana_prop: std::collections::BTreeMap<String, String>,
}

impl VJournal {
    fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn parse() {
        crate::test::test_files::<crate::VJournal>("journals");
    }
}
