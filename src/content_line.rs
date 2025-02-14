use std::collections::BTreeMap;

/**
 * See [3.1. Content Lines](https://datatracker.ietf.org/doc/html/rfc5545#section-3.1)
 */

#[derive(Clone, Eq, PartialEq)]
pub struct ContentLine {
    pub key: String,
    pub params: BTreeMap<String, String>,
    pub value: String,
}

impl std::fmt::Debug for ContentLine {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ContentLine")
            .field("params", &self.params)
            .field("value", &self.value)
            .finish()
    }
}

impl ContentLine {
    #[cfg(test)]
    pub fn from(value: &str) -> Self {
        Self {
            key: String::new(),
            value: value.to_string(),
            params: BTreeMap::new(),
        }
    }
}

impl From<String> for ContentLine {
    fn from(value: String) -> Self {
        Self {
            key: String::new(),
            params: BTreeMap::new(),
            value,
        }
    }
}

impl From<&str> for ContentLine {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl std::fmt::Display for ContentLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.value)
    }
}

impl crate::ser::Serialize for ContentLine {
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
