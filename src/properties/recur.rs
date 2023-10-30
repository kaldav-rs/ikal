/**
 * This value type is used to identify properties that contain a recurrence rule specification.
 *
 * See [3.3.10. Recurrence Rule](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.5.3)
 */
#[derive(Clone, Debug, PartialEq)]
pub struct Recur {
    pub freq: Freq,
    pub until: Option<crate::DateTime>,
    pub count: Option<u8>,
    pub interval: Option<u8>,
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

    fn try_from(raw: String) -> Result<Self, Self::Error> {
        crate::parser::parse_rrule(raw.as_str())
            .map_err(crate::Error::from)
            .map(|(_, x)| x)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Freq {
    Secondly,
    Minutely,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Yearly,
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

#[derive(Clone, Debug, PartialEq)]
pub struct WeekdayNum {
    pub weekday: Weekday,
    pub ord: i8,
}

impl std::str::FromStr for WeekdayNum {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        crate::parser::parse_weekdaynum(s)
            .map_err(crate::Error::from)
            .map(|(_, x)| x)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Weekday {
    Sunday,
    Monday,
    Tuesday,
    Wenesday,
    Thurday,
    Friday,
    Saturday,
}

impl std::str::FromStr for Weekday {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        crate::parser::parse_weekday(s)
            .map_err(crate::Error::from)
            .map(|(_, x)| x)
    }
}
