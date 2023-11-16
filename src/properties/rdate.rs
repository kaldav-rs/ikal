#[derive(Clone, Debug, PartialEq)]
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
    pub fn new() -> Self {
        Self::Date(Vec::new())
    }
}
