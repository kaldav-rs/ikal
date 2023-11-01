use std::collections::BTreeMap;

/**
 * See [3.6. Calendar Components](https://datatracker.ietf.org/doc/html/rfc5545#section-3.4)
 */
#[derive(Debug, Default, PartialEq)]
pub struct VCalendar {
    pub prodid: String,
    pub version: String,
    pub calscale: Option<String>,
    pub method: Option<String>,
    pub events: Vec<crate::VEvent>,
    pub journals: Vec<crate::VJournal>,
    pub todo: Vec<crate::VTodo>,
    pub timezones: Vec<crate::VTimezone>,
    pub x_prop: BTreeMap<String, String>,
    pub iana_prop: BTreeMap<String, String>,
}

impl VCalendar {
    fn new() -> Self {
        Self::default()
    }
}

impl TryFrom<std::collections::BTreeMap<String, String>> for VCalendar {
    type Error = crate::Error;

    fn try_from(properties: BTreeMap<String, String>) -> Result<Self, Self::Error> {
        let mut vcalendar = VCalendar::new();

        for (key, value) in properties {
            match key.as_str() {
                "PRODID" => vcalendar.prodid = value,
                "VERSION" => vcalendar.version = value,
                "CALSCALE" => vcalendar.calscale = Some(value),
                "METHOD" => vcalendar.method = Some(value),
                _ => {
                    if key.starts_with("X-") {
                        vcalendar.x_prop.insert(key, value);
                    } else {
                        vcalendar.iana_prop.insert(key, value);
                    }
                }
            };
        }

        Ok(vcalendar)
    }
}

impl TryFrom<String> for VCalendar {
    type Error = crate::Error;

    fn try_from(raw: String) -> Result<Self, Self::Error> {
        crate::parser::vcalendar(&raw)
            .map_err(crate::Error::from)
            .map(|(_, x)| x)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn parse() {
        crate::test::test_files::<crate::VCalendar>("calendars")
    }
}
