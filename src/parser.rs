use nom::branch::alt;
use nom::bytes::complete::{tag, take_till, take_while};
use nom::character::complete::{anychar, char, line_ending};
use nom::combinator::{map, map_res, not, opt};
use nom::multi::{count, fold_many0, many0, many1};
use nom::number::complete::float;
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
                acc.push_str(&v);
            }
            acc + &value_end
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

pub(crate) fn parse_vevent(input: &str) -> nom::IResult<&str, crate::VEvent> {
    map_res(
        delimited(
            tag("BEGIN:VEVENT\r\n"),
            content_lines,
            tag("END:VEVENT\r\n"),
        ),
        |values| values.try_into(),
    )(input)
}

pub(crate) fn parse_standard(input: &str) -> nom::IResult<&str, crate::vtimezone::Prop> {
    map_res(
        delimited(
            tag("BEGIN:STANDARD\r\n"),
            content_lines,
            tag("END:STANDARD\r\n"),
        ),
        |values| values.try_into(),
    )(input)
}

pub(crate) fn parse_daylight(input: &str) -> nom::IResult<&str, crate::vtimezone::Prop> {
    map_res(
        delimited(
            tag("BEGIN:DAYLIGHT\r\n"),
            content_lines,
            tag("END:DAYLIGHT\r\n"),
        ),
        |values| values.try_into(),
    )(input)
}

