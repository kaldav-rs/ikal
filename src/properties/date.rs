#[derive(Clone, Debug, PartialEq)]
pub enum DateTime {
    Naive(chrono::NaiveDateTime),
    Local(chrono::DateTime<chrono::Local>),
}

impl DateTime {
    pub fn date_naive(&self) -> chrono::NaiveDate {
        match self {
            Self::Naive(date) => date.date(),
            Self::Local(date) => date.date_naive(),
        }
    }

    pub fn format<'a>(
        &self,
        fmt: &'a str,
    ) -> chrono::format::DelayedFormat<chrono::format::StrftimeItems<'a>> {
        match self {
            Self::Naive(date) => date.format(fmt),
            Self::Local(date) => date.format(fmt),
        }
    }
}

impl Default for DateTime {
    fn default() -> Self {
        Self::Naive(chrono::NaiveDateTime::default())
    }
}

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Naive(date) => date.fmt(f),
            Self::Local(date) => date.fmt(f),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Date {
    Date(chrono::NaiveDate),
    DateTime(DateTime),
}

impl Date {
    #[must_use]
    pub fn date_naive(&self) -> chrono::NaiveDate {
        match self {
            Self::Date(date) => *date,
            Self::DateTime(date_time) => date_time.date_naive(),
        }
    }

    #[must_use]
    pub fn format<'a>(
        &self,
        fmt: &'a str,
    ) -> chrono::format::DelayedFormat<chrono::format::StrftimeItems<'a>> {
        match self {
            Self::Date(date) => date.format(fmt),
            Self::DateTime(date_time) => date_time.format(fmt),
        }
    }

    #[must_use]
    pub fn has_time(&self) -> bool {
        matches!(self, Self::DateTime(_))
    }
}

impl Default for Date {
    fn default() -> Self {
        Self::DateTime(DateTime::default())
    }
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Date(date) => date.fmt(f),
            Self::DateTime(date_time) => date_time.fmt(f),
        }
    }
}
