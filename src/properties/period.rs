/**
 * See [3.3.9. Period of Time](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.9)
 */
#[derive(Clone, Debug, PartialEq)]
pub enum Period {
    StartEnd(StartEnd),
    StartDur(StartDur),
}

impl TryFrom<String> for Period {
    type Error = crate::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl std::str::FromStr for Period {
    type Err = crate::Error;

    fn from_str(s: &str) -> crate::Result<Self> {
        crate::parser::datatype::period(s)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StartEnd {
    pub start: crate::DateTime,
    pub end: crate::DateTime,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StartDur {
    pub start: crate::DateTime,
    pub duration: chrono::Duration,
}