pub(crate) fn parse_vtimezone(input: &str) -> nom::IResult<&str, crate::VTimezone> {
    map_res(
        delimited(
            tag("BEGIN:VTIMEZONE\r\n"),
            tuple((
                content_lines,
                many0(alt((
                    map(parse_standard, crate::vtimezone::Component::Standard),
                    map(parse_daylight, crate::vtimezone::Component::Daylight),
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

pub(crate) fn parse_vtodo(input: &str) -> nom::IResult<&str, crate::VTodo> {
    map_res(
        delimited(tag("BEGIN:VTODO\r\n"), content_lines, tag("END:VTODO\r\n")),
        |values| values.try_into(),
    )(input)
}

pub(crate) fn parse_component(input: &str) -> nom::IResult<&str, crate::Component> {
    alt((
        map(parse_vevent, crate::Component::Event),
        map(parse_vtimezone, crate::Component::Timezone),
        map(parse_vtodo, crate::Component::Todo),
    ))(input)
}

pub(crate) fn parse_components(input: &str) -> nom::IResult<&str, Vec<crate::Component>> {
    many0(parse_component)(input)
}

pub(crate) fn parse_vcalendar(input: &str) -> nom::IResult<&str, crate::VCalendar> {
    map_res(
        delimited(
            tag("BEGIN:VCALENDAR\r\n"),
            tuple((content_lines, parse_components)),
            tag("END:VCALENDAR\r\n"),
        ),
        |(content_lines, components)| {
            let mut vcalendar: crate::VCalendar = content_lines.try_into()?;

            for component in components {
                match component {
                    crate::Component::Event(event) => vcalendar.events.push(event),
                    crate::Component::Todo(todo) => vcalendar.todo.push(todo),
                    crate::Component::Timezone(timezone) => vcalendar.timezones.push(timezone),
                }
            }

            Ok::<_, crate::Error>(vcalendar)
        },
    )(input)
}

pub(crate) fn parse_weekday(input: &str) -> nom::IResult<&str, crate::Weekday> {
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

pub(crate) fn parse_weekdaynum(input: &str) -> nom::IResult<&str, crate::WeekdayNum> {
    map(
        tuple((nom::character::complete::i8, parse_weekday)),
        |(ord, weekday)| crate::WeekdayNum { weekday, ord },
    )(input)
}

/**
 * See [3.3.5. Date-Time](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.5)
 */
pub(crate) fn parse_date<S>(date: S) -> crate::Result<crate::DateTime>
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
pub(crate) fn parse_duration(input: &str) -> crate::Result<chrono::Duration> {
    fn week(input: &str) -> nom::IResult<&str, i64> {
        map_res(terminated(digits, tag("W")), |x| str::parse(&x))(input)
    }

    fn day(input: &str) -> nom::IResult<&str, i64> {
        map_res(terminated(digits, tag("D")), |x| str::parse(&x))(input)
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
        map_res(terminated(digits, tag("H")), |x| str::parse(&x))(input)
    }

    fn minute(input: &str) -> nom::IResult<&str, i64> {
        map_res(terminated(digits, tag("M")), |x| str::parse(&x))(input)
    }

    fn seconde(input: &str) -> nom::IResult<&str, i64> {
        map_res(terminated(digits, tag("S")), |x| str::parse(&x))(input)
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
pub(crate) fn parse_attach(input: &str) -> String {
    input.to_string()
}

/**
 * See [3.8.1.2. Categories](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.2)
 */
pub(crate) fn parse_categories(input: &str) -> Vec<String> {
    input.split(',').map(String::from).collect()
}

/**
 * See [3.8.1.4. Comment](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.4)
 */
pub(crate) fn parse_comment(input: &str) -> String {
    input.to_string()
}

/**
 * See [3.8.1.6. Geographic Position](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.6)
 */
pub(crate) fn parse_geo(input: &str) -> nom::IResult<&str, crate::Geo> {
    map(separated_pair(float, char(';'), float), |(lat, lon)| {
        crate::Geo { lat, lon }
    })(input)
}

/**
 * See [3.8.1.9. Priority](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.9)
 */
pub(crate) fn parse_priority(input: &str) -> crate::Result<u8> {
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
pub(crate) fn parse_resources(input: &str) -> Vec<String> {
    input.split(',').map(String::from).collect()
}

/**
 * See [3.8.3.3. Time Zone Offset From](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.3.3)
 * See [3.8.3.4. Time Zone Offset To](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.3.4)
 */
pub(crate) fn parse_tzoffset(input: &str) -> crate::Result<chrono::offset::FixedOffset> {
    input.parse().map_err(crate::Error::from)
}

/**
 * See [3.8.4.1. Attendee](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.4.1)
 */
pub(crate) fn parse_attendee(input: &str) -> String {
    input.to_string()
}

/**
 * See [3.8.4.2. Contact](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.4.2)
 */
pub(crate) fn parse_contact(input: &str) -> String {
    input.to_string()
}

/**
 * See [3.8.4.3. Organizer](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.4.3)
 */
pub(crate) fn parse_organizer(input: &str) -> crate::Result<String> {
    // @TODO
    Ok(input.to_string())
}

/**
 * See [3.8.4.4. Recurrence ID](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.4.4)
 */
pub(crate) fn parse_recurid(input: &str) -> crate::Result<String> {
    // @TODO
    Ok(input.to_string())
}

/**
 * See [3.8.4.5. Related To](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.4.5)
 */
pub(crate) fn parse_related(input: &str) -> String {
    input.to_string()
}

/**
 * See [3.8.5.1. Exception Date-Times](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.5.1)
 */
pub(crate) fn parse_exdate(input: &str) -> crate::Result<Vec<crate::DateTime>> {
    input.split(',').map(parse_date).collect()
}

/**
 * See [3.8.5.2. Recurrence Date-Times](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.5.2)
 */
pub(crate) fn parse_rdate(input: &str) -> crate::Result<Vec<String>> {
    // @TODO
    Ok(input.split(',').map(String::from).collect())
}

/**
 * See [3.8.5.3. Recurrence Rule](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.5.3)
 */
pub(crate) fn parse_rrule(input: &str) -> nom::IResult<&str, crate::Recur> {
    fn item(input: &str) -> nom::IResult<&str, (&str, &str)> {
        terminated(separated_pair(key, char('='), key), opt(char(';')))(input)
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
        let map =
            std::collections::BTreeMap::from_iter(items.iter().map(|(k, v)| (k.to_string(), v)));

        let recur = crate::Recur {
            freq: map["FREQ"].parse()?,
            until: map
                .get("UNTIL")
                .map(|x| parse_date(x.to_string()))
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
}

/**
 * See [3.8.7.4. Sequence Number](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.7.4)
 */
pub(crate) fn parse_sequence(input: &str) -> crate::Result<u32> {
    input.parse().map_err(crate::Error::from)
}

/**
 * See [3.8.8.3. Request Status](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.8.3)
 */
pub(crate) fn parse_rstatus(input: &str) -> crate::Result<String> {
    // @TODO
    Ok(input.to_string())
}
