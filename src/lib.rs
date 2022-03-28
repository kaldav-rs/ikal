use chrono::offset::TimeZone;
use chrono::offset::Local;

mod parser;

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
    type Error = String;

    fn try_from(properties: std::collections::BTreeMap<String, String>) -> Result<Self, Self::Error> {
        let mut vcalendar = VCalendar::new();

        for (key, value) in properties {
            match key.as_str() {
                "PRODID" => vcalendar.prodid = value,
                "VERSION" => vcalendar.version = value,
                "CALSCALE" => vcalendar.calscale = Some(value),
                _ => return Err(format!("Unknow key {}", key)),
            };
        }

        Ok(vcalendar)
    }
}

impl TryFrom<String> for VCalendar {
    type Error = String;

    fn try_from(raw: String) -> Result<Self, Self::Error> {
        match parser::parse_vcalendar(raw.as_str()) {
            Ok((_, Ok(o))) => Ok(o),
            Ok((_, Err(err))) => Err(err),
            Err(err) => Err(format!("{:?}", err)),
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
    created: DateTime,
    dtstamp: DateTime,
    last_modified: DateTime,
    uid: String,
    summary: String,
    class: Class,
    status: Status,
    dt_start: DateTime,
    dt_end: DateTime,
    extra: std::collections::BTreeMap<String, String>,
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

impl TryFrom<std::collections::BTreeMap<String, String>> for VEvent {
    type Error = String;

    fn try_from(properties: std::collections::BTreeMap<String, String>) -> Result<Self, Self::Error> {
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
                    "COMPLETED" => vevent.status = Status::Completed,
                    _ => return Err(format!("Unknow status {}", value)),
                },
                "DTSTART" => vevent.dt_start = VEvent::parse_date(value)?,
                "DTEND" => vevent.dt_end = VEvent::parse_date(value)?,
                _ => {
                    vevent.extra.insert(key.to_owned(), value);
                },
            };
        }

        Ok(vevent)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct VTodo {
    created: DateTime,
    dtstamp: DateTime,
    last_modified: DateTime,
    uid: String,
    summary: String,
    status: Status,
    percent_complete: u8,
    extra: std::collections::BTreeMap<String, String>,
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

impl TryFrom<std::collections::BTreeMap<String, String>> for VTodo {
    type Error = String;

    fn try_from(properties: std::collections::BTreeMap<String, String>) -> Result<Self, Self::Error> {
        let mut vtodo = VTodo::new();

        for (key, value) in properties {
            match key.as_str() {
                "CREATED" => vtodo.created = VTodo::parse_date(value)?,
                "DTSTAMP" => vtodo.dtstamp = VTodo::parse_date(value)?,
                "LAST-MODIFIED" => vtodo.last_modified = VTodo::parse_date(value)?,
                "UID" => vtodo.uid = value,
                "SUMMARY" => vtodo.summary = value,
                "PERCENT-COMPLETE" => vtodo.percent_complete = match value.parse() {
                    Ok(percent_complete) => percent_complete,
                    Err(err) => return Err(format!("{}", err)),
                },
                "STATUS" => match value.as_str() {
                    "CONFIRMED" => vtodo.status = Status::Confirmed,
                    "COMPLETED" => vtodo.status = Status::Completed,
                    _ => return Err(format!("Unknow status {}", value)),
                },
                _ => {
                    vtodo.extra.insert(key.to_owned(), value);
                },
            };
        }

        Ok(vtodo)
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Class {
    Public,
}

#[derive(Clone, Debug, PartialEq)]
enum Status {
    Confirmed,
    Completed,
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
            String::from("This is a long description that exists on a long line.")
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

        assert_eq!(
            crate::parser::properties(line),
            Ok(("\n", expected))
        );
    }

    #[test]
    fn test_properties() {
        let line = "VERSION:2.0
CALSCALE:GREGORIAN

";

        let mut expected = std::collections::BTreeMap::new();
        expected.insert("VERSION".into(), "2.0".into());
        expected.insert("CALSCALE".into(), "GREGORIAN".into());

        assert_eq!(
            crate::parser::properties(line),
            Ok(("\n", expected))
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
            crate::parser::parse_vevent(line),
            Ok(("", Ok(crate::VEvent {
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
            })))
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

    fn file_get_contents(path: &std::path::PathBuf) -> Result<String, String> {
        use std::io::Read;

        let mut content = String::new();

        let mut file = match std::fs::File::open(path) {
            Ok(file) => file,
            Err(err) => return Err(format!("Unable to open {:?}: {}", path, err)),
        };

        match file.read_to_string(&mut content) {
            Ok(_) => (),
            Err(err) => return Err(format!("Unable to read {:?}: {}", path, err)),
        };

        Ok(content)
    }
}
