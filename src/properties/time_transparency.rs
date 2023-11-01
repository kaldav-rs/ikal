/**
 * This property defines whether or not an event is transparent to busy time searches.
 *
 * See [3.8.2.7. Time Transparency](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.7)
 */
#[derive(Clone, Debug, PartialEq)]
pub enum TimeTransparency {
    /** Blocks or opaque on busy time searches */
    Opaque,
    /** Transparent on busy time searches */
    Transparent,
}

impl std::str::FromStr for TimeTransparency {
    type Err = crate::Error;

    fn from_str(value: &str) -> crate::Result<Self> {
        let status = match value {
            "OPAQUE" => Self::Opaque,
            "TRANSPARENT" => Self::Transparent,

            _ => return Err(crate::Error::TimeTransparency(value.to_string())),
        };

        Ok(status)
    }
}
