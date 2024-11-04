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

#[cfg(test)]
mod test {
    #[test]
    fn parse() {
        crate::test::test_files::<crate::VTodo>("todos");
    }

    #[test]
    fn ser() -> crate::Result {
        let vtodo = crate::VTodo {
            dtstamp: "20070313T123432Z".parse()?,
            uid: "20070313T123432Z-456553@example.com".into(),
            due: Some("20070501".parse()?),
            summary: Some("Submit Quebec Income Tax Return for 2006".into()),
            class: crate::Class::Confidential.into(),
            categories: vec!["FAMILY".into(), "FINANCE".into()],
            status: Some(crate::Status::NeedsAction),

            ..Default::default()
        };

        let ical = crate::ser::ical(&vtodo)?;

        similar_asserts::assert_eq!(
            ical,
            "BEGIN:VTODO\r
DTSTAMP:20070313T123432Z\r
UID:20070313T123432Z-456553@example.com\r
CLASS:CONFIDENTIAL\r
STATUS:NEEDS-ACTION\r
SUMMARY:Submit Quebec Income Tax Return for 2006\r
DUE;VALUE=DATE:20070501\r
CATEGORIES:FAMILY,FINANCE\r
END:VTODO\r
"
        );

        Ok(())
    }
}
