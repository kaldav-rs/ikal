/*!
 * See [3.3. Property Value Data Types](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.5)
 */

use nom::Parser as _;
use nom::bytes::complete::tag;
use nom::combinator::{map, map_res, opt};
use nom::error::{FromExternalError, context};
use nom::sequence::{pair, preceded, terminated};

/**
 * See [3.3.3. Calendar User Address](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.3)
 */
pub(crate) fn cal_address(input: &str) -> crate::Result<String> {
    uri(input)
}

/**
 * See [3.3.4. Date](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.4)
 */
pub(crate) fn date(input: &str) -> super::NomResult<&str, chrono::NaiveDate> {
    let date = chrono::NaiveDate::parse_from_str(input, "%Y%m%d").map_err(|e| {
        nom::Err::Error(nom_language::error::VerboseError::from_external_error(
            input,
            nom::error::ErrorKind::Fail,
            e,
        ))
    })?;

    Ok(("", date))
}

/**
 * See [3.3.5. Date-Time](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.5)
 */
pub(crate) fn date_time(input: &str) -> super::NomResult<&str, crate::DateTime> {
    let date = input.to_string();

    let dt = chrono::NaiveDateTime::parse_from_str(date.trim_end_matches('Z'), "%Y%m%dT%H%M%S")
        .map_err(|e| {
            nom::Err::Error(nom_language::error::VerboseError::from_external_error(
                input,
                nom::error::ErrorKind::Fail,
                e,
            ))
        })?;

    if date.ends_with('Z') {
        Ok((
            "",
            crate::DateTime::Local(dt.and_utc().with_timezone(&chrono::Local)),
        ))
    } else {
        Ok(("", crate::DateTime::Naive(dt)))
    }
}

pub(crate) fn date_or_dt(input: &str) -> super::NomResult<&str, crate::Date> {
    context(
        "date_or_dt",
        nom::branch::alt((
            map(date, crate::Date::Date),
            map(date_time, crate::Date::DateTime),
        )),
    )
    .parse(input)
}

/**
 * See [3.3.6. Duration](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.6)
 */
pub(crate) fn duration(input: &str) -> super::NomResult<&str, chrono::Duration> {
    fn week(input: &str) -> super::NomResult<&str, i64> {
        context(
            "week",
            map_res(terminated(super::digits, tag("W")), str::parse),
        )
        .parse(input)
    }

    fn day(input: &str) -> super::NomResult<&str, i64> {
        context(
            "day",
            map_res(terminated(super::digits, tag("D")), str::parse),
        )
        .parse(input)
    }

    fn time(input: &str) -> super::NomResult<&str, (i64, i64, i64)> {
        let (input, (h, i, s)) = context(
            "time",
            preceded(tag("T"), (opt(hour), opt(minute), opt(seconde))),
        )
        .parse(input)?;

        Ok((
            input,
            (
                h.unwrap_or_default(),
                i.unwrap_or_default(),
                s.unwrap_or_default(),
            ),
        ))
    }

    fn hour(input: &str) -> super::NomResult<&str, i64> {
        context(
            "hour",
            map_res(terminated(super::digits, tag("H")), str::parse),
        )
        .parse(input)
    }

    fn minute(input: &str) -> super::NomResult<&str, i64> {
        context(
            "minute",
            map_res(terminated(super::digits, tag("M")), str::parse),
        )
        .parse(input)
    }

    fn seconde(input: &str) -> super::NomResult<&str, i64> {
        context(
            "seconde",
            map_res(terminated(super::digits, tag("S")), str::parse),
        )
        .parse(input)
    }

    context(
        "duration",
        map(
            pair(
                opt(tag("-")),
                preceded(tag("P"), (opt(week), opt(day), opt(time))),
            ),
            |(neg, (w, d, t))| {
                let mut duration = chrono::Duration::weeks(w.unwrap_or_default())
                    + chrono::Duration::days(d.unwrap_or_default());

                if let Some((h, i, s)) = t {
                    duration = duration
                        + chrono::Duration::hours(h)
                        + chrono::Duration::minutes(i)
                        + chrono::Duration::seconds(s);
                }

                if neg.is_some() { -duration } else { duration }
            },
        ),
    )
    .parse(input)
}

/**
 * See [3.3.9. Period of Time](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.9)
 */
pub(crate) fn period(input: &str) -> crate::Result<crate::Period> {
    let tokens = input.splitn(2, '/').collect::<Vec<_>>();

    let start = date_time(tokens[0])?.1;

    let period = if tokens[1].starts_with('P') {
        crate::Period::StartDur(crate::period::StartDur {
            start,
            duration: super::duration(tokens[1].into())?,
        })
    } else {
        crate::Period::StartEnd(crate::period::StartEnd {
            start,
            end: date_time(tokens[1])?.1,
        })
    };

    Ok(period)
}

/**
 * See [3.3.11. Text](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.11)
 */
pub(crate) fn text(input: crate::ContentLine) -> crate::Result<crate::Text> {
    Ok(input.into())
}

/**
 * See [3.3.13. URI](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.13)
 */
pub(crate) fn uri(input: &str) -> crate::Result<String> {
    Ok(input.to_string())
}
