/**
 * See [3.6.4. Free/Busy Component](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6.4)
 */
#[derive(Clone, Debug, Default, PartialEq, crate::Component)]
pub struct VFreebusy {
    pub dtstamp: crate::DateTime,
    pub uid: crate::Text,
    pub contact: Option<crate::Text>,
    pub dtstart: crate::Date,
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
}
