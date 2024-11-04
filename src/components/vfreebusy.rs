/**
 * See [3.6.4. Free/Busy Component](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6.4)
 */
#[derive(Clone, Debug, Default, PartialEq, crate::Component)]
pub struct VFreebusy {
    pub dtstamp: crate::DateTime,
    pub uid: crate::Text,
    pub contact: Option<crate::Text>,
    pub dtstart: Option<crate::Date>,
    pub dtend: Option<crate::Date>,
    pub organizer: Option<crate::Uri>,
    pub url: Option<crate::Uri>,
    pub attendee: Vec<crate::Uri>,
    pub comment: Vec<crate::Text>,
    #[component(append)]
    pub freebusy: Vec<crate::Period>,
    pub rstatus: Vec<crate::RequestStatus>,
    #[component(ignore)]
    pub x_prop: std::collections::BTreeMap<String, crate::ContentLine>,
    #[component(ignore)]
    pub iana_prop: std::collections::BTreeMap<String, crate::ContentLine>,
}
impl VFreebusy {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn parse() {
        crate::test::test_files::<crate::VFreebusy>("freebusy");
    }

    #[test]
    fn ser() -> crate::Result {
        let vfreebusy = crate::VFreebusy {
            dtstamp: "19970901T083000Z".parse()?,
            uid: "19970901T082949Z-FA43EF@example.com".into(),
            organizer: Some("mailto:jane_doe@example.com".into()),
            attendee: vec!["mailto:john_public@example.com".into()],
            dtstart: "19971015T050000Z".parse().ok(),
            dtend: "19971016T050000Z".parse().ok(),

            ..Default::default()
        };

        let ical = crate::ser::ical(&vfreebusy)?;

        similar_asserts::assert_eq!(
            ical,
            "BEGIN:VFREEBUSY\r
DTSTAMP:19970901T083000Z\r
UID:19970901T082949Z-FA43EF@example.com\r
DTSTART:19971015T050000Z\r
DTEND:19971016T050000Z\r
ORGANIZER:mailto:jane_doe@example.com\r
ATTENDEE:mailto:john_public@example.com\r
END:VFREEBUSY\r
"
        );

        Ok(())
    }
}
