use std::collections::BTreeMap;

/**
 * See [3.3.11. Text](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.11)
 */
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Text {
    pub params: BTreeMap<String, String>,
    pub text: String,
}

impl Text {
    pub fn from(text: &str) -> Self {
        Self {
            params: BTreeMap::new(),
            text: text.to_string(),
        }
    }
}

impl From<crate::ContentLine> for Text {
    fn from(value: crate::ContentLine) -> Self {
        Self {
            params: value.params,
            text: value.value,
        }
    }
}

impl std::fmt::Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.text)
    }
}

impl std::ops::Deref for Text {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.text
    }
}

impl From<String> for Text {
    fn from(value: String) -> Self {
        Self {
            params: BTreeMap::new(),
            text: value,
        }
    }
}

impl From<&str> for Text {
    fn from(value: &str) -> Self {
        Self::from(value)
    }
}

impl crate::ser::Serialize for Text {
    fn ical(&self) -> crate::Result<String> {
        self.to_string().ical()
    }

    fn attr(&self) -> Option<String> {
        if self.params.is_empty() {
            None
        } else {
            self.params.ical().ok()
        }
    }
}

impl AsRef<str> for Text {
    fn as_ref(&self) -> &str {
        self.text.as_ref()
    }
}

mod test {
    #[test]
    fn ser() -> crate::Result {
        let text = crate::Text::from(
            "Project XYZ ; Final Review
Conference Room - 3B
Come Prepared.",
        );

        assert_eq!(
            crate::ser::ical(&text)?,
            "Project XYZ \\; Final Review\\nConference Room - 3B\\nCome Prepared."
        );

        let text = crate::Text {
            params: [
                ("VALUE".to_string(), "DATE-TIME".to_string()),
                ("TZID".to_string(), "Europe/Paris".to_string()),
            ]
            .into(),
            text: "20150219T190000".to_string(),
        };
        assert_eq!(
            crate::ser::ical(&text)?,
            "TZID=Europe/Paris;VALUE=DATE-TIME:20150219T190000"
        );

        Ok(())
    }
}
