/**
 * See [3.8.8. Miscellaneous Component Properties](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.8)
 */

/**
 * See [3.8.8.3. Request Status](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.8.3)
 */
pub(crate) fn rstatus(input: crate::ContentLine) -> crate::Result<crate::RequestStatus> {
    use nom::bytes::complete::{take_till, take_while};
    use nom::character::complete::char;
    use nom::combinator::{map, opt};
    use nom::error::context;
    use nom::number::complete::float;
    use nom::sequence::{preceded, tuple};

    fn text(input: &str) -> super::NomResult<&str, &str> {
        context("text", take_till(|c| c == ';'))(input)
    }

    fn end(input: &str) -> super::NomResult<&str, &str> {
        context("end", take_while(|_| true))(input)
    }

    context(
        "rstatus",
        map(
            tuple((
                float,
                char(';'),
                map(text, String::from),
                opt(preceded(char(';'), map(end, String::from))),
            )),
            |(statcode, _, statdesc, extdata)| crate::RequestStatus {
                statcode,
                statdesc,
                extdata,
            },
        ),
    )(input.value.as_str())
    .map_err(crate::Error::from)
    .map(|(_, x)| x)
}

#[cfg(test)]
mod test {
    #[test]
    fn rstatus() {
        assert_eq!(
            crate::parser::rstatus("2.0;Success".into()).unwrap(),
            crate::RequestStatus {
                statcode: 2.0,
                statdesc: "Success".to_string(),
                extdata: None,
            }
        );

        assert_eq!(
            crate::parser::rstatus("3.1;Invalid property value;DTSTART:96-Apr-01".into()).unwrap(),
            crate::RequestStatus {
                statcode: 3.1,
                statdesc: "Invalid property value".to_string(),
                extdata: Some("DTSTART:96-Apr-01".to_string()),
            }
        );

        assert_eq!(
            crate::parser::rstatus("2.8; Success\\, repeating event ignored. Scheduled\r\n as a single event.;RRULE:FREQ=WEEKLY\\;INTERVAL=2".into()).unwrap(),
            crate::RequestStatus {
                statcode: 2.8,
                statdesc: " Success\\, repeating event ignored. Scheduled\r\n as a single event.".to_string(),
                extdata: Some("RRULE:FREQ=WEEKLY\\;INTERVAL=2".to_string()),
            }
        );

        assert_eq!(
            crate::parser::rstatus("4.1;Event conflict.  Date-time is busy.".into()).unwrap(),
            crate::RequestStatus {
                statcode: 4.1,
                statdesc: "Event conflict.  Date-time is busy.".to_string(),
                extdata: None,
            }
        );

        assert_eq!(
            crate::parser::rstatus(
                "3.7;Invalid calendar user;ATTENDEE:\r\n mailto:jsmith@example.com".into()
            )
            .unwrap(),
            crate::RequestStatus {
                statcode: 3.7,
                statdesc: "Invalid calendar user".to_string(),
                extdata: Some("ATTENDEE:\r\n mailto:jsmith@example.com".to_string()),
            }
        );
    }
}
