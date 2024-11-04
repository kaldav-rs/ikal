/**
 * See [3.6. Calendar Components](https://datatracker.ietf.org/doc/html/rfc5545#section-3.4)
 */
#[derive(Clone, Debug, Default, PartialEq, crate::Component)]
pub struct VCalendar {
    pub prodid: crate::Text,
    pub version: crate::Text,
    pub calscale: Option<crate::Text>,
    pub method: Option<crate::Text>,
    #[component(ignore)]
    pub alarms: Vec<crate::VAlarm>,
    #[component(ignore)]
    pub events: Vec<crate::VEvent>,
    #[component(ignore)]
    pub freebusy: Vec<crate::VFreebusy>,
    #[component(ignore)]
    pub journals: Vec<crate::VJournal>,
    #[component(ignore)]
    pub todo: Vec<crate::VTodo>,
    #[component(ignore)]
    pub timezones: Vec<crate::VTimezone>,
    #[component(ignore)]
    pub x_prop: std::collections::BTreeMap<String, crate::ContentLine>,
    #[component(ignore)]
    pub iana_prop: std::collections::BTreeMap<String, crate::ContentLine>,
}

impl VCalendar {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn parse() {
        crate::test::test_files::<crate::VCalendar>("calendars")
    }

    #[test]
    fn ser() -> crate::Result {
        let vcalendar = crate::VCalendar {
            version: "2.0".into(),
            prodid: "-//hacksw/handcal//NONSGML v1.0//EN".into(),
            events: vec![crate::VEvent {
                uid: "19970610T172345Z-AF23B2@example.com".into(),
                dtstamp: "19970610T172345Z".parse()?,
                dtstart: "19970714T170000Z".parse()?,
                dtend: "19970715T040000Z".parse().ok(),
                summary: Some("Bastille Day Party".into()),

                ..Default::default()
            }],

            ..Default::default()
        };

        let ical = crate::ser::ical(&vcalendar)?;

        similar_asserts::assert_eq!(
            ical,
            "BEGIN:VCALENDAR\r
PRODID:-//hacksw/handcal//NONSGML v1.0//EN\r
VERSION:2.0\r
BEGIN:VEVENT\r
DTSTAMP:19970610T172345Z\r
UID:19970610T172345Z-AF23B2@example.com\r
DTSTART:19970714T170000Z\r
SUMMARY:Bastille Day Party\r
DTEND:19970715T040000Z\r
END:VEVENT\r
END:VCALENDAR\r
"
        );

        Ok(())
    }
}
