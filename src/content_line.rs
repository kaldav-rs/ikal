use std::collections::BTreeMap;

/**
 * See [3.1. Content Lines](https://datatracker.ietf.org/doc/html/rfc5545#section-3.1)
 */

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContentLine {
    pub params: BTreeMap<String, String>,
    pub value: String,
}

impl ContentLine {
    #[cfg(test)]
    pub fn from(value: &str) -> Self {
        Self {
            value: value.to_string(),
            params: BTreeMap::new(),
        }
    }
}

impl From<String> for ContentLine {
    fn from(value: String) -> Self {
        Self {
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
