mod change;
mod datetime;
mod descriptive;
mod misc;
mod recurrence;
mod relationship;
mod timezone;

pub(crate) use change::*;
pub(crate) use datetime::*;
pub(crate) use descriptive::*;
pub(crate) use misc::*;
pub(crate) use recurrence::*;
pub(crate) use relationship::*;
pub(crate) use timezone::*;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_till, take_while};
use nom::character::complete::{anychar, char, line_ending};
use nom::combinator::{map, map_res, not, opt};
use nom::multi::{count, fold_many0, many0};
use nom::sequence::{delimited, preceded, separated_pair, terminated, tuple};

fn is_alphabetic(chr: char) -> bool {
    nom::character::is_alphabetic(chr as u8)
}

fn is_digit(chr: char) -> bool {
    nom::character::is_digit(chr as u8)
}

fn is_sep(chr: char) -> bool {
    chr == '-' || chr == '/'
}

fn is_alphanumeric(chr: char) -> bool {
    is_alphabetic(chr) || is_digit(chr) || is_sep(chr)
}

fn is_line_ending(chr: char) -> bool {
    chr == '\n' || chr == '\r'
}

fn digits(input: &str) -> nom::IResult<&str, &str> {
    take_while(is_digit)(input)
}

fn key(input: &str) -> nom::IResult<&str, &str> {
    take_while(is_alphanumeric)(input)
}

fn attr(input: &str) -> nom::IResult<&str, &str> {
    take_while(is_alphanumeric)(input)
}

fn value_line(input: &str) -> nom::IResult<&str, &str> {
    take_till(is_line_ending)(input)
}

fn value_part(input: &str) -> nom::IResult<&str, &str> {
    map(
        tuple((value_line, line_ending, tag(" "))),
        |(value, _, _)| value,
    )(input)
}

fn value(input: &str) -> nom::IResult<&str, String> {
    map(
        tuple((many0(value_part), value_line)),
        |(value, value_end)| {
            let mut acc = String::new();

            for v in value {
                acc.push_str(v);
            }
            acc + value_end
        },
    )(input)
}

fn param(input: &str) -> nom::IResult<&str, (&str, &str)> {
    preceded(char(';'), separated_pair(key, char('='), attr))(input)
}

/**
 * See [3.1. Content Lines](https://datatracker.ietf.org/doc/html/rfc5545#section-3.1)
 */
pub(crate) fn content_line(input: &str) -> nom::IResult<&str, (&str, String)> {
    map(
        tuple((
            not(tag("BEGIN:")),
            not(tag("END:")),
            key,
            many0(param),
            char(':'),
            opt(value),
            line_ending,
        )),
        |(_, _, key, _, _, value, _)| (key, value.unwrap_or_default()),
    )(input)
}

pub(crate) fn content_lines(
    input: &str,
) -> nom::IResult<&str, std::collections::BTreeMap<String, String>> {
    fold_many0(
        content_line,
        std::collections::BTreeMap::new,
        |mut acc, (key, value)| {
            acc.insert(key.to_string(), value);
            acc
        },
    )(input)
}

pub(crate) fn vevent(input: &str) -> nom::IResult<&str, crate::VEvent> {
    map_res(
        delimited(
            tag("BEGIN:VEVENT\r\n"),
            content_lines,
            tag("END:VEVENT\r\n"),
        ),
        |values| values.try_into(),
    )(input)
}

pub(crate) fn vjournal(input: &str) -> nom::IResult<&str, crate::VJournal> {
    map_res(
        delimited(
            tag("BEGIN:VJOURNAL\r\n"),
            content_lines,
            tag("END:VJOURNAL\r\n"),
        ),
        |values| values.try_into(),
    )(input)
}

pub(crate) fn standard(input: &str) -> nom::IResult<&str, crate::vtimezone::Prop> {
    map_res(
        delimited(
            tag("BEGIN:STANDARD\r\n"),
            content_lines,
            tag("END:STANDARD\r\n"),
        ),
        |values| values.try_into(),
    )(input)
}

pub(crate) fn daylight(input: &str) -> nom::IResult<&str, crate::vtimezone::Prop> {
    map_res(
        delimited(
            tag("BEGIN:DAYLIGHT\r\n"),
            content_lines,
            tag("END:DAYLIGHT\r\n"),
        ),
        |values| values.try_into(),
    )(input)
}

pub(crate) fn prop(_: &str) -> nom::IResult<&str, crate::vtimezone::Prop> {
    unreachable!()
}

