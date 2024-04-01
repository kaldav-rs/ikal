use std::collections::BTreeMap;

/**
 * See [3.6.5. Time Zone Component](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6.5)
 */
#[derive(Clone, Debug, Default, Eq, PartialEq, crate::Component)]
pub struct VTimezone {
    pub tzid: crate::Text,
    pub last_modified: Option<crate::DateTime>,
    pub tzurl: Option<crate::Uri>,
    #[component(ignore)]
    pub standard: Vec<Prop>,
    #[component(ignore)]
    pub daylight: Vec<Prop>,
    #[component(ignore)]
    pub x_prop: std::collections::BTreeMap<String, crate::ContentLine>,
    #[component(ignore)]
    pub iana_prop: std::collections::BTreeMap<String, crate::ContentLine>,
}

impl VTimezone {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum Component {
    Standard(Prop),
    Daylight(Prop),
}

#[derive(Clone, Debug, Eq, PartialEq, crate::Component)]
pub struct Prop {
    pub dtstart: crate::Date,
    pub tzoffsetto: chrono::offset::FixedOffset,
    pub tzoffsetfrom: chrono::offset::FixedOffset,
    pub rrule: Option<crate::Recur>,
    pub comment: Vec<crate::Text>,
    pub rdate: Vec<crate::RDate>,
    pub tzname: Vec<crate::Text>,
    #[component(ignore)]
    pub x_prop: BTreeMap<String, crate::ContentLine>,
    #[component(ignore)]
    pub iana_prop: BTreeMap<String, crate::ContentLine>,
}

impl Default for Prop {
    fn default() -> Self {
        Self::new()
    }
}

impl Prop {
    #[must_use]
    fn new() -> Self {
        Self {
            dtstart: crate::Date::default(),
            tzoffsetto: chrono::offset::FixedOffset::east_opt(0).unwrap(),
            tzoffsetfrom: chrono::offset::FixedOffset::east_opt(0).unwrap(),
            rrule: None,
            comment: Vec::new(),
            rdate: Vec::new(),
            tzname: Vec::new(),
            x_prop: BTreeMap::new(),
            iana_prop: BTreeMap::new(),
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn parse() {
        crate::test::test_files::<crate::VTimezone>("timezones");
    }
}
