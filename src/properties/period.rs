/**
 * See [3.3.9. Period of Time](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.9)
 */
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Period {
    StartEnd(StartEnd),
    StartDur(StartDur),
}

impl Period {
    #[must_use]
    pub fn duration(&self) -> chrono::Duration {
        match self {
            Self::StartEnd(StartEnd { start, end }) => *end - *start,
            Self::StartDur(StartDur { duration, .. }) => *duration,
        }
    }
}

impl TryFrom<String> for Period {
    type Error = crate::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<&str> for Period {
    type Error = crate::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl std::str::FromStr for Period {
    type Err = crate::Error;

    fn from_str(s: &str) -> crate::Result<Self> {
        crate::parser::datatype::period(s)
    }
}

impl std::cmp::PartialOrd for Period {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Period {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.duration().cmp(&other.duration())
    }
}

impl std::fmt::Display for Period {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Period::StartEnd(start_end) => start_end.to_string(),
            Period::StartDur(start_dur) => start_dur.to_string(),
        };

        f.write_str(&s)
    }
}

crate::ser::ical_for_tostring!(Period);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StartEnd {
    pub start: crate::DateTime,
    pub end: crate::DateTime,
}

impl std::cmp::PartialOrd for StartEnd {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for StartEnd {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = self.end - self.start;
        let b = other.end - other.start;

        a.cmp(&b)
    }
}

impl std::fmt::Display for StartEnd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.start, self.end)
    }
}

crate::ser::ical_for_tostring!(StartEnd);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StartDur {
    pub start: crate::DateTime,
    pub duration: chrono::Duration,
}

impl std::cmp::PartialOrd for StartDur {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for StartDur {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = self.start - self.duration;
        let b = other.start - other.duration;

        a.cmp(&b)
    }
}

impl std::fmt::Display for StartDur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.start, self.duration)
    }
}

crate::ser::ical_for_tostring!(StartDur);

#[cfg(test)]
mod test {
    #[test]
    fn ser() {
        let period = crate::Period::StartEnd(crate::period::StartEnd {
            start: crate::DateTime::default(),
            end: crate::DateTime::default(),
        });
        assert_eq!(crate::ser::ical(&period), "19700101T000000/19700101T000000");

        let period = crate::Period::StartDur(crate::period::StartDur {
            start: crate::DateTime::default(),
            duration: chrono::TimeDelta::hours(5),
        });
        assert_eq!(crate::ser::ical(&period), "19700101T000000/PT18000S");
    }
}
