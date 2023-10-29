use chrono::offset::Local;
use std::collections::BTreeMap;

mod components;
mod parser;
mod errors;

pub use components::*;
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

impl TryFrom<BTreeMap<String, String>> for VCalendar {
    type Error = Error;

    fn try_from(
        properties: BTreeMap<String, String>,
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
        parser::parse_vcalendar(raw.as_str())
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

impl TryFrom<String> for TimeTransparency {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        let status = match value.as_str() {
            "OPAQUE" => Self::Opaque,
            "TRANSPARENT" => Self::Transparent,

            _ => return Err(Error::TimeTransparency(value.to_string())),
        };

        Ok(status)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Geo {
    pub lat: f32,
    pub lon: f32,
}

/**
 * This value type is used to identify properties that contain a recurrence rule specification.
 *
 * See [3.3.10. Recurrence Rule](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.5.3)
 */
#[derive(Clone, Debug, PartialEq)]
pub struct Recur {
    freq: Freq,
    until: Option<DateTime>,
    count: Option<u8>,
    interval: Option<u8>,
    by_second: Vec<i8>,
    by_minute: Vec<i8>,
    by_hour: Vec<i8>,
    by_day: Vec<WeekdayNum>,
    by_monthday: Vec<i8>,
    by_yearday: Vec<i8>,
    by_weekno: Vec<i8>,
    by_month: Vec<i8>,
    by_setpos: Vec<i8>,
    wkst: Option<Weekday>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Freq {
    Secondly,
    Minutely,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

impl std::str::FromStr for Freq {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let freq = match s {
            "SECONDLY" => Self::Secondly,
            "MINUTELY" => Self::Minutely,
            "HOURLY" => Self::Hourly,
            "DAILY" => Self::Daily,
            "WEEKLY" => Self::Weekly,
            "MONTHLY" => Self::Monthly,
            "YEARLY" => Self::Yearly,

            _ => return Err(Error::Freq(s.to_string())),
        };

        Ok(freq)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WeekdayNum {
    weekday: Weekday,
    ord: i8,
}

impl std::str::FromStr for WeekdayNum {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        parser::parse_weekdaynum(s)
            .map_err(Error::from)
            .map(|(_, x)| x)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Weekday {
    Sunday,
    Monday,
    Tuesday,
    Wenesday,
    Thurday,
    Friday,
    Saturday,
}

impl std::str::FromStr for Weekday {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        parser::parse_weekday(s)
            .map_err(Error::from)
            .map(|(_, x)| x)
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
                    created: Some(crate::parser::parse_date("20170209T192358").unwrap()),
                    dtstamp: crate::parser::parse_date("20170209T192358").unwrap(),
                    last_modified: Some(crate::parser::parse_date("20170209T192358").unwrap()),
                    uid: "5UILHLI7RI6K2IDRAQX7O".into(),
                    summary: Some("Vers".into()),
                    class: Some(crate::Class::Public),
                    status: Some(crate::Status::Confirmed),
                    dt_start: crate::parser::parse_date("20170209").unwrap(),
                    dt_end: crate::parser::parse_date("20170210").unwrap(),

                    ..Default::default()
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
                    println!("{err}");
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

                let actual = match file_get_contents(&file.with_extension("out")) {
                    Ok(actual) => actual,
                    Err(_) => continue,
                };

                let vcalendar = crate::parser::parse_vcalendar(input.as_str());
                let expected = format!("{:#?}\n", vcalendar);

                if actual != expected {
                    let path = file.with_extension("fail");
                    std::fs::write(path, &expected).unwrap();
                }

                assert_eq!(actual, expected, "{file:?}");
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
