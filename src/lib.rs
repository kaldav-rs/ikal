/* https://tools.ietf.org/html/rfc2445 */
#![feature(try_from)]

extern crate chrono;
#[macro_use]
extern crate nom;

use chrono::offset::TimeZone;
use chrono::offset::local::Local;

mod parser;

type DateTime = ::chrono::datetime::DateTime<Local>;

#[derive(Debug, PartialEq)]
pub struct VCalendar {
    prodid: String,
    version: String,
    calscale: String,
    pub event: VEvent,
}

impl VCalendar {
    fn new() -> Self {
        VCalendar {
            prodid: String::new(),
            version: String::new(),
            calscale: String::new(),
            event: VEvent::new(),
        }
    }
}

impl ::std::convert::TryFrom<::std::collections::HashMap<String, String>> for VCalendar {
    type Error = String;

    fn try_from(properties: ::std::collections::HashMap<String, String>) -> Result<Self, Self::Error> {
        let mut vcalendar = VCalendar::new();

        for (key, value) in properties {
            match key.as_str() {
                "PRODID" => vcalendar.prodid = value,
                "VERSION" => vcalendar.version = value,
                "CALSCALE" => vcalendar.calscale = value,
                _ => return Err(format!("Unknow key {}", key)),
            };
        }

        Ok(vcalendar)
    }
}

// @TODO use TryFrom
impl ::std::convert::TryFrom<String> for VCalendar {
    type Error = String;

