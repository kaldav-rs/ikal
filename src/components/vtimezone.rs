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
    pub standard: Vec<Standard>,
    #[component(ignore)]
    pub daylight: Vec<Daylight>,
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
    Standard(Standard),
    Daylight(Daylight),
}

macro_rules! prop {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq, crate::Component)]
        pub struct $name {
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

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl $name {
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
    }
}

prop!(Standard);
prop!(Daylight);

#[cfg(test)]
mod test {
    #[test]
    fn parse() {
        crate::test::test_files::<crate::VTimezone>("timezones");
    }

    #[test]
    fn ser() -> crate::Result {
        let vtimezone = crate::vtimezone! {
            tzid: "America/New_York",
            last_modified: "20050809T050000Z",
            tzurl: "http://zones.example.com/tz/America-New_York.ics",
            standard: [
                {
                    dtstart: "20071104T020000",
                    tzoffsetfrom: "-0400",
                    tzoffsetto: "-0500",
                    tzname: ["EST"],
                },
            ],
            daylight: [
                {
                    dtstart: "20070311T020000",
                    tzoffsetfrom: "-0500",
                    tzoffsetto: "-0400",
                    tzname: ["EDT"],
                },
            ],
        }?;

        let ical = crate::ser::ical(&vtimezone)?;

        similar_asserts::assert_eq!(
            ical,
            "BEGIN:VTIMEZONE\r
TZID:America/New_York\r
LAST-MODIFIED:20050809T050000Z\r
TZURL:http://zones.example.com/tz/America-New_York.ics\r
BEGIN:STANDARD\r
DTSTART:20071104T020000\r
TZOFFSETTO:-0500\r
TZOFFSETFROM:-0400\r
TZNAME:EST\r
END:STANDARD\r
BEGIN:DAYLIGHT\r
DTSTART:20070311T020000\r
TZOFFSETTO:-0400\r
TZOFFSETFROM:-0500\r
TZNAME:EDT\r
END:DAYLIGHT\r
END:VTIMEZONE\r
"
        );

        Ok(())
    }
}
