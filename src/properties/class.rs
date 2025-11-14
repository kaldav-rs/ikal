/**
 * See [3.8.1.3. Classification](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.3)
 */
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum Class {
    #[default]
    Public,
    Private,
    Confidential,
    Custom(String),
}

impl TryFrom<String> for Class {
    type Error = crate::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<&str> for Class {
    type Error = crate::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl std::str::FromStr for Class {
    type Err = crate::Error;

    fn from_str(s: &str) -> crate::Result<Self> {
        let class = match s {
            "PUBLIC" => Self::Public,
            "PRIVATE" => Self::Private,
            "CONFIDENTIAL" => Self::Confidential,
            c => Self::Custom(c.to_string()),
        };

        Ok(class)
    }
}

impl std::fmt::Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Public => "PUBLIC".to_string(),
            Self::Private => "PRIVATE".to_string(),
            Self::Confidential => "CONFIDENTIAL".to_string(),
            Self::Custom(s) => s.to_uppercase(),
        };

        f.write_str(&s)
    }
}

crate::ser::ical_for_tostring!(Class);

#[cfg(test)]
mod test {
    #[test]
    fn ser() {
        let class = crate::Class::Public;
        assert_eq!(crate::ser::ical(&class), "PUBLIC");

        let class = crate::Class::Custom("Custom".to_string());
        assert_eq!(crate::ser::ical(&class), "CUSTOM");
    }
}