    fn try_from(raw: String) -> Result<Self, Self::Error> {
        match ::parser::parse_vcalendar(raw.as_str()) {
            ::nom::IResult::Done(_, o) => o,
            ::nom::IResult::Error(err) => Err(format!("{}", err)),
            ::nom::IResult::Incomplete(_) => Err("Incomplete".into()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct VEvent {
    created: DateTime,
    dtstamp: DateTime,
    last_modified: DateTime,
    uid: String,
    summary: String,
    class: Class,
    status: Status,
    dt_start: DateTime,
    dt_end: DateTime,
    location: String,
    description: String,
    categories: String,
}

impl VEvent {
    pub fn new() -> Self {
        VEvent {
            created: Local::now(),
            dtstamp: Local::now(),
            last_modified: Local::now(),
            uid: String::new(),
            summary: String::new(),
            class: Class::Public,
            status: Status::Confirmed,
            dt_start: Local::now(),
            dt_end: Local::now(),
            location: String::new(),
            description: String::new(),
            categories: String::new(),
        }
    }

    fn parse_date<S>(date: S) -> Result<DateTime, String> where S: Into<String> {
        let mut date = date.into();

        if date.len() == 8 {
            date.push_str("T000000Z");
        }
        if date.len() == 15 {
            date.push_str("Z");
        }

        match Local.datetime_from_str(date.as_str(), "%Y%m%dT%H%M%SZ") {
            Ok(date) => Ok(date),
            Err(_) => Err(format!("Invalid date: {}", date)),
        }
    }
}

impl ::std::convert::TryFrom<::std::collections::HashMap<String, String>> for VEvent {
    type Error = String;

    fn try_from(properties: ::std::collections::HashMap<String, String>) -> Result<Self, Self::Error> {
        let mut vevent = VEvent::new();

        for (key, value) in properties {
            match key.as_str() {
                "CREATED" => vevent.created = VEvent::parse_date(value)?,
                "DTSTAMP" => vevent.dtstamp = VEvent::parse_date(value)?,
                "LAST-MODIFIED" => vevent.last_modified = VEvent::parse_date(value)?,
                "UID" => vevent.uid = value,
                "SUMMARY" => vevent.summary = value,
                "CLASS" => match value.as_str() {
                    "PUBLIC" => vevent.class = Class::Public,
                    _ => return Err(format!("Unknow class {}", value)),
                },
                "STATUS" => match value.as_str() {
                    "CONFIRMED" => vevent.status = Status::Confirmed,
                    _ => return Err(format!("Unknow status {}", value)),
                },
                "DTSTART" => vevent.dt_start = VEvent::parse_date(value)?,
                "DTEND" => vevent.dt_end = VEvent::parse_date(value)?,
                "LOCATION" => vevent.location = value,
                "DESCRIPTION" => vevent.location = value,
                "CATEGORIES" => vevent.location = value,
                _ => return Err(format!("Unknow key {}", key)),
            };
        }

        Ok(vevent)
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Class {
    Public,
}

#[derive(Clone, Debug, PartialEq)]
enum Status {
    Confirmed,
}

#[cfg(test)]
mod test {
    #[test]
    fn test_property() {
        let line = "VERSION:2.0
PRODID:-//Nextcloud calendar v1.5.0";

        let expected = ("VERSION".to_owned(), "2.0".to_owned());
        assert_eq!(
            ::parser::property(line),
            ::nom::IResult::Done("PRODID:-//Nextcloud calendar v1.5.0", expected)
        );
    }

    #[test]
    fn test_folding() {
        let line = "DESCRIPTION:This is a lo
 ng description
  that exists on a long line.
PRODID:-//Nextcloud calendar v1.5.0";

        let expected = (
            String::from("DESCRIPTION"),
            String::from("This is a long description that exists on a long line.")
        );

        assert_eq!(
            ::parser::property(line),
            ::nom::IResult::Done("PRODID:-//Nextcloud calendar v1.5.0", expected)
        );
    }

    #[test]
    fn test_binary_content() {
        // @TODO https://tools.ietf.org/html/rfc2445#section-4.1.3
    }

    #[test]
    fn test_param() {
        // @TODO https://tools.ietf.org/html/rfc2445#section-4.2
        let line = "CREATED;VALUE=DATE-TIME:20141009T141617Z

";

        let mut expected = ::std::collections::HashMap::new();
        expected.insert("CREATED".into(), "20141009T141617Z".into());

        assert_eq!(
            ::parser::properties(line),
            ::nom::IResult::Done("\n", expected)
        );
    }

    #[test]
    fn test_properties() {
        let line = "VERSION:2.0
CALSCALE:GREGORIAN

";

        let mut expected = ::std::collections::HashMap::new();
        expected.insert("VERSION".into(), "2.0".into());
        expected.insert("CALSCALE".into(), "GREGORIAN".into());

        assert_eq!(
            ::parser::properties(line),
            ::nom::IResult::Done("\n", expected)
        );
    }

    #[test]
    fn test_component() {
    }

    #[test]
    fn test_parse_vevent() {
        let line = "BEGIN:VEVENT
CREATED:20170209T192358
DTSTAMP:20170209T192358
LAST-MODIFIED:20170209T192358
UID:5UILHLI7RI6K2IDRAQX7O
SUMMARY:Vers
CLASS:PUBLIC
STATUS:CONFIRMED
DTSTART;VALUE=DATE:20170209
DTEND;VALUE=DATE:20170210
END:VEVENT
";

        assert_eq!(
            ::parser::parse_vevent(line),
            ::nom::IResult::Done("", Ok(::VEvent {
                created: ::VEvent::parse_date("20170209T192358").unwrap(),
                dtstamp: ::VEvent::parse_date("20170209T192358").unwrap(),
                last_modified: ::VEvent::parse_date("20170209T192358").unwrap(),
                uid: "5UILHLI7RI6K2IDRAQX7O".into(),
                summary: "Vers".into(),
                class: ::Class::Public,
                status: ::Status::Confirmed,
                dt_start: ::VEvent::parse_date("20170209").unwrap(),
                dt_end: ::VEvent::parse_date("20170210").unwrap(),
                location: "".into(),
                description: "".into(),
                categories: "".into(),
            }))
        );
    }

    #[test]
    fn test_parse_vcalendar() {
        let line = "BEGIN:VCALENDAR
PRODID:-//Nextcloud calendar v1.5.0
VERSION:2.0
CALSCALE:GREGORIAN
BEGIN:VEVENT
CREATED:20170209T192358
DTSTAMP:20170209T192358
LAST-MODIFIED:20170209T192358
UID:5UILHLI7RI6K2IDRAQX7O
SUMMARY:Vers
CLASS:PUBLIC
STATUS:CONFIRMED
DTSTART;VALUE=DATE:20170209
DTEND;VALUE=DATE:20170210
END:VEVENT
END:VCALENDAR
";

        let event = ::VEvent {
            created: ::VEvent::parse_date("20170209T192358").unwrap(),
            dtstamp: ::VEvent::parse_date("20170209T192358").unwrap(),
            last_modified: ::VEvent::parse_date("20170209T192358").unwrap(),
            uid: "5UILHLI7RI6K2IDRAQX7O".into(),
            summary: "Vers".into(),
            class: ::Class::Public,
            status: ::Status::Confirmed,
            dt_start: ::VEvent::parse_date("20170209").unwrap(),
            dt_end: ::VEvent::parse_date("20170210").unwrap(),
            location: "".into(),
            description: "".into(),
            categories: "".into(),
        };

        let calendar = ::VCalendar {
            prodid: "-//Nextcloud calendar v1.5.0".into(),
            version: "2.0".into(),
            calscale: "GREGORIAN".into(),
            event: event,
        };

        assert_eq!(
            ::parser::parse_vcalendar(line),
            ::nom::IResult::Done("", Ok(calendar))
        );
    }

    #[test]
    fn test_parse_vcalendar_empty() {
        let line = "BEGIN:VCALENDAR
VERSION:2.0
PRODID:ownCloud Calendar
CALSCALE:GREGORIAN
BEGIN:VEVENT
UID:7e27d9fc69
DTSTAMP:20150717T101959Z
CREATED:20150717T101959Z
LAST-MODIFIED:20150717T101959Z
SUMMARY:Garage
DTSTART;VALUE=DATE:20150722
DTEND;VALUE=DATE:20150723
LOCATION:
DESCRIPTION:
CATEGORIES:
END:VEVENT
END:VCALENDAR
";

        let event = ::VEvent {
            created: ::VEvent::parse_date("20150717T101959Z").unwrap(),
            dtstamp: ::VEvent::parse_date("20150717T101959Z").unwrap(),
            last_modified: ::VEvent::parse_date("20150717T101959Z").unwrap(),
            uid: "7e27d9fc69".into(),
            summary: "Garage".into(),
            class: ::Class::Public,
            status: ::Status::Confirmed,
            dt_start: ::VEvent::parse_date("20150722").unwrap(),
            dt_end: ::VEvent::parse_date("20150723").unwrap(),
            location: "".into(),
            description: "".into(),
            categories: "".into(),
        };

        let calendar = ::VCalendar {
            prodid: "ownCloud Calendar".into(),
            version: "2.0".into(),
            calscale: "GREGORIAN".into(),
            event: event,
        };

        assert_eq!(
            ::parser::parse_vcalendar(line),
            ::nom::IResult::Done("", Ok(calendar))
        );
    }
}
