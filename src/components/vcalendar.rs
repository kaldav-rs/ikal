/**
 * See [3.6. Calendar Components](https://datatracker.ietf.org/doc/html/rfc5545#section-3.4)
 */
#[derive(Debug, Default, PartialEq, crate::Component)]
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
    fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn parse() {
        crate::test::test_files::<crate::VCalendar>("calendars")
    }
}
