/**
 * See [3.8.8.3. Request Status](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.8.3)
 */
#[derive(Clone, Debug, Default, PartialEq)]
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

impl crate::ser::Serialize for RequestStatus {
    fn ical(&self) -> crate::Result<String> {
        let mut s = format!(
            "{:.1};{}",
            self.statcode,
            crate::ser::escape(&self.statdesc)
        );

        if let Some(extdata) = &self.extdata {
            s.push(';');
            s.push_str(&crate::ser::escape(extdata));
        }

        Ok(s)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn ser() -> crate::Result {
        let status = crate::RequestStatus {
            statcode: 2.0,
            statdesc: "Success".to_string(),
            extdata: None,
        };
        assert_eq!(crate::ser::ical(&status)?, "2.0;Success");

        let status = crate::RequestStatus {
            statcode: 2.8,
            statdesc: "Success, repeating event ignored. Scheduled as a single event.".to_string(),
            extdata: Some("RRULE:FREQ=WEEKLY;INTERVAL=2".to_string()),
        };
        assert_eq!(
            crate::ser::ical(&status)?,
            "2.8;Success\\, repeating event ignored. Scheduled as a single event.;RRULE:FREQ=WEEKLY\\;INTERVAL=2"
        );

        Ok(())
    }
}
