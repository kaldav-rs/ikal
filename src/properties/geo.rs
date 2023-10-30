#[derive(Clone, Debug, PartialEq)]
pub struct Geo {
    pub lat: f32,
    pub lon: f32,
}

impl TryFrom<String> for Geo {
    type Error = crate::Error;

    fn try_from(raw: String) -> Result<Self, Self::Error> {
        crate::parser::parse_geo(raw.as_str())
            .map_err(crate::Error::from)
            .map(|(_, x)| x)
    }
}
