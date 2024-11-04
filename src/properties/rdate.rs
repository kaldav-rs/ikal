#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RDate {
    Date(Vec<crate::Date>),
    Period(Vec<crate::Period>),
}

impl Default for RDate {
    fn default() -> Self {
        Self::new()
    }
}

impl RDate {
    #[must_use]
    pub fn new() -> Self {
        Self::Date(Vec::new())
    }
}

impl std::fmt::Display for RDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            RDate::Date(date) => format!(
                "VALUE=DATE:{}",
                date.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            RDate::Period(period) => format!(
                "VALUE=PERIOD:{}",
                period
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            ),
        };

        f.write_str(&s)
    }
}

crate::ser::ical_for_tostring!(RDate);

#[cfg(test)]
mod test {
    #[test]
    fn ser() -> crate::Result {
        let rdate = crate::RDate::Date(vec![
            "19970101".parse()?,
            "19970120".parse()?,
            "19970217".parse()?,
            "19970421".parse()?,
            "19970526".parse()?,
            "19970704".parse()?,
            "19970901".parse()?,
            "19971014".parse()?,
            "19971128".parse()?,
            "19971129".parse()?,
            "19971225".parse()?,
        ]);

        assert_eq!(crate::ser::ical(&rdate)?, "VALUE=DATE:19970101,19970120,19970217,19970421,19970526,19970704,19970901,19971014,19971128,19971129,19971225");

        Ok(())
    }
}
