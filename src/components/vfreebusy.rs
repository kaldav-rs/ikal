/**
 * See [3.6.4. Free/Busy Component](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6.4)
 */
#[derive(Clone, Debug, Default, PartialEq, crate::Component)]
pub struct VFreebusy {
    pub dtstamp: crate::DateTime,
    pub uid: String,
    pub contact: Option<String>,
    pub dtstart: crate::Date,
    pub dtend: Option<crate::Date>,
    pub organizer: Option<String>,
    pub url: Option<String>,
    pub attendee: Vec<String>,
    pub comment: Vec<String>,
    #[component(append)]
    pub freebusy: Vec<crate::Period>,
    pub rstatus: Vec<crate::RequestStatus>,
    #[component(ignore)]
    pub x_prop: std::collections::BTreeMap<String, String>,
    #[component(ignore)]
    pub iana_prop: std::collections::BTreeMap<String, String>,
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
