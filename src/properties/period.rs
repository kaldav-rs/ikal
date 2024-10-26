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
            Self::StartEnd(StartEnd { start, end }) => end.clone() - start.clone(),
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
        let a = self.end.clone() - self.start.clone();
        let b = other.end.clone() - other.start.clone();

        a.cmp(&b)
    }
}

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
        let a = self.start.clone() - self.duration;
        let b = other.start.clone() - other.duration;

        a.cmp(&b)
    }
}
