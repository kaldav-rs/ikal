/**
 * See [3.6.3. Journal Component](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6.3)
 */
#[derive(Debug, Default, PartialEq, crate::Component)]
pub struct VJournal {
    pub dtstamp: crate::DateTime,
    pub uid: crate::Text,
    pub class: Option<crate::Class>,
    pub created: Option<crate::DateTime>,
    pub dtstart: crate::Date,
    pub last_modified: Option<crate::DateTime>,
    pub organizer: Option<crate::Text>,
    pub recurid: Option<crate::Date>,
    pub sequence: Option<u32>,
    pub status: Option<crate::Status>,
    pub summary: Option<crate::Text>,
    pub url: Option<crate::Text>,
    pub rrule: Option<crate::Recur>,
    pub attach: Vec<crate::Text>,
    pub attendee: Vec<crate::Text>,
    #[component(append)]
    pub categories: Vec<crate::Text>,
    pub comment: Vec<crate::Text>,
    pub contact: Vec<crate::Text>,
    pub description: Vec<crate::Text>,
    #[component(append)]
    pub exdate: Vec<crate::Date>,
    pub related_to: Vec<crate::Text>,
    #[component(append)]
    pub rdate: Vec<crate::Date>,
    pub rstatus: Vec<crate::RequestStatus>,
    #[component(ignore)]
    pub x_prop: std::collections::BTreeMap<String, crate::ContentLine>,
    #[component(ignore)]
    pub iana_prop: std::collections::BTreeMap<String, crate::ContentLine>,
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
