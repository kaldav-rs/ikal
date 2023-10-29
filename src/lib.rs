use chrono::offset::Local;
use std::collections::BTreeMap;

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

#[derive(Debug, PartialEq)]
pub enum Content {
    Empty,
    Event(crate::VEvent),
    Todo(crate::VTodo),
}

/**
 * See [3.6.1. Event Component](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6.1)
 */
#[derive(Clone, Debug, PartialEq)]
pub struct VEvent {
    pub dtstamp: DateTime,
    pub uid: String,
    pub dt_start: DateTime,
    pub dt_end: DateTime,
    pub class: Option<Class>,
    pub created: Option<DateTime>,
    pub description: Option<String>,
    pub geo: Option<Geo>,
    pub last_modified: Option<DateTime>,
    pub location: Option<String>,
    pub organizer: Option<String>,
    pub priority: Option<u8>,
    pub seq: Option<u32>,
    pub status: Option<Status>,
    pub summary: Option<String>,
    pub transp: Option<TimeTransparency>,
    pub url: Option<String>,
    pub recurid: Option<String>,
    pub rrule: Option<Recur>,
    pub attach: Vec<String>,
    pub attendee: Vec<String>,
    pub categories: Vec<String>,
    pub comment: Vec<String>,
    pub contact: Vec<String>,
    pub exdate: Vec<DateTime>,
    pub rstatus: Vec<String>,
    pub related: Vec<String>,
    pub resources: Vec<String>,
    pub rdate: Vec<String>,
    pub x_prop: BTreeMap<String, String>,
    pub iana_prop: BTreeMap<String, String>,
}

impl Default for VEvent {
    fn default() -> Self {
        Self::new()
    }
}

impl VEvent {
    pub fn new() -> Self {
        Self {
            dtstamp: Local::now(),
            uid: String::new(),
            dt_start: Local::now(),
            dt_end: Local::now(),
            class: None,
            created: None,
            description: None,
            geo: None,
            last_modified: None,
            location: None,
            organizer: None,
            priority: None,
            seq: None,
            status: None,
            summary: None,
            transp: None,
            url: None,
            recurid: None,
            rrule: None,
            attach: Vec::new(),
            attendee: Vec::new(),
            categories: Vec::new(),
            comment: Vec::new(),
            contact: Vec::new(),
            exdate: Vec::new(),
            rstatus: Vec::new(),
            related: Vec::new(),
            resources: Vec::new(),
            rdate: Vec::new(),
            x_prop: BTreeMap::new(),
            iana_prop: BTreeMap::new(),
        }
    }
}

impl TryFrom<BTreeMap<String, String>> for VEvent {
    type Error = Error;

