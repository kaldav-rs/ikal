#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DateTime {
    Naive(chrono::NaiveDateTime),
    Local(chrono::DateTime<chrono::Local>),
}

impl DateTime {
    #[must_use]
    pub fn date_naive(&self) -> chrono::NaiveDate {
        match self {
            Self::Naive(date) => date.date(),
            Self::Local(date) => date.date_naive(),
        }
    }

    #[must_use]
    pub fn format<'a>(
        &self,
        fmt: &'a str,
    ) -> chrono::format::DelayedFormat<chrono::format::StrftimeItems<'a>> {
        match self {
            Self::Naive(date) => date.format(fmt),
            Self::Local(date) => date.format(fmt),
        }
    }

    #[must_use]
    pub fn naive(&self) -> chrono::NaiveDateTime {
        match self {
            Self::Naive(date) => *date,
            Self::Local(date) => date.naive_local(),
        }
    }
}

impl Default for DateTime {
    fn default() -> Self {
        Self::Naive(chrono::NaiveDateTime::default())
    }
}

impl From<Date> for DateTime {
    fn from(value: Date) -> Self {
        match value {
            Date::Date(date) => DateTime::Naive(date.and_hms_opt(0, 0, 0).unwrap()),
            Date::DateTime(dt) => dt.clone(),
        }
    }
}
impl From<chrono::NaiveDateTime> for DateTime {
    fn from(value: chrono::NaiveDateTime) -> Self {
        Self::Naive(value)
    }
}

impl From<chrono::DateTime<chrono::Local>> for DateTime {
    fn from(value: chrono::DateTime<chrono::Local>) -> Self {
        Self::Local(value)
    }
}

impl From<DateTime> for chrono::NaiveDateTime {
    fn from(value: DateTime) -> Self {
        match value {
            DateTime::Naive(naive) => naive,
            DateTime::Local(local) => local.naive_local(),
        }
    }
}

impl TryFrom<DateTime> for chrono::DateTime<chrono::Local> {
    type Error = crate::Error;

    fn try_from(value: DateTime) -> Result<Self, Self::Error> {
        match value {
            DateTime::Naive(naive) => naive.and_local_timezone(chrono::Local).earliest().ok_or(crate::Error::Local(value)),
            DateTime::Local(local) => Ok(local),
        }
    }
}

impl std::ops::Sub for DateTime {
    type Output = chrono::Duration;

    fn sub(self, rhs: Self) -> Self::Output {
        self.naive().sub(rhs.naive())
    }
}

impl std::ops::Sub<chrono::Duration> for DateTime {
    type Output = chrono::NaiveDateTime;

    fn sub(self, rhs: chrono::Duration) -> Self::Output {
        self.naive().sub(rhs)
    }
}

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.naive().fmt(f)
    }
}

impl std::cmp::PartialOrd for DateTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for DateTime {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.naive().cmp(&other.naive())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

impl From<DateTime> for Date {
    fn from(value: DateTime) -> Self {
        Date::DateTime(value)
    }
}

impl From<chrono::NaiveDate> for Date {
    fn from(value: chrono::NaiveDate) -> Self {
        Self::Date(value)
    }
}

impl From<chrono::DateTime<chrono::Local>> for Date {
    fn from(value: chrono::DateTime<chrono::Local>) -> Self {
        Self::DateTime(value.into())
    }
}

impl From<chrono::NaiveDateTime> for Date {
    fn from(value: chrono::NaiveDateTime) -> Self {
        Self::DateTime(value.into())
    }
}

impl From<Date> for chrono::NaiveDate {
    fn from(value: Date) -> Self {
        match value {
            Date::Date(date) => date,
            Date::DateTime(dt) => dt.date_naive(),
        }
    }
}

impl std::cmp::PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Date {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Date(a), Self::Date(b)) => a.cmp(b),
            (Self::DateTime(a), Self::DateTime(b)) => a.cmp(b),
            (Self::Date(a), Self::DateTime(b)) => a.cmp(&b.date_naive()),
            (Self::DateTime(a), Self::Date(b)) => a.date_naive().cmp(b),
        }
    }
}
