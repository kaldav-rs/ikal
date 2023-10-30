use nom::branch::alt;
use nom::bytes::complete::{tag, take_till, take_until, take_while};
use nom::character::complete::{anychar, char, line_ending};
use nom::combinator::{map, map_res, not, opt};
use nom::multi::{count, many0, many1};
use nom::number::complete::float;
use nom::sequence::{preceded, separated_pair, terminated, tuple};

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

fn value_part(input: &str) -> nom::IResult<&str, String> {
    let (input, (value, _, _)) = tuple((value_line, line_ending, tag(" ")))(input)?;

    Ok((input, value.to_string()))
}

fn value(input: &str) -> nom::IResult<&str, String> {
    let (input, (value, value_end)) = tuple((many0(value_part), value_line))(input)?;

    Ok((input, value.join("") + value_end))
}

fn param(input: &str) -> nom::IResult<&str, (String, String)> {
    let (input, (key, attr)) = preceded(char(';'), separated_pair(key, char('='), attr))(input)?;

    Ok((input, (key.into(), attr.into())))
}

pub fn property(input: &str) -> nom::IResult<&str, (String, String)> {
    let (input, (_, _, key, _, _, value, _)) = tuple((
        not(tag("BEGIN")),
        not(tag("END")),
        key,
        many0(param),
        char(':'),
        opt(value),
        line_ending,
    ))(input)?;

    let result = (
        key.into(),
        if let Some(value) = value {
            value
        } else {
            String::new()
        },
    );

    Ok((input, result))
}

pub fn properties(input: &str) -> nom::IResult<&str, std::collections::BTreeMap<String, String>> {
    let (input, values) = many0(property)(input)?;

    let mut hash = std::collections::BTreeMap::new();

    for (key, value) in values {
        hash.insert(key, value);
    }

    Ok((input, hash))
}

pub fn parse_vevent(input: &str) -> nom::IResult<&str, crate::VEvent> {
    map_res(
        tuple((
            tag("BEGIN:VEVENT"),
            line_ending,
            properties,
            tag("END:VEVENT"),
            line_ending,
        )),
        |(_, _, values, _, _)| values.try_into(),
    )(input)
}

pub fn parse_vtodo(input: &str) -> nom::IResult<&str, crate::VTodo> {
    map_res(
        tuple((
            tag("BEGIN:VTODO"),
            line_ending,
            properties,
            tag("END:VTODO"),
            line_ending,
        )),
        |(_, _, values, _, _)| values.try_into(),
    )(input)
}

pub fn parse_component(input: &str) -> nom::IResult<&str, crate::Component> {
    alt((
        map(parse_vevent, crate::Component::Event),
        map(parse_vtodo, crate::Component::Todo),
    ))(input)
}

pub fn parse_vcalendar(input: &str) -> nom::IResult<&str, crate::VCalendar> {
    map_res(
        tuple((
            tag("BEGIN:VCALENDAR"),
            line_ending,
            properties,
            parse_component,
            take_until("END:VCALENDAR"),
            tag("END:VCALENDAR"),
        )),
        |(_, _, values, component, _, _)| {
            let calendar: crate::Result<crate::VCalendar> = values.try_into();

            match calendar {
                Ok(mut calendar) => {
                    calendar.component = component;
                    Ok(calendar)
                }
                Err(err) => Err(err),
            }
        }
    )(input)
}

