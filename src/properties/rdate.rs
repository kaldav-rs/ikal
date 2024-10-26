#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RDate {
    Date(Vec<crate::Date>),
    Period(Vec<crate::Period>),
}

impl Default for RDate {
    fn default() -> Self {
        Self::new()
    }
}

impl RDate {
    #[must_use]
    pub fn new() -> Self {
        Self::Date(Vec::new())
    }
}
