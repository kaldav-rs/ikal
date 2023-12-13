/**
 * See [3.8.5. Recurrence Component Properties](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.5)
 */

/**
 * See [3.8.5.1. Exception Date-Times](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.5.1)
 */
pub(crate) fn exdate(input: crate::ContentLine) -> crate::Result<Vec<crate::Date>> {
    input
        .value
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
pub(crate) fn rdate(input: crate::ContentLine) -> crate::Result<crate::RDate> {
    let tokens = input.value.split(',');

    if input.params.get("VALUE") == Some(&"PERIOD".to_string()) {
        let periods = tokens
            .map(|x| super::datatype::period(x))
            .collect::<crate::Result<Vec<_>>>()?;

        Ok(crate::RDate::Period(periods))
    } else {
        let dates = tokens
            .map(|x| {
                super::datatype::date_or_dt(x)
                    .map(|x| x.1)
                    .map_err(crate::Error::from)
            })
            .collect::<crate::Result<Vec<_>>>()?;

        Ok(crate::RDate::Date(dates))
    }
}

/**
 * See [3.8.5.3. Recurrence Rule](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.5.3)
 */
pub(crate) fn rrule(input: crate::ContentLine) -> crate::Result<crate::Recur> {
    use nom::bytes::complete::take_till;
    use nom::character::complete::char;
    use nom::combinator::{map_res, opt};
    use nom::error::context;
    use nom::multi::many1;
    use nom::sequence::{separated_pair, terminated};

    fn item(input: &str) -> super::NomResult<&str, (&str, &str)> {
        context(
            "item",
            terminated(
                separated_pair(super::key, char('='), take_till(|c| c == ';')),
                opt(char(';')),
            ),
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

    context(
        "rrule",
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
                        super::datatype::date_or_dt(x)
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
                    .get("BYDAY")
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
        }),
    )(input.value.as_str())
    .map(|(_, x)| x)
    .map_err(crate::Error::from)
}
