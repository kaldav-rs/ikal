/**
 * See [3.6.3. Journal Component](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6.3)
 */
#[derive(Clone, Debug, Default, PartialEq, crate::Component)]
pub struct VJournal {
    pub dtstamp: crate::DateTime,
    pub uid: crate::Text,
    pub class: Option<crate::Class>,
    pub created: Option<crate::DateTime>,
    pub dtstart: crate::Date,
    pub last_modified: Option<crate::DateTime>,
    pub organizer: Option<crate::Uri>,
    pub recurid: Option<crate::Date>,
    pub sequence: Option<u32>,
    pub status: Option<crate::Status>,
    pub summary: Option<crate::Text>,
    pub url: Option<crate::Uri>,
    pub rrule: Option<crate::Recur>,
    pub attach: Vec<crate::Text>,
    pub attendee: Vec<crate::Uri>,
    #[component(append)]
    pub categories: Vec<crate::Text>,
    pub comment: Vec<crate::Text>,
    pub contact: Vec<crate::Text>,
    pub description: Vec<crate::Text>,
    #[component(append)]
    pub exdate: Vec<crate::Date>,
    pub related_to: Vec<crate::Text>,
    pub rdate: Vec<crate::RDate>,
    pub rstatus: Vec<crate::RequestStatus>,
    #[component(ignore)]
    pub x_prop: std::collections::BTreeMap<String, crate::ContentLine>,
    #[component(ignore)]
    pub iana_prop: std::collections::BTreeMap<String, crate::ContentLine>,
}

impl VJournal {
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
        crate::test::test_files::<crate::VJournal>("journals");
    }

    #[test]
    fn ser() -> crate::Result {
        let vjournal = crate::vjournal! {
            uid: "19970901T130000Z-123405@example.com",
            dtstart: "19970317",
            dtstamp: "19970901T130000Z",
            summary: "Staff meeting minutes",
            description: ["1. Staff meeting: Participants include Joe, Lisa, and Bob. Aurora project plans were reviewed. There is currently no budget reserves for this project. Lisa will escalate to management. Next meeting on Tuesday.
2. Telephone Conference: ABC Corp. sales representative called to discuss new printer. Promised to get us a demo by Friday.
3. Henry Miller (Handsoff Insurance): Car was totaled by tree. Is looking into a loaner car. 555-2323 (tel)."],
        }?;

        let ical = crate::ser::ical(&vjournal)?;

        similar_asserts::assert_eq!(
            ical,
            "BEGIN:VJOURNAL\r
DTSTAMP:19970901T130000Z\r
UID:19970901T130000Z-123405@example.com\r
DTSTART;VALUE=DATE:19970317\r
SUMMARY:Staff meeting minutes\r
DESCRIPTION:1. Staff meeting: Participants include Joe\\, Lisa\\, and Bob. Au\r
 rora project plans were reviewed. There is currently no budget reserves for\r
  this project. Lisa will escalate to management. Next meeting on Tuesday.\\n\r
 2. Telephone Conference: ABC Corp. sales representative called to discuss n\r
 ew printer. Promised to get us a demo by Friday.\\n3. Henry Miller (Handsoff\r
  Insurance): Car was totaled by tree. Is looking into a loaner car. 555-232\r
 3 (tel).\r
END:VJOURNAL\r
"
        );

        Ok(())
    }

    #[test]
    fn macros() -> crate::Result {
        let _vjournal = crate::vjournal! {
            dtstamp: "19970901T130000Z",
            uid: "19970901T130000Z-123405@example.com",
            class: Custom("Custom".to_string()),
            created: "19970901T130000Z",
            dtstart: "19970317",
            last_modified: "19970901T130000Z",
            organizer: "",
            recurid: "19970317",
            sequence: 0,
            status: NeedsAction,
            summary: "Staff meeting minutes",
            url: "",
            rrule: {
                freq: Yearly,
                interval: 1,
            },
            attach: [""],
            attendee: [""],
            categories: [""],
            comment: [""],
            contact: [""],
            description: [""],
            exdate: ["19970317"],
            related_to: [""],
            rstatus: [
                {
                    statcode: 2.0,
                    statdesc: "Success",
                }
            ],
        }?;

        Ok(())
    }
}
