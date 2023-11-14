/**
 * See [3.8.8.3. Request Status](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.8.3)
 */
#[derive(Clone, Debug, PartialEq)]
pub struct RequestStatus {
    pub statcode: f32,
    pub statdesc: String,
    pub extdata: Option<String>,
}

impl TryFrom<String> for RequestStatus {
    type Error = crate::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<&str> for RequestStatus {
    type Error = crate::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl std::str::FromStr for RequestStatus {
    type Err = crate::Error;

    fn from_str(s: &str) -> crate::Result<Self> {
        crate::parser::rstatus(s.into())
    }
}
