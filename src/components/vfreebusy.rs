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
    use crate as ikal;

    #[test]
    fn parse() {
        crate::test::test_files::<crate::VFreebusy>("freebusy");
    }

    #[test]
    fn ser() -> crate::Result {
        let vfreebusy = crate::vfreebusy! {
            dtstamp: "19970901T083000Z",
            uid: "19970901T082949Z-FA43EF@example.com",
            organizer: "mailto:jane_doe@example.com",
            attendee: ["mailto:john_public@example.com"],
            dtstart: "19971015T050000Z",
            dtend: "19971016T050000Z",
        }?;

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

    #[test]
    fn macros() -> crate::Result {
        let _vfreebusy = crate::vfreebusy! {
            dtstamp: "19970901T083000Z",
            uid: "19970901T082949Z-FA43EF@example.com",
            contact: "",
            dtstart: "19971015T050000Z",
            dtend: "19971016T050000Z",
            organizer: "mailto:jane_doe@example.com",
            url: "",
            attendee: ["mailto:john_public@example.com"],
            comment: [""],
        }?;

        Ok(())
    }
}
