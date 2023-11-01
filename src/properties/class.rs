/**
 * This property defines the access classification for a calendar component.
 *
 * See [3.8.1.3. Classification](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.3)
 */
#[derive(Clone, Debug, PartialEq)]
pub enum Class {
    Public,
    Private,
    Confidential,
    Custom(String),
}

impl std::str::FromStr for Class {
    type Err = crate::Error;

    fn from_str(value: &str) -> crate::Result<Self> {
        let class = match value {
            "PUBLIC" => Self::Public,
            "PRIVATE" => Self::Private,
            "CONFIDENTIAL" => Self::Confidential,
            c => Self::Custom(c.to_string()),
        };

        Ok(class)
    }
}
