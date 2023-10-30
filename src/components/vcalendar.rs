/**
 * See [3.6. Calendar Components](https://datatracker.ietf.org/doc/html/rfc5545#section-3.4)
 */
#[derive(Debug, PartialEq)]
pub struct VCalendar {
    pub prodid: String,
    pub version: String,
    pub calscale: Option<String>,
    pub method: Option<String>,
    pub component: crate::Component,
}

impl Default for VCalendar {
    fn default() -> Self {
        Self::new()
    }
}

impl VCalendar {
    fn new() -> Self {
        VCalendar {
            prodid: String::new(),
            version: String::new(),
            calscale: None,
            method: None,
            component: crate::Component::default(),
        }
    }
}

impl TryFrom<std::collections::BTreeMap<String, String>> for VCalendar {
    type Error = crate::Error;

    fn try_from(properties: std::collections::BTreeMap<String, String>) -> Result<Self, Self::Error> {
        let mut vcalendar = VCalendar::new();

        for (key, value) in properties {
            match key.as_str() {
                "PRODID" => vcalendar.prodid = value,
                "VERSION" => vcalendar.version = value,
                "CALSCALE" => vcalendar.calscale = Some(value),
                "METHOD" => vcalendar.method = Some(value),
                _ => return Err(crate::Error::Key(key.to_string())),
            };
        }

        Ok(vcalendar)
    }
}

impl TryFrom<String> for VCalendar {
    type Error = crate::Error;

    fn try_from(raw: String) -> Result<Self, Self::Error> {
        crate::parser::parse_vcalendar(raw.as_str())
            .map_err(crate::Error::from)
            .map(|(_, x)| x)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_parse_vcalendar() {
        crate::test::test_files::<crate::VCalendar>("calendars")
    }
}
