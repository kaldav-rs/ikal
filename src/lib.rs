use chrono::offset::Local;

mod parser;
mod errors;

pub use errors::*;

type DateTime = chrono::DateTime<Local>;

#[derive(Debug, PartialEq)]
pub struct VCalendar {
    prodid: String,
    version: String,
    calscale: Option<String>,
    pub content: Content,
}

impl VCalendar {
    fn new() -> Self {
        VCalendar {
            prodid: String::new(),
            version: String::new(),
            calscale: None,
            content: Content::Empty,
        }
    }
}

impl TryFrom<std::collections::BTreeMap<String, String>> for VCalendar {
    type Error = Error;

    fn try_from(
        properties: std::collections::BTreeMap<String, String>,
    ) -> Result<Self> {
        let mut vcalendar = VCalendar::new();

        for (key, value) in properties {
            match key.as_str() {
                "PRODID" => vcalendar.prodid = value,
                "VERSION" => vcalendar.version = value,
                "CALSCALE" => vcalendar.calscale = Some(value),
                _ => return Err(Error::Key(key.to_string())),
            };
        }

        Ok(vcalendar)
    }
}

impl TryFrom<String> for VCalendar {
    type Error = Error;

    fn try_from(raw: String) -> Result<Self> {
        match parser::parse_vcalendar(raw.as_str()) {
            Ok((_, Ok(o))) => Ok(o),
            Ok((_, Err(err))) => Err(err),
            Err(err) => Err(Error::Parser(format!("{err:?}"))),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Content {
    Empty,
    Event(crate::VEvent),
    Todo(crate::VTodo),
}

#[derive(Clone, Debug, PartialEq)]
pub struct VEvent {
    pub created: DateTime,
    pub dtstamp: DateTime,
    pub last_modified: DateTime,
    pub uid: String,
    pub summary: String,
    pub class: Class,
    pub status: Status,
    pub dt_start: DateTime,
    pub dt_end: DateTime,
    pub extra: std::collections::BTreeMap<String, String>,
}

impl Default for VEvent {
    fn default() -> Self {
        Self::new()
    }
}

impl VEvent {
    pub fn new() -> Self {
        Self {
            created: Local::now(),
            dtstamp: Local::now(),
            last_modified: Local::now(),
            uid: String::new(),
            summary: String::new(),
            class: Class::Public,
            status: Status::Confirmed,
            dt_start: Local::now(),
            dt_end: Local::now(),
            extra: std::collections::BTreeMap::new(),
        }
    }

    // <https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.5>
    fn parse_date<S>(date: S) -> Result<DateTime>
    where
        S: Into<String>,
    {
        let mut date = date.into();

        if date.len() == 8 {
            date.push_str("T000000");
        }

        let dt = chrono::NaiveDateTime::parse_from_str(date.as_str(), "%Y%m%dT%H%M%S")?;

        if date.ends_with('Z') {
            Ok(dt.and_utc().with_timezone(&chrono::Local))
        } else {
            Ok(dt.and_local_timezone(chrono::Local).unwrap())
        }
    }
}

impl TryFrom<std::collections::BTreeMap<String, String>> for VEvent {
    type Error = Error;

    fn try_from(
        properties: std::collections::BTreeMap<String, String>,
    ) -> Result<Self> {
        let mut vevent = VEvent::new();

        for (key, value) in properties {
            match key.as_str() {
                "CREATED" => vevent.created = VEvent::parse_date(value)?,
                "DTSTAMP" => vevent.dtstamp = VEvent::parse_date(value)?,
                "LAST-MODIFIED" => vevent.last_modified = VEvent::parse_date(value)?,
                "UID" => vevent.uid = value,
                "SUMMARY" => vevent.summary = value,
                "CLASS" => vevent.class = value.into(),
                "STATUS" => vevent.status = value.try_into()?,
                "DTSTART" => vevent.dt_start = VEvent::parse_date(value)?,
                "DTEND" => vevent.dt_end = VEvent::parse_date(value)?,
                _ => {
                    vevent.extra.insert(key.to_owned(), value);
                }
            };
        }

        Ok(vevent)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct VTodo {
    pub created: DateTime,
    pub dtstamp: DateTime,
    pub last_modified: DateTime,
    pub uid: String,
    pub summary: String,
    pub status: Status,
    pub percent_complete: u8,
    pub extra: std::collections::BTreeMap<String, String>,
}

impl Default for VTodo {
    fn default() -> Self {
        Self::new()
    }
}

impl VTodo {
    pub fn new() -> Self {
        VTodo {
            created: Local::now(),
            dtstamp: Local::now(),
            last_modified: Local::now(),
            uid: String::new(),
            summary: String::new(),
            status: Status::Confirmed,
            percent_complete: 0,
            extra: std::collections::BTreeMap::new(),
        }
    }

    fn parse_date<S>(date: S) -> Result<DateTime>
    where
        S: Into<String>,
    {
        VEvent::parse_date(date)
    }
}

impl TryFrom<std::collections::BTreeMap<String, String>> for VTodo {
    type Error = Error;

    fn try_from(
        properties: std::collections::BTreeMap<String, String>,
    ) -> Result<Self> {
        let mut vtodo = VTodo::new();

        for (key, value) in properties {
            match key.as_str() {
                "CREATED" => vtodo.created = VTodo::parse_date(value)?,
                "DTSTAMP" => vtodo.dtstamp = VTodo::parse_date(value)?,
                "LAST-MODIFIED" => vtodo.last_modified = VTodo::parse_date(value)?,
                "UID" => vtodo.uid = value,
                "SUMMARY" => vtodo.summary = value,
                "PERCENT-COMPLETE" => vtodo.percent_complete = value.parse()?,
                "STATUS" => vtodo.status = value.try_into()?,
                _ => {
                    vtodo.extra.insert(key.to_owned(), value);
                }
            };
        }

        Ok(vtodo)
    }
}

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

impl From<String> for Class {
    fn from(value: String) -> Self {
        match value.as_str() {
            "PUBLIC" => Self::Public,
            "PRIVATE" => Self::Private,
            "CONFIDENTIAL" => Self::Confidential,
            c => Self::Custom(c.to_string()),
        }
    }
}

/**
 * This property defines the overall status or confirmation for the calendar component.
 *
 * See [3.8.1.11. Status](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.11)
 */
#[derive(Clone, Debug, PartialEq)]
pub enum Status {
    /** Indicates event is tentative */
    Tentative,
    /** Indicates event is definite */
    Confirmed,
    /** Indicates event/to-do/journal was cancelled/removed */
    Cancelled,
    /** Indicates to-do needs action */
    NeedsAction,
    /** Indicates to-do completed */
    Completed,
    /** Indicates to-do in process of */
    InProcess,
    /** Indicates journal is draft */
    Draft,
    /** Indicates journal is final */
    Final,
}

impl TryFrom<String> for Status {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        let status = match value.as_str() {
            "TENTATIVE" => Self::Tentative,
            "CONFIRMED" => Self::Confirmed,
            "CANCELLED" => Self::Cancelled,
            "NEEDS-ACTION" => Self::NeedsAction,
            "COMPLETED" => Self::Completed,
            "IN-PROCESS" => Self::InProcess,
            "DRAFT" => Self::Draft,
            "FINAL" => Self::Final,

            _ => return Err(Error::Status(value.to_string())),
        };

        Ok(status)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_property() {
        let line = "VERSION:2.0
PRODID:-//Nextcloud calendar v1.5.0";

        let expected = ("VERSION".to_owned(), "2.0".to_owned());
        assert_eq!(
            crate::parser::property(line),
            Ok(("PRODID:-//Nextcloud calendar v1.5.0", expected))
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
            String::from("This is a long description that exists on a long line."),
        );

        assert_eq!(
            crate::parser::property(line),
            Ok(("PRODID:-//Nextcloud calendar v1.5.0", expected))
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

        let mut expected = std::collections::BTreeMap::new();
        expected.insert("CREATED".into(), "20141009T141617Z".into());

        assert_eq!(crate::parser::properties(line), Ok(("\n", expected)));
    }

    #[test]
    fn test_properties() {
        let line = "VERSION:2.0
CALSCALE:GREGORIAN

";

        let mut expected = std::collections::BTreeMap::new();
        expected.insert("VERSION".into(), "2.0".into());
        expected.insert("CALSCALE".into(), "GREGORIAN".into());

        assert_eq!(crate::parser::properties(line), Ok(("\n", expected)));
    }

    #[test]
    fn test_component() {}

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
            crate::parser::parse_vevent(line),
            Ok((
                "",
                Ok(crate::VEvent {
                    created: crate::VEvent::parse_date("20170209T192358").unwrap(),
                    dtstamp: crate::VEvent::parse_date("20170209T192358").unwrap(),
                    last_modified: crate::VEvent::parse_date("20170209T192358").unwrap(),
                    uid: "5UILHLI7RI6K2IDRAQX7O".into(),
                    summary: "Vers".into(),
                    class: crate::Class::Public,
                    status: crate::Status::Confirmed,
                    dt_start: crate::VEvent::parse_date("20170209").unwrap(),
                    dt_end: crate::VEvent::parse_date("20170210").unwrap(),
                    extra: std::collections::BTreeMap::new(),
                })
            ))
        );
    }

    #[test]
    fn test_parse_vcalendar() {
        let tests = std::path::Path::new("tests");

        for entry in tests.read_dir().expect("Unable to open tests dir") {
            let file = match entry {
                Ok(entry) => entry.path(),
                Err(err) => {
                    println!("{}", err);
                    continue;
                }
            };

            let extension = match file.extension() {
                Some(extension) => extension,
                None => continue,
            };

            if extension == std::ffi::OsStr::new("ics") {
                let input = match file_get_contents(&file) {
                    Ok(input) => input,
                    Err(_) => continue,
                };

                let output = match file_get_contents(&file.with_extension("out")) {
                    Ok(output) => output,
                    Err(_) => continue,
                };

                let vcalendar = crate::parser::parse_vcalendar(input.as_str());
                assert_eq!(output, format!("{:#?}\n", vcalendar));
            }
        }
    }

    fn file_get_contents(path: &std::path::PathBuf) -> Result<String, std::io::Error> {
        use std::io::Read;

        let mut content = String::new();

        let mut file = std::fs::File::open(path)?;

        file.read_to_string(&mut content)?;

        Ok(content)
    }
}