pub fn parse_weekday(input: &str) -> nom::IResult<&str, crate::Weekday> {
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

pub fn parse_weekdaynum(input: &str) -> nom::IResult<&str, crate::WeekdayNum> {
    map(
        tuple((nom::character::complete::i8, parse_weekday)),
        |(ord, weekday)| crate::WeekdayNum { weekday, ord },
    )(input)
}

/**
 * See [3.3.5. Date-Time](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.5)
 */
pub fn parse_date<S>(date: S) -> crate::Result<crate::DateTime>
where
    S: Into<String>,
{
    let mut date = date.into();

    if date.len() == 8 {
        date.push_str("T000000");
    }

    let dt = chrono::NaiveDateTime::parse_from_str(
        date.as_str().trim_end_matches('Z'),
        "%Y%m%dT%H%M%S",
    )?;

    if date.ends_with('Z') {
        Ok(dt.and_utc().with_timezone(&chrono::Local))
    } else {
        Ok(dt.and_local_timezone(chrono::Local).unwrap())
    }
}

/**
 * See [3.3.6. Duration](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.6)
 */
pub fn parse_duration(input: &str) -> crate::Result<chrono::Duration> {
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
 * See [3.8.1.1. Attachment](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.1)
 */
pub fn parse_attach(input: &str) -> String {
    input.to_string()
}

/**
 * See [3.8.1.2. Categories](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.2)
 */
pub fn parse_categories(input: &str) -> Vec<String> {
    input.split(',').map(String::from).collect()
}

/**
 * See [3.8.1.4. Comment](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.4)
 */
pub fn parse_comment(input: &str) -> String {
    input.to_string()
}

/**
 * See [3.8.1.6. Geographic Position](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.6)
 */
pub fn parse_geo(input: &str) -> nom::IResult<&str, crate::Geo> {
    map(separated_pair(float, char(';'), float), |(lat, lon)| {
        crate::Geo { lat, lon }
    })(input)
}

/**
 * See [3.8.1.9. Priority](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.9)
 */
pub fn parse_priority(input: &str) -> crate::Result<u8> {
    let priority = input.parse()?;

    if priority > 9 {
        Err(crate::Error::Priority(priority))
    } else {
        Ok(priority)
    }
}

/**
 * See [3.8.1.10. Resources](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.10)
 */
pub fn parse_resources(input: &str) -> Vec<String> {
    input.split(',').map(String::from).collect()
}

/**
 * See [3.8.4.1. Attendee](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.4.1)
 */
pub fn parse_attendee(input: &str) -> String {
    input.to_string()
}

/**
 * See [3.8.4.2. Contact](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.4.2)
 */
pub fn parse_contact(input: &str) -> String {
    input.to_string()
}

/**
 * See [3.8.4.3. Organizer](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.4.3)
 */
pub fn parse_organizer(input: &str) -> crate::Result<String> {
    // @TODO
    Ok(input.to_string())
}

/**
 * See [3.8.4.4. Recurrence ID](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.4.4)
 */
pub fn parse_recurid(input: &str) -> crate::Result<String> {
    // @TODO
    Ok(input.to_string())
}

/**
 * See [3.8.4.5. Related To](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.4.5)
 */
pub fn parse_related(input: &str) -> String {
    input.to_string()
}

/**
 * See [3.8.5.1. Exception Date-Times](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.5.1)
 */
pub fn parse_exdate(input: &str) -> crate::Result<Vec<crate::DateTime>> {
    input.split(',').map(parse_date).collect()
}

/**
 * See [3.8.5.2. Recurrence Date-Times](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.5.2)
 */
pub fn parse_rdate(input: &str) -> crate::Result<Vec<String>> {
    // @TODO
    Ok(input.split(',').map(String::from).collect())
}

/**
 * See [3.8.5.3. Recurrence Rule](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.5.3)
 */
pub fn parse_rrule(input: &str) -> nom::IResult<&str, crate::Recur> {
    fn item(input: &str) -> nom::IResult<&str, (&str, String)> {
        terminated(separated_pair(key, char('='), value), opt(char(';')))(input)
    }

    fn by(input: &str) -> crate::Result<Vec<i8>> {
        input
            .split(',')
            .map(str::parse)
            .collect::<Result<_, _>>()
            .map_err(crate::Error::from)
    }

    fn bywdaylist(input: &str) -> crate::Result<Vec<crate::WeekdayNum>> {
        input
            .split(',')
            .map(str::parse)
            .collect::<Result<_, _>>()
            .map_err(crate::Error::from)
    }

    map_res(many1(item), |items| {
        let map = std::collections::BTreeMap::from_iter(items);

        let recur = crate::Recur {
            freq: map["FREQ"].parse()?,
            until: map.get("UNTIL").map(parse_date).transpose()?,
            count: map.get("COUNT").map(|x| x.parse()).transpose()?,
            interval: map.get("INTERVAL").map(|x| x.parse()).transpose()?,
            by_second: map.get("BYSECOND").map(|x| by(x)).transpose()?.unwrap_or_default(),
            by_minute: map.get("BYMINUTE").map(|x| by(x)).transpose()?.unwrap_or_default(),
            by_hour: map.get("BYHOUR").map(|x| by(x)).transpose()?.unwrap_or_default(),
            by_day: map
                .get("BYHOUR")
                .map(|x| bywdaylist(x))
                .transpose()?
                .unwrap_or_default(),
                by_monthday: map
                    .get("BYMONTHDAY")
                    .map(|x| by(x))
                    .transpose()?
                    .unwrap_or_default(),
                    by_yearday: map
                        .get("BYYEARDAY")
                        .map(|x| by(x))
                        .transpose()?
                        .unwrap_or_default(),
                        by_weekno: map.get("BYWEEKNO").map(|x| by(x)).transpose()?.unwrap_or_default(),
                        by_month: map.get("BYMONTH").map(|x| by(x)).transpose()?.unwrap_or_default(),
                        by_setpos: map.get("BYSETPOS").map(|x| by(x)).transpose()?.unwrap_or_default(),
                        wkst: map.get("WKST").map(|x| x.parse()).transpose()?,
        };

        Ok::<_, crate::Error>(recur)
    })(input)
}

/**
 * See [3.8.7.4. Sequence Number](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.7.4)
 */
pub fn parse_sequence(input: &str) -> crate::Result<u32> {
    input.parse().map_err(crate::Error::from)
}

/**
 * See [3.8.8.3. Request Status](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.8.3)
 */
pub fn parse_rstatus(input: &str) -> crate::Result<String> {
    // @TODO
    Ok(input.to_string())
}
