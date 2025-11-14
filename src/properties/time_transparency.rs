/**
 * See [3.8.2.7. Time Transparency](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.7)
 */
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TimeTransparency {
    /** Blocks or opaque on busy time searches */
    Opaque,
    /** Transparent on busy time searches */
    Transparent,
}

impl TryFrom<String> for TimeTransparency {
    type Error = crate::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<&str> for TimeTransparency {
    type Error = crate::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl std::str::FromStr for TimeTransparency {
    type Err = crate::Error;

    fn from_str(s: &str) -> crate::Result<Self> {
        let status = match s {
            "OPAQUE" => Self::Opaque,
            "TRANSPARENT" => Self::Transparent,

            _ => return Err(crate::Error::TimeTransparency(s.to_string())),
        };

        Ok(status)
    }
}

impl std::fmt::Display for TimeTransparency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Opaque => "OPAQUE",
            Self::Transparent => "TRANSPARENT",
        };

        f.write_str(s)
    }
}

crate::ser::ical_for_tostring!(TimeTransparency);

#[cfg(test)]
mod test {
    #[test]
    fn ser() {
        assert_eq!(
            crate::ser::ical(&crate::TimeTransparency::Transparent),
            "TRANSPARENT"
        );
        assert_eq!(crate::ser::ical(&crate::TimeTransparency::Opaque), "OPAQUE");
    }
}
