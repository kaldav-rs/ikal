/**
 * See [3.8.6.3. Trigger](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.6.3)
 */

#[derive(Clone, Debug, PartialEq)]
pub enum Trigger {
    DateTime(crate::DateTime),
    Duration(chrono::Duration),
}

impl Default for Trigger {
    fn default() -> Self {
        Self::Duration(chrono::Duration::zero())
    }
}

impl Trigger {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