    fn try_from(
        properties: BTreeMap<String, String>,
    ) -> Result<Self> {
        let mut vevent = VEvent::new();

        for (key, value) in properties {
            match key.as_str() {
                "DTSTAMP" => vevent.dtstamp = parser::parse_date(value)?,
                "UID" => vevent.uid = value,
                "DTSTART" => vevent.dt_start = parser::parse_date(value)?,
                "DTEND" => vevent.dt_end = parser::parse_date(value)?,
                "DURATION" => vevent.dt_end = vevent.dt_start + parser::parse_duration(value)?,
                "CLASS" => vevent.class = Some(value.into()),
                "CREATED" => vevent.created = Some(parser::parse_date(value)?),
                "DESCRIPTION" => vevent.description = Some(value),
                "GEO" => vevent.geo = Some(parser::parse_geo(value)?),
                "LAST-MODIFIED" => vevent.last_modified = Some(parser::parse_date(value)?),
                "LOCATION" => vevent.location = Some(value),
                "ORGANIZER" => vevent.organizer = Some(parser::parse_organizer(value)?),
                "PRIORITY" => vevent.priority = Some(parser::parse_priority(value)?),
                "SEQ" => vevent.seq = Some(parser::parse_sequence(value)?),
                "STATUS" => vevent.status = Some(value.try_into()?),
                "SUMMARY" => vevent.summary = Some(value),
                "STRANSP" => vevent.transp = Some(value.try_into()?),
                "URL" => vevent.url = Some(value),
                "RECURID" => vevent.recurid = Some(parser::parse_recurid(value)?),
                "RRULE" => vevent.rrule = Some(parser::parse_rrule(value)?),
                "ATTACH" => vevent.attach.push(parser::parse_attach(value)),
                "ATTENDEE" => vevent.attendee.push(parser::parse_attendee(value)),
                "CATEGORIES" => vevent.categories.append(&mut parser::parse_categories(value)),
                "COMMENT" => vevent.comment.push(parser::parse_comment(value)),
                "CONTACT" => vevent.contact.push(parser::parse_contact(value)),
                "EXDATE" => vevent.exdate.append(&mut parser::parse_exdate(value)?),
                "RSTATUS" => vevent.rstatus.push(parser::parse_rstatus(value)?),
                "RELATED-TO" => vevent.related.push(parser::parse_related(value)),
                "RESOURCES" => vevent.resources.append(&mut parser::parse_resources(value)),
                "RDATE" => vevent.rdate.append(&mut parser::parse_rdate(value)?),
                _ => if key.starts_with("X-") {
                    vevent.x_prop.insert(key, value);
                } else {
                    vevent.iana_prop.insert(key, value);
                }
            };
        }

        Ok(vevent)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct VTodo {
    pub dtstamp: DateTime,
    pub uid: String,
    pub class: Option<Class>,
    pub completed: Option<DateTime>,
    pub created: Option<DateTime>,
    pub dt_start: Option<DateTime>,
    pub geo: Option<Geo>,
    pub last_modified: Option<DateTime>,
    pub location: Option<String>,
    pub organizer: Option<String>,
    pub percent_complete: Option<u8>,
    pub priority: Option<u8>,
    pub recurid: Option<String>,
    pub seq: Option<u32>,
    pub status: Option<Status>,
    pub summary: Option<String>,
    pub url: Option<String>,
    pub rrule: Option<Recur>,
    pub due: Option<DateTime>,
    pub duration: Option<chrono::Duration>,
    pub attach: Vec<String>,
    pub attendee: Vec<String>,
    pub categories: Vec<String>,
    pub comment: Vec<String>,
    pub contact: Vec<String>,
    pub exdate: Vec<DateTime>,
    pub rstatus: Vec<String>,
    pub related: Vec<String>,
    pub resources: Vec<String>,
    pub rdate: Vec<String>,
    pub x_prop: BTreeMap<String, String>,
    pub iana_prop: BTreeMap<String, String>,
}

impl Default for VTodo {
    fn default() -> Self {
        Self::new()
    }
}

impl VTodo {
    pub fn new() -> Self {
        Self {
            dtstamp: Local::now(),
            uid: String::new(),
            class: None,
            completed: None,
            created: None,
            dt_start: None,
            geo: None,
            last_modified: None,
            location: None,
            organizer: None,
            percent_complete: None,
            priority: None,
            recurid: None,
            seq: None,
            status: None,
            summary: None,
            url: None,
            rrule: None,
            due: None,
            duration: None,
            attach: Vec::new(),
            attendee: Vec::new(),
            categories: Vec::new(),
            comment: Vec::new(),
            contact: Vec::new(),
            exdate: Vec::new(),
            rstatus: Vec::new(),
            related: Vec::new(),
            resources: Vec::new(),
            rdate: Vec::new(),
            x_prop: BTreeMap::new(),
            iana_prop: BTreeMap::new(),
        }
    }
}

impl TryFrom<BTreeMap<String, String>> for VTodo {
    type Error = Error;

    fn try_from(
        properties: BTreeMap<String, String>,
    ) -> Result<Self> {
        let mut vtodo = Self::new();

        for (key, value) in properties {
            match key.as_str() {
                "DTSTAMP" => vtodo.dtstamp = parser::parse_date(value)?,
                "UID" => vtodo.uid = value,
                "CLASS" => vtodo.class = Some(value.into()),
                "COMPLETED" => vtodo.completed = Some(parser::parse_date(value)?),
                "CREATED" => vtodo.created = Some(parser::parse_date(value)?),
                "DTSTART" => vtodo.dt_start = Some(parser::parse_date(value)?),
                "GEO" => vtodo.geo = Some(parser::parse_geo(value)?),
                "LAST-MODIFIED" => vtodo.last_modified = Some(parser::parse_date(value)?),
                "LOCATION" => vtodo.location = Some(value),
                "ORGANIZER" => vtodo.organizer = Some(parser::parse_organizer(value)?),
                "PERCENT-COMPLETE" => vtodo.percent_complete = Some(value.parse()?),
                "PRIORITY" => vtodo.priority = Some(parser::parse_priority(value)?),
                "RECURID" => vtodo.recurid = Some(parser::parse_recurid(value)?),
                "SEQ" => vtodo.seq = Some(parser::parse_sequence(value)?),
                "STATUS" => vtodo.status = Some(value.try_into()?),
                "SUMMARY" => vtodo.summary = Some(value),
                "URL" => vtodo.url = Some(value),
                "RRULE" => vtodo.rrule = Some(parser::parse_rrule(value)?),
                "DUE" => vtodo.due = Some(parser::parse_date(value)?),
                "DURATION" => vtodo.duration = Some(parser::parse_duration(value)?),
                "ATTACH" => vtodo.attach.push(parser::parse_attach(value)),
                "ATTENDEE" => vtodo.attendee.push(parser::parse_attendee(value)),
                "CATEGORIES" => vtodo.categories.append(&mut parser::parse_categories(value)),
                "COMMENT" => vtodo.comment.push(parser::parse_comment(value)),
                "CONTACT" => vtodo.contact.push(parser::parse_contact(value)),
                "EXDATE" => vtodo.exdate.append(&mut parser::parse_exdate(value)?),
                "RSTATUS" => vtodo.rstatus.push(parser::parse_rstatus(value)?),
                "RELATED-TO" => vtodo.related.push(parser::parse_related(value)),
                "RESOURCES" => vtodo.resources.append(&mut parser::parse_resources(value)),
                "RDATE" => vtodo.rdate.append(&mut parser::parse_rdate(value)?),
                _ => if key.starts_with("X-") {
                    vtodo.x_prop.insert(key, value);
                } else {
                    vtodo.iana_prop.insert(key, value);
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
