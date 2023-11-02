use nom::bytes::complete::tag;
use nom::combinator::{map_res, opt};
use nom::sequence::{preceded, terminated, tuple};

/**
 * See [3.3. Property Value Data Types](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.5)
 */

/**
 * See [3.3.3. Calendar User Address](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.3)
 */
pub(crate) fn cal_address(input: &str) -> crate::Result<String> {
    uri(input)
}

/**
 * See [3.3.5. Date-Time](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.5)
 */
pub(crate) fn date<S>(date: S) -> crate::Result<crate::DateTime>
where
    S: Into<String>,
{
    let mut date = date.into();

    if date.len() == 8 {
        date.push_str("T000000");
    }

    let dt = chrono::NaiveDateTime::parse_from_str(date.trim_end_matches('Z'), "%Y%m%dT%H%M%S")?;

    if date.ends_with('Z') {
        Ok(dt.and_utc().with_timezone(&chrono::Local))
    } else {
        Ok(dt.and_local_timezone(chrono::Local).unwrap())
    }
}

/**
 * See [3.3.6. Duration](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.6)
 */
pub(crate) fn duration(input: &str) -> crate::Result<chrono::Duration> {
    fn week(input: &str) -> nom::IResult<&str, i64> {
        map_res(terminated(super::digits, tag("W")), str::parse)(input)
    }

    fn day(input: &str) -> nom::IResult<&str, i64> {
        map_res(terminated(super::digits, tag("D")), str::parse)(input)
    }

    fn time(input: &str) -> nom::IResult<&str, (i64, i64, i64)> {
        let (input, (h, i, s)) =
            preceded(tag("T"), tuple((opt(hour), opt(minute), opt(seconde))))(input)?;

        Ok((
            input,
            (
                h.unwrap_or_default(),
                i.unwrap_or_default(),
                s.unwrap_or_default(),
            ),
        ))
    }

    fn hour(input: &str) -> nom::IResult<&str, i64> {
        map_res(terminated(super::digits, tag("H")), str::parse)(input)
    }

    fn minute(input: &str) -> nom::IResult<&str, i64> {
        map_res(terminated(super::digits, tag("M")), str::parse)(input)
    }

    fn seconde(input: &str) -> nom::IResult<&str, i64> {
        map_res(terminated(super::digits, tag("S")), str::parse)(input)
    }

    let (_, (w, d, t)) = preceded(tag("P"), tuple((opt(week), opt(day), opt(time))))(input)?;

    let mut duration = chrono::Duration::weeks(w.unwrap_or_default())
        + chrono::Duration::days(d.unwrap_or_default());

    if let Some((h, i, s)) = t {
        duration = duration
            + chrono::Duration::hours(h)
            + chrono::Duration::minutes(i)
            + chrono::Duration::seconds(s);
    }

    Ok(duration)
}

/**
 * See [3.3.9. Period of Time](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.9)
 */
pub(crate) fn period(input: &str) -> crate::Result<crate::Period> {
    let tokens = input.splitn(2, '/').collect::<Vec<_>>();

    let start = date(tokens[0])?;

    let period = if tokens[1].starts_with('P') {
        crate::Period::StartDur(
            crate::period::StartDur {
                start,
                duration: duration(tokens[1])?,
            }
        )
    } else {
        crate::Period::StartEnd(
            crate::period::StartEnd {
                start,
                end: date(tokens[1])?,
            }
        )
    };

    Ok(period)
}

/**
 * See [3.3.13. URI](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.13)
 */
pub(crate) fn uri(input: &str) -> crate::Result<String> {
    Ok(input.to_string())
}
