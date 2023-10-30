use std::collections::BTreeMap;

mod components;
mod errors;
mod parser;
mod properties;

pub use components::*;
pub use errors::*;
pub use properties::*;

type DateTime = chrono::DateTime<chrono::Local>;

/**
 * See [3.6. Calendar Components](https://datatracker.ietf.org/doc/html/rfc5545#section-3.4)
 */
#[derive(Debug, PartialEq)]
pub struct VCalendar {
    pub prodid: String,
    pub version: String,
    pub calscale: Option<String>,
    pub method: Option<String>,
    pub content: Content,
}

impl Default for VCalendar {
    fn default() -> Self {
        Self::new()
    }
}

impl VCalendar {
    fn new() -> Self {
        VCalendar {
            prodid: String::new(),
            version: String::new(),
            calscale: None,
            method: None,
            content: Content::default(),
        }
    }
}

impl TryFrom<BTreeMap<String, String>> for VCalendar {
    type Error = Error;

    fn try_from(properties: BTreeMap<String, String>) -> Result<Self> {
        let mut vcalendar = VCalendar::new();

        for (key, value) in properties {
            match key.as_str() {
                "PRODID" => vcalendar.prodid = value,
                "VERSION" => vcalendar.version = value,
                "CALSCALE" => vcalendar.calscale = Some(value),
                "METHOD" => vcalendar.method = Some(value),
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
            .map_err(crate::Error::from)
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
                crate::VEvent {
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
                }
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
