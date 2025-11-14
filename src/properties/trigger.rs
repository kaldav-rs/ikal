/**
 * See [3.8.6.3. Trigger](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.6.3)
 */

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Trigger {
    DateTime(crate::DateTime),
    Duration(chrono::Duration),
}

impl Default for Trigger {
    fn default() -> Self {
        Self::Duration(chrono::Duration::zero())
    }
}

impl Trigger {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl std::str::FromStr for Trigger {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(dt) = crate::DateTime::from_str(s) {
            Ok(Self::DateTime(dt))
        } else {
            crate::parse_duration(s).map(Self::Duration)
        }
    }
}

impl crate::ser::Serialize for Trigger {
    fn ical(&self) -> String {
        match self {
            Self::DateTime(dt) => dt.ical(),
            Self::Duration(duration) => duration.ical(),
        }
    }

    fn attr(&self) -> Option<String> {
        let attr = match self {
            Self::DateTime(_) => "VALUE=DATE-TIME",
            Self::Duration(_) => "VALUE=DURATION",
        };

        attr.to_string().into()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn ser() -> crate::Result {
        let trigger = crate::Trigger::DateTime("19980101T050000Z".parse()?);
        assert_eq!(
            crate::ser::ical(&trigger),
            "VALUE=DATE-TIME:19980101T050000Z"
        );

        let trigger = crate::Trigger::Duration(chrono::Duration::days(-15));
        assert_eq!(crate::ser::ical(&trigger), "VALUE=DURATION:-PT1296000S");

        Ok(())
    }
}
