use chrono::Datelike as _;

/**
 * See [3.3.10. Recurrence Rule](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.5.3)
 */
#[derive(Clone, Debug, Default, Eq, PartialEq, crate::Serialize)]
pub struct Recur {
    pub freq: Freq,
    pub until: Option<crate::Date>,
    pub count: Option<u8>,
    pub interval: u8,
    #[serialize(rename = "BYSECOND")]
    pub by_second: Vec<i8>,
    #[serialize(rename = "BYMINUTE")]
    pub by_minute: Vec<i8>,
    #[serialize(rename = "BYHOUR")]
    pub by_hour: Vec<i8>,
    #[serialize(rename = "BYDAY")]
    pub by_day: Vec<WeekdayNum>,
    #[serialize(rename = "BYMONTHDAY")]
    pub by_monthday: Vec<i8>,
    #[serialize(rename = "BYYEARDAY")]
    pub by_yearday: Vec<i8>,
    #[serialize(rename = "BYWEEKNO")]
    pub by_weekno: Vec<i8>,
    #[serialize(rename = "BYMONTH")]
    pub by_month: Vec<i8>,
    #[serialize(rename = "BYSETPOS")]
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
    type Output = crate::Date;

    fn add(self, rhs: crate::Date) -> Self::Output {
        match rhs {
            crate::Date::Date(date) => {
                if self.freq >= Freq::Daily {
                    crate::Date::Date(self + date)
                } else {
                    crate::DateTime::Naive(self + date.and_hms_opt(0, 0, 0).unwrap()).into()
                }
            }
            crate::Date::DateTime(dt) => (self + dt).into(),
        }
    }
}
impl std::ops::Add<chrono::NaiveDate> for crate::Recur {
    type Output = chrono::NaiveDate;

    fn add(self, rhs: chrono::NaiveDate) -> Self::Output {
        let interval = match self.freq {
            Freq::Secondly => return rhs,
            Freq::Minutely => return rhs,
            Freq::Hourly => return rhs,
            Freq::Daily => chrono::TimeDelta::days(self.interval.into()),
            Freq::Weekly => chrono::TimeDelta::weeks(self.interval.into()),
            Freq::Monthly => return rhs + chrono::Months::new(self.interval.into()),
            Freq::Yearly => return rhs.with_year(rhs.year() + self.interval as i32).unwrap(),
        };

        rhs + interval
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
            Freq::Yearly => return rhs.with_year(rhs.year() + self.interval as i32).unwrap(),
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
            Freq::Yearly => return rhs.with_year(rhs.year() + self.interval as i32).unwrap(),
        };

        rhs + interval
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
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

impl std::fmt::Display for Freq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Secondly => "SECONDLY",
            Self::Minutely => "MINUTELY",
            Self::Hourly => "HOURLY",
            Self::Daily => "DAILY",
            Self::Weekly => "WEEKLY",
            Self::Monthly => "MONTHLY",
            Self::Yearly => "YEARLY",
        };

        f.write_str(s)
    }
}

crate::ser::ical_for_tostring!(Freq);

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

impl std::fmt::Display for WeekdayNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ord) = self.ord {
            write!(f, "{ord}")?;
        }

        self.weekday.fmt(f)
    }
}

crate::ser::ical_for_tostring!(WeekdayNum);

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

impl std::fmt::Display for Weekday {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Weekday::Sunday => "SU",
            Weekday::Monday => "MO",
            Weekday::Tuesday => "TU",
            Weekday::Wenesday => "WE",
            Weekday::Thurday => "TH",
            Weekday::Friday => "FR",
            Weekday::Saturday => "SA",
        };

        f.write_str(s)
    }
}

crate::ser::ical_for_tostring!(Weekday);

#[cfg(test)]
mod test {
    #[test]
    fn ser_recur() -> crate::Result {
        let mut recur = crate::Recur::default();
        assert_eq!(crate::ser::ical(&recur)?, "FREQ=DAILY;INTERVAL=0");

        recur.by_hour = vec![1];
        assert_eq!(crate::ser::ical(&recur)?, "FREQ=DAILY;INTERVAL=0;BYHOUR=1");

        Ok(())
    }

    #[test]
    fn ser_freq() -> crate::Result {
        assert_eq!(crate::ser::ical(&crate::Freq::Yearly)?, "YEARLY");

        Ok(())
    }

    #[test]
    fn ser_weekday_num() -> crate::Result {
        let weekday_num = crate::WeekdayNum {
            weekday: crate::Weekday::Sunday,
            ord: Some(-1),
        };

        assert_eq!(crate::ser::ical(&weekday_num)?, "-1SU");

        Ok(())
    }

    #[test]
    fn ser_weekday() -> crate::Result {
        assert_eq!(crate::ser::ical(&crate::Weekday::Monday)?, "MO");

        Ok(())
    }
}
