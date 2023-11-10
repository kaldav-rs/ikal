/**
 * See [3.8.5. Recurrence Component Properties](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.5)
 */

/**
 * See [3.8.5.1. Exception Date-Times](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.5.1)
 */
pub(crate) fn exdate(input: &str) -> crate::Result<Vec<crate::Date>> {
    input
        .split(',')
        .map(|x| {
            super::datatype::date_or_dt(x)
                .map(|x| x.1)
                .map_err(crate::Error::from)
        })
        .collect()
}

/**
 * See [3.8.5.2. Recurrence Date-Times](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.5.2)
 */
pub(crate) fn rdate(input: &str) -> crate::Result<Vec<crate::Date>> {
    input
        .split(',')
        .map(|x| {
            super::datatype::date_or_dt(x)
                .map(|x| x.1)
                .map_err(crate::Error::from)
        })
        .collect()
}

/**
 * See [3.8.5.3. Recurrence Rule](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.5.3)
 */
pub(crate) fn rrule(input: &str) -> crate::Result<crate::Recur> {
    use nom::character::complete::char;
    use nom::combinator::{map_res, opt};
    use nom::multi::many1;
    use nom::sequence::{separated_pair, terminated};

    fn item(input: &str) -> nom::IResult<&str, (&str, &str)> {
        terminated(
            separated_pair(super::key, char('='), super::key),
            opt(char(';')),
        )(input)
    }

    fn by(input: &&&str) -> crate::Result<Vec<i8>> {
        input
            .split(',')
            .map(str::parse)
            .collect::<Result<_, _>>()
            .map_err(crate::Error::from)
    }

    fn bywdaylist(input: &&&str) -> crate::Result<Vec<crate::WeekdayNum>> {
        input
            .split(',')
            .map(str::parse)
            .collect::<Result<_, _>>()
            .map_err(crate::Error::from)
    }

    map_res(many1(item), |items| {
        let map = items
            .iter()
            .map(|(k, v)| ((*k).to_string(), v))
            .collect::<std::collections::BTreeMap<_, _>>();

        let recur = crate::Recur {
            freq: map["FREQ"].parse()?,
            until: map
                .get("UNTIL")
                .map(|x| {
                    super::datatype::date_time(x)
                        .map(|x| x.1)
                        .map_err(crate::Error::from)
                })
                .transpose()?,
            count: map.get("COUNT").map(|x| x.parse()).transpose()?,
            interval: map.get("INTERVAL").map(|x| x.parse()).transpose()?,
            by_second: map.get("BYSECOND").map(by).transpose()?.unwrap_or_default(),
            by_minute: map.get("BYMINUTE").map(by).transpose()?.unwrap_or_default(),
            by_hour: map.get("BYHOUR").map(by).transpose()?.unwrap_or_default(),
            by_day: map
                .get("BYHOUR")
                .map(bywdaylist)
                .transpose()?
                .unwrap_or_default(),
            by_monthday: map
                .get("BYMONTHDAY")
                .map(by)
                .transpose()?
                .unwrap_or_default(),
            by_yearday: map
                .get("BYYEARDAY")
                .map(by)
                .transpose()?
                .unwrap_or_default(),
            by_weekno: map.get("BYWEEKNO").map(by).transpose()?.unwrap_or_default(),
            by_month: map.get("BYMONTH").map(by).transpose()?.unwrap_or_default(),
            by_setpos: map.get("BYSETPOS").map(by).transpose()?.unwrap_or_default(),
            wkst: map.get("WKST").map(|x| x.parse()).transpose()?,
        };

        Ok::<_, crate::Error>(recur)
    })(input)
    .map(|(_, x)| x)
    .map_err(crate::Error::from)
}
