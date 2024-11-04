use std::collections::BTreeMap;

/**
 * See [3.3.13. URI](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.13)
 */
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Uri {
    pub params: BTreeMap<String, String>,
    pub uri: String,
}

impl From<crate::ContentLine> for Uri {
    fn from(value: crate::ContentLine) -> Self {
        Self {
            params: value.params,
            uri: value.value,
        }
    }
}

impl std::fmt::Display for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.uri)
    }
}

impl std::ops::Deref for Uri {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.uri
    }
}

impl From<String> for Uri {
    fn from(value: String) -> Self {
        Self {
            params: BTreeMap::new(),
            uri: value,
        }
    }
}

impl From<&str> for Uri {
    fn from(value: &str) -> Self {
        Self {
            params: BTreeMap::new(),
            uri: value.to_string(),
        }
    }
}

impl crate::ser::Serialize for Uri {
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

#[cfg(test)]
mod test {
    #[test]
    fn ser() -> crate::Result {
        let uri = crate::Uri::from("http://tzurl.org/zoneinfo/Pacific/Fiji");
        assert_eq!(
            crate::ser::ical(&uri)?,
            "http://tzurl.org/zoneinfo/Pacific/Fiji"
        );

        let uri = crate::Uri {
            params: [("RSVP".to_string(), "TRUE".to_string())].into(),
            uri: "mailto:someone@example.com".to_string(),
        };
        assert_eq!(
            crate::ser::ical(&uri)?,
            "RSVP=TRUE:mailto:someone@example.com"
        );

        Ok(())
    }
}
