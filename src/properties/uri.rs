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
