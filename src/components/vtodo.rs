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
    use crate as ikal;

    #[test]
    fn parse() {
        crate::test::test_files::<crate::VTodo>("todos");
    }

    #[test]
    fn ser() -> crate::Result {
        let vtodo = crate::vtodo! {
            dtstamp: "20070313T123432Z",
            uid: "20070313T123432Z-456553@example.com",
            due: "20070501",
            summary: "Submit Quebec Income Tax Return for 2006",
            class: Confidential,
            categories: ["FAMILY", "FINANCE"],
            status: NeedsAction,
        }?;

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

    #[test]
    fn macros() -> crate::Result {
        let _vtodo = crate::vtodo! {
            dtstamp: "20070313T123432Z",
            uid: "20070313T123432Z-456553@example.com",
            class: Confidential,
            completed: "20070313T123432Z",
            created: "20070313T123432Z",
            dtstart: "20070501",
            geo: {
                lat: 0.,
                lon: 0.,
            },
            last_modified: "20070313T123432Z",
            location: "",
            organizer: "",
            percent_complete: 100,
            priority: 10,
            recurid: "20070313T123432Z",
            sequence: 0,
            status: NeedsAction,
            summary: "",
            url: "",
            rrule: {
                freq: Yearly,
                interval: 1,
            },
            due: "20070501",
            duration: "-PT10M",
            attach: [""],
            attendee: [""],
            categories: ["FAMILY", "FINANCE"],
            comment: [""],
            contact: [""],
            exdate: ["20070501"],
            rstatus: [
                {
                    statcode: 2.8,
                    statdesc: "Success",
                    extdata: "",
                }
            ],
            related_to: [""],
            resources: [""],
            //rdate: [""],
        }?;

        Ok(())
    }
}
