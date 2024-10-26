use chrono::Datelike as _;

/**
 * See [3.3.10. Recurrence Rule](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.5.3)
 */
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Recur {
    pub freq: Freq,
    pub until: Option<crate::Date>,
    pub count: Option<u8>,
    pub interval: u8,
    pub by_second: Vec<i8>,
    pub by_minute: Vec<i8>,
    pub by_hour: Vec<i8>,
    pub by_day: Vec<WeekdayNum>,
    pub by_monthday: Vec<i8>,
    pub by_yearday: Vec<i8>,
    pub by_weekno: Vec<i8>,
    pub by_month: Vec<i8>,
    pub by_setpos: Vec<i8>,
    pub wkst: Option<Weekday>,
}

impl TryFrom<String> for Recur {
    type Error = crate::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<&str> for Recur {
    type Error = crate::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl std::str::FromStr for Recur {
    type Err = crate::Error;

    fn from_str(s: &str) -> crate::Result<Self> {
        crate::parser::rrule(s.into())
    }
}

impl std::ops::Add<crate::Date> for crate::Recur {
    type Output = crate::DateTime;

    fn add(self, rhs: crate::Date) -> Self::Output {
        match rhs {
            crate::Date::Date(date) => {
                crate::DateTime::Naive(self + date.and_hms_opt(0, 0, 0).unwrap())
            }
            crate::Date::DateTime(dt) => self + dt,
        }
    }
}
impl std::ops::Add<chrono::NaiveDate> for crate::Recur {
    type Output = chrono::NaiveDateTime;

    fn add(self, rhs: chrono::NaiveDate) -> Self::Output {
        let interval = match self.freq {
            Freq::Secondly => chrono::TimeDelta::seconds(self.interval.into()),
            Freq::Minutely => chrono::TimeDelta::minutes(self.interval.into()),
            Freq::Hourly => chrono::TimeDelta::hours(self.interval.into()),
            Freq::Daily => chrono::TimeDelta::days(self.interval.into()),
            Freq::Weekly => chrono::TimeDelta::weeks(self.interval.into()),
            Freq::Monthly => {
                return rhs.and_hms_opt(0, 0, 0).unwrap()
                    + chrono::Months::new(self.interval.into())
            }
            Freq::Yearly => {
                return rhs
                    .with_year(rhs.year() + self.interval as i32)
                    .unwrap()
                    .into()
            }
        };

        (rhs + interval).into()
    }
}

impl std::ops::Add<crate::DateTime> for crate::Recur {
    type Output = crate::DateTime;

    fn add(self, rhs: crate::DateTime) -> Self::Output {
        match rhs {
            crate::DateTime::Naive(date) => crate::DateTime::Naive(self + date),
            crate::DateTime::Local(date) => crate::DateTime::Local(self + date),
        }
    }
}

impl std::ops::Add<chrono::DateTime<chrono::Local>> for crate::Recur {
    type Output = chrono::DateTime<chrono::Local>;

    fn add(self, rhs: chrono::DateTime<chrono::Local>) -> Self::Output {
        let interval = match self.freq {
            Freq::Secondly => chrono::TimeDelta::seconds(self.interval.into()),
            Freq::Minutely => chrono::TimeDelta::minutes(self.interval.into()),
            Freq::Hourly => chrono::TimeDelta::hours(self.interval.into()),
            Freq::Daily => chrono::TimeDelta::days(self.interval.into()),
            Freq::Weekly => chrono::TimeDelta::weeks(self.interval.into()),
            Freq::Monthly => return rhs + chrono::Months::new(self.interval.into()),
            Freq::Yearly => {
                return rhs
                    .with_year(rhs.year() + self.interval as i32)
                    .unwrap()
                    .into()
            }
        };

        rhs + interval
    }
}

impl std::ops::Add<chrono::NaiveDateTime> for crate::Recur {
    type Output = chrono::NaiveDateTime;

    fn add(self, rhs: chrono::NaiveDateTime) -> Self::Output {
        let interval = match self.freq {
            Freq::Secondly => chrono::TimeDelta::seconds(self.interval.into()),
            Freq::Minutely => chrono::TimeDelta::minutes(self.interval.into()),
            Freq::Hourly => chrono::TimeDelta::hours(self.interval.into()),
            Freq::Daily => chrono::TimeDelta::days(self.interval.into()),
            Freq::Weekly => chrono::TimeDelta::weeks(self.interval.into()),
            Freq::Monthly => return rhs + chrono::Months::new(self.interval.into()),
            Freq::Yearly => {
                return rhs
                    .with_year(rhs.year() + self.interval as i32)
                    .unwrap()
                    .into()
            }
        };

        rhs + interval
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum Freq {
    Secondly,
    Minutely,
    Hourly,
    #[default]
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

impl TryFrom<String> for Freq {
    type Error = crate::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<&str> for Freq {
    type Error = crate::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl std::str::FromStr for Freq {
    type Err = crate::Error;

    fn from_str(s: &str) -> crate::Result<Self> {
        let freq = match s {
            "SECONDLY" => Self::Secondly,
            "MINUTELY" => Self::Minutely,
            "HOURLY" => Self::Hourly,
            "DAILY" => Self::Daily,
            "WEEKLY" => Self::Weekly,
            "MONTHLY" => Self::Monthly,
            "YEARLY" => Self::Yearly,

            _ => return Err(crate::Error::Freq(s.to_string())),
        };

        Ok(freq)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WeekdayNum {
    pub weekday: Weekday,
    pub ord: Option<i8>,
}

impl TryFrom<String> for WeekdayNum {
    type Error = crate::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<&str> for WeekdayNum {
    type Error = crate::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl std::str::FromStr for WeekdayNum {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        crate::parser::weekdaynum(s)
            .map_err(crate::Error::from)
            .map(|(_, x)| x)
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Weekday {
    Sunday,
    Monday,
    Tuesday,
    Wenesday,
    Thurday,
    Friday,
    Saturday,
}

impl TryFrom<String> for Weekday {
    type Error = crate::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<&str> for Weekday {
    type Error = crate::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl std::str::FromStr for Weekday {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        crate::parser::weekday(s)
            .map_err(crate::Error::from)
            .map(|(_, x)| x)
    }
}
