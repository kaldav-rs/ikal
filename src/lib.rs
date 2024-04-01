#![warn(warnings)]

mod components;
mod content_line;
mod errors;
mod parser;
mod properties;

pub use components::*;
pub use errors::*;
pub use properties::*;

use content_line::*;
use ikal_derive::Component;

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
    fn test_folding() {
        let line = "DESCRIPTION:This is a lo
 ng description
  that exists on a long line.
PRODID:-//Nextcloud calendar v1.5.0";

        let expected = (
            "DESCRIPTION",
            crate::ContentLine::from("This is a long description that exists on a long line."),
        );

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

                if let Ok(expected) = std::fs::read_to_string(&file.with_extension("out")) {
                    let fail = file.with_extension("fail");
                    std::fs::remove_file(&fail).ok();

                    let actual = format!("{component:#?}\n");

                    if actual != expected {
                        std::fs::write(&fail, &actual).unwrap();
                    }

                    similar_asserts::assert_eq!(actual, expected, "{file:?}");
                } else {
                    assert!(component.is_err());
                }
            }
        }
    }
}
