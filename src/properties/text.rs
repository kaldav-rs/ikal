use std::collections::BTreeMap;

/**
 * See [3.3.11. Text](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.11)
 */
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Text {
    pub params: BTreeMap<String, String>,
    pub text: String,
}

impl From<crate::ContentLine> for Text {
    fn from(value: crate::ContentLine) -> Self {
        Self {
            params: value.params,
            text: value.value,
        }
    }
}

impl ToString for Text {
    fn to_string(&self) -> String {
        self.text.clone()
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
        Self {
            params: BTreeMap::new(),
            text: value.to_string(),
        }
    }
}
