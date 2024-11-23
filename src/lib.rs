#![warn(warnings)]

pub mod iter;
pub mod ser;

mod components;
mod content_line;
mod errors;
mod parser;
mod properties;

pub use components::*;
pub use errors::*;
pub use properties::*;

use content_line::*;
use ikal_derive::*;

#[macro_export]
macro_rules! valarm {
    (@$ty:ident, $( $tt:tt )*) => {
        $crate::$ty! { $( $tt )* }.map($crate::VAlarm::from)
    };
}

#[doc(hidden)]
pub fn parse_duration(value: &str) -> crate::Result<chrono::TimeDelta> {
    let mut duration = chrono::TimeDelta::default();
    let mut it = value.chars().peekable();

    let mut negative = false;
    let mut time = false;
    let mut interval = String::new();

    if it.next_if(|x| *x == '-').is_some() {
        negative = true;
    }

    if it.next_if(|x| *x == 'P').is_none() {
        return Err(crate::Error::ParseDuration(format!(
            "Invalid duration: {value}"
        )));
    };

    loop {
        if it.next_if(|x| *x == 'T').is_some() {
            time = true;
            continue;
        }

        if let Some(x) = it.next_if(|x| x.is_ascii_digit()) {
            interval.push(x);
            continue;
        };

        let Some(ty) = it.next() else {
            break;
        };

        let part = match (time, ty) {
            (false, 'Y') => chrono::TimeDelta::days(365 * interval.parse::<i64>()?),
            (false, 'M') => chrono::TimeDelta::days(30 * interval.parse::<i64>()?),
            (false, 'D') => chrono::TimeDelta::days(interval.parse()?),
            (true, 'H') => chrono::TimeDelta::hours(interval.parse()?),
            (true, 'M') => chrono::TimeDelta::minutes(interval.parse()?),
            (true, 'S') => chrono::TimeDelta::seconds(interval.parse()?),
            _ => {
                return Err(crate::Error::ParseDuration(format!(
                    "Invalid duration: {interval}"
                )))
            }
        };

        duration += part;
    }

    if negative {
        duration = -duration;
    }

    Ok(duration)
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    #[test]
    fn test_content_line() {
        let line = "VERSION:2.0
PRODID:-//Nextcloud calendar v1.5.0";

        let expected = ("VERSION", crate::ContentLine::from("2.0"));
        assert_eq!(
            crate::parser::content_line(line),
            Ok(("PRODID:-//Nextcloud calendar v1.5.0", expected))
        );
    }

    #[test]
    fn test_binary_content() {
        // @TODO https://tools.ietf.org/html/rfc2445#section-4.1.3
    }

    #[test]
    fn test_param() {
        let line = "CREATED;VALUE=DATE-TIME:20141009T141617Z\r\n";

        let mut params = BTreeMap::new();
        params.insert("VALUE".to_string(), "DATE-TIME".to_string());
        let mut expected = BTreeMap::new();
        expected.insert(
            "CREATED".to_string(),
            crate::ContentLine {
                value: "20141009T141617Z".to_string(),
                params,
            },
        );

        assert_eq!(crate::parser::content_lines(line), Ok(("", expected)));
    }

    #[test]
    fn test_content_lines() {
        let line = "VERSION:2.0
CALSCALE:GREGORIAN

";

        let mut expected = BTreeMap::new();
        expected.insert("VERSION".to_string(), crate::ContentLine::from("2.0"));
        expected.insert(
            "CALSCALE".to_string(),
            crate::ContentLine::from("GREGORIAN"),
        );

        assert_eq!(crate::parser::content_lines(line), Ok(("\n", expected)));
    }

    pub(crate) fn test_files<T: std::fmt::Debug + TryFrom<String, Error = crate::Error>>(
        path: &str,
    ) {
        let tests = std::path::Path::new("tests").join(path);

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
                let input = match std::fs::read_to_string(&file) {
                    Ok(input) => input,
                    Err(_) => continue,
                };

                let component: crate::Result<T> = input.try_into();

                if let Ok(expected) = std::fs::read_to_string(file.with_extension("out")) {
                    let fail = file.with_extension("fail");
                    std::fs::remove_file(&fail).ok();

                    let actual = format!("{component:#?}\n");

                    if actual != expected {
                        std::fs::write(&fail, &actual).unwrap();
                    }

                    similar_asserts::assert_eq!(actual, expected, "{file:?}");
                } else {
                    assert!(component.is_err(), "{file:?}");
                }
            }
        }
    }
}