pub(crate) fn vtimezone(input: &str) -> nom::IResult<&str, crate::VTimezone> {
    map_res(
        delimited(
            tag("BEGIN:VTIMEZONE\r\n"),
            tuple((
                content_lines,
                many0(alt((
                    map(standard, crate::vtimezone::Component::Standard),
                    map(daylight, crate::vtimezone::Component::Daylight),
                ))),
            )),
            tag("END:VTIMEZONE\r\n"),
        ),
        |(values, components)| {
            let mut vtimezone: crate::VTimezone = values.try_into()?;

            for component in components {
                match component {
                    crate::vtimezone::Component::Standard(standard) => {
                        vtimezone.standard.push(standard)
                    }
                    crate::vtimezone::Component::Daylight(daylight) => {
                        vtimezone.daylight.push(daylight)
                    }
                }
            }

            Ok::<_, crate::Error>(vtimezone)
        },
    )(input)
}

pub(crate) fn vtodo(input: &str) -> nom::IResult<&str, crate::VTodo> {
    map_res(
        delimited(tag("BEGIN:VTODO\r\n"), content_lines, tag("END:VTODO\r\n")),
        |values| values.try_into(),
    )(input)
}

pub(crate) fn component(input: &str) -> nom::IResult<&str, crate::Component> {
    alt((
        map(vevent, crate::Component::Event),
        map(vjournal, crate::Component::Journal),
        map(vtimezone, crate::Component::Timezone),
        map(vtodo, crate::Component::Todo),
    ))(input)
}

pub(crate) fn components(input: &str) -> nom::IResult<&str, Vec<crate::Component>> {
    many0(component)(input)
}

pub(crate) fn vcalendar(input: &str) -> nom::IResult<&str, crate::VCalendar> {
    map_res(
        delimited(
            tag("BEGIN:VCALENDAR\r\n"),
            tuple((content_lines, components)),
            tag("END:VCALENDAR\r\n"),
        ),
        |(content_lines, components)| {
            let mut vcalendar: crate::VCalendar = content_lines.try_into()?;

            for component in components {
                match component {
                    crate::Component::Event(event) => vcalendar.events.push(event),
                    crate::Component::Journal(journal) => vcalendar.journals.push(journal),
                    crate::Component::Todo(todo) => vcalendar.todo.push(todo),
                    crate::Component::Timezone(timezone) => vcalendar.timezones.push(timezone),
                }
            }

            Ok::<_, crate::Error>(vcalendar)
        },
    )(input)
}

pub(crate) fn weekday(input: &str) -> nom::IResult<&str, crate::Weekday> {
    use crate::Weekday::*;

    map_res(count(anychar, 2), |s| {
        let weekday = match s.as_slice() {
            ['S', 'U'] => Sunday,
            ['M', 'O'] => Monday,
            ['T', 'U'] => Tuesday,
            ['W', 'E'] => Wenesday,
            ['T', 'H'] => Thurday,
            ['F', 'R'] => Friday,
            ['S', 'A'] => Saturday,

            _ => return Err(crate::Error::Weekday(format!("{}{}", s[0], s[1]))),
        };

        Ok(weekday)
    })(input)
}

pub(crate) fn weekdaynum(input: &str) -> nom::IResult<&str, crate::WeekdayNum> {
    map(
        tuple((nom::character::complete::i8, weekday)),
        |(ord, weekday)| crate::WeekdayNum { weekday, ord },
    )(input)
}

/**
 * See [3.3.5. Date-Time](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.5)
 */
fn date<S>(date: S) -> crate::Result<crate::DateTime>
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
        map_res(terminated(digits, tag("W")), str::parse)(input)
    }

    fn day(input: &str) -> nom::IResult<&str, i64> {
        map_res(terminated(digits, tag("D")), str::parse)(input)
    }

    fn time(input: &str) -> nom::IResult<&str, (i64, i64, i64)> {
        let (input, (h, i, s)) =
            preceded(tag("H"), tuple((opt(hour), opt(minute), opt(seconde))))(input)?;

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
        map_res(terminated(digits, tag("H")), str::parse)(input)
    }

    fn minute(input: &str) -> nom::IResult<&str, i64> {
        map_res(terminated(digits, tag("M")), str::parse)(input)
    }

    fn seconde(input: &str) -> nom::IResult<&str, i64> {
        map_res(terminated(digits, tag("S")), str::parse)(input)
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
 * See [3.7.1. Calendar Scale](https://datatracker.ietf.org/doc/html/rfc5545#section-3.7.1)
 */
pub(crate) fn calscale(input: &str) -> crate::Result<String> {
    Ok(input.to_string())
}

/**
 * See [3.7.2. Method](https://datatracker.ietf.org/doc/html/rfc5545#section-3.7.2)
 */
pub(crate) fn method(input: &str) -> crate::Result<String> {
    Ok(input.to_string())
}

/**
 * See [3.7.3. Product Identifier](https://datatracker.ietf.org/doc/html/rfc5545#section-3.7.3)
 */
pub(crate) fn prodid(input: &str) -> crate::Result<String> {
    Ok(input.to_string())
}

/**
 * See [3.7.4. Version](https://datatracker.ietf.org/doc/html/rfc5545#section-3.7.4)
 */
pub(crate) fn version(input: &str) -> crate::Result<String> {
    Ok(input.to_string())
}
