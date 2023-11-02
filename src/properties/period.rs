#[derive(Clone, Debug, PartialEq)]
pub enum Period {
    StartEnd(StartEnd),
    StartDur(StartDur),
}

impl TryFrom<String> for Period {
    type Error = crate::Error;

    fn try_from(raw: String) -> Result<Self, Self::Error> {
        crate::parser::datatype::period(&raw)
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
