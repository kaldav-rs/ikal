/**
 * See [3.6.1. Event Component](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6.1)
 */
#[derive(Clone, Debug, Default, PartialEq, crate::Component)]
pub struct VEvent {
    pub dtstamp: crate::DateTime,
    pub uid: crate::Text,
    pub dtstart: crate::Date,
    pub class: Option<crate::Class>,
    pub created: Option<crate::DateTime>,
    pub description: Option<crate::Text>,
    pub geo: Option<crate::Geo>,
    pub last_modified: Option<crate::DateTime>,
    pub location: Option<crate::Text>,
    pub organizer: Option<crate::Uri>,
    pub priority: Option<u8>,
    pub sequence: Option<u32>,
    pub status: Option<crate::Status>,
    pub summary: Option<crate::Text>,
    pub transp: Option<crate::TimeTransparency>,
    pub url: Option<crate::Uri>,
    pub recurid: Option<crate::Date>,
    pub rrule: Option<crate::Recur>,
    pub dtend: Option<crate::Date>,
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
    pub alarms: Vec<crate::VAlarm>,
    #[component(ignore)]
    pub x_prop: std::collections::BTreeMap<String, crate::ContentLine>,
    #[component(ignore)]
    pub iana_prop: std::collections::BTreeMap<String, crate::ContentLine>,
}

impl VEvent {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn recurrent(&self) -> crate::iter::Recur<Self> {
        crate::iter::Recur::from(self)
    }
}

#[cfg(test)]
mod test {
    use crate as ikal;

    #[test]
    fn parse() {
        crate::test::test_files::<crate::VEvent>("events");
    }

    #[test]
    fn ser() -> crate::Result {
        let vevent = crate::vevent! {
            created: "20170209T192358",
            dtstamp: "20170209T192358",
            last_modified: "20170209T192358",
            uid: "5UILHLI7RI6K2IDRAQX7O",
            summary: "Vers",
            class: Public,
            status: Confirmed,
            dtstart: "20170209",
            dtend: "20170210",
        }?;

        let ical = crate::ser::ical(&vevent)?;

        similar_asserts::assert_eq!(
            ical,
            "BEGIN:VEVENT\r
DTSTAMP:20170209T192358\r
UID:5UILHLI7RI6K2IDRAQX7O\r
DTSTART;VALUE=DATE:20170209\r
CLASS:PUBLIC\r
CREATED:20170209T192358\r
LAST-MODIFIED:20170209T192358\r
STATUS:CONFIRMED\r
SUMMARY:Vers\r
DTEND;VALUE=DATE:20170210\r
END:VEVENT\r
"
        );

        Ok(())
    }

    #[test]
    fn macros() -> crate::Result {
        let _vevent = crate::vevent! {
            dtstamp: "20170209T192358",
            uid: "5UILHLI7RI6K2IDRAQX7O",
            dtstart: "20170209",
            class: Public,
            created: "20170209T192358",
            description: "",
            geo: {
                lat: 37.386013,
                lon: -122.08293,
            },
            last_modified: "20170209T192358",
            location: "",
            organizer: "",
            priority: 1,
            sequence: 0,
            status: Confirmed,
            summary: "Vers",
            transp: Transparent,
            recurid: "20170210",
            rrule: {
                freq: Yearly,
                interval: 1,
            },
            dtend: "20170210",
            duration: "P1Y",
            attach: [""],
            attendee: [""],
            categories: [""],
            comment: [""],
            contact: [""],
            exdate: ["20170209"],
            rstatus: [
                {
                    statcode: 2.0,
                    statdesc: "Success",
                }
            ],
            related_to: [""],
            resources: [""],
            //rdate: [],
            alarms: [
                //crate::valarm! {
                //    @email,
                //},
            ],
        }?;

        Ok(())
    }
}
