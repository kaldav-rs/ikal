mod components;
mod errors;
mod parser;
mod properties;

pub use components::*;
pub use errors::*;
pub use properties::*;

type DateTime = chrono::DateTime<chrono::Local>;

#[cfg(test)]
mod test {
    #[test]
    fn test_content_line() {
        let line = "VERSION:2.0
PRODID:-//Nextcloud calendar v1.5.0";

        let expected = ("VERSION", "2.0".to_string());
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
            "This is a long description that exists on a long line.".to_string(),
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
        // @TODO https://tools.ietf.org/html/rfc2445#section-4.2
        let line = "CREATED;VALUE=DATE-TIME:20141009T141617Z

";

        let mut expected = std::collections::BTreeMap::new();
        expected.insert("CREATED".to_string(), "20141009T141617Z".to_string());

        assert_eq!(crate::parser::content_lines(line), Ok(("\n", expected)));
    }

    #[test]
    fn test_content_lines() {
        let line = "VERSION:2.0
CALSCALE:GREGORIAN

";

        let mut expected = std::collections::BTreeMap::new();
        expected.insert("VERSION".to_string(), "2.0".to_string());
        expected.insert("CALSCALE".to_string(), "GREGORIAN".to_string());

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
                let input = match file_get_contents(&file) {
                    Ok(input) => input,
                    Err(_) => continue,
                };

                let expected = match file_get_contents(&file.with_extension("out")) {
                    Ok(expected) => expected,
                    Err(_) => continue,
                };

                let fail = file.with_extension("fail");
                std::fs::remove_file(&fail).ok();

                let component: crate::Result<T> = input.try_into();
                let actual = format!("{component:#?}\n");

                if actual != expected {
                    std::fs::write(&fail, &actual).unwrap();
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
