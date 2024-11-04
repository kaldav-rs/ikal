/**
 * See [3.8.1.6. Geographic Position](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.6)
 */
#[derive(Clone, Debug, PartialEq)]
pub struct Geo {
    pub lat: f32,
    pub lon: f32,
}

impl TryFrom<String> for Geo {
    type Error = crate::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<&str> for Geo {
    type Error = crate::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl std::str::FromStr for Geo {
    type Err = crate::Error;

    fn from_str(s: &str) -> crate::Result<Self> {
        crate::parser::geo(s.into())
    }
}

impl std::fmt::Display for Geo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{};{}", self.lat, self.lon)
    }
}

crate::ser::ical_for_tostring!(Geo);

#[cfg(test)]
mod test {
    #[test]
    fn ser() -> crate::Result {
        let geo = crate::Geo {
            lat: 37.386013,
            lon: -122.08293,
        };

        assert_eq!(crate::ser::ical(&geo)?, "37.386013;-122.08293");

        Ok(())
    }
}
