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

/**
 * See [3.8.1.1. Attachment](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.1)
 */
pub(crate) fn attach(input: &str) -> crate::Result<String> {
    Ok(input.to_string())
}

/**
 * See [3.8.1.2. Categories](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.2)
 */
pub(crate) fn categories(input: &str) -> crate::Result<Vec<String>> {
    Ok(input.split(',').map(String::from).collect())
}

/**
 * See [3.8.1.3. Classification](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.3)
 */
pub(crate) fn class(input: &str) -> crate::Result<crate::Class> {
    input.parse()
}

/**
 * See [3.8.1.4. Comment](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.4)
 */
pub(crate) fn comment(input: &str) -> crate::Result<String> {
    Ok(input.to_string())
}

/**
 * See [3.8.1.5. Description](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.5)
 */
pub(crate) fn description(input: &str) -> crate::Result<String> {
    Ok(input.to_string())
}

/**
 * See [3.8.1.6. Geographic Position](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.6)
 */
pub(crate) fn geo(input: &str) -> crate::Result<crate::Geo> {
    map(separated_pair(float, char(';'), float), |(lat, lon)| {
        crate::Geo { lat, lon }
    })(input)
    .map_err(crate::Error::from)
    .map(|(_, x)| x)
}

/**
 * See [3.8.1.7. Location](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.7)
 */
pub(crate) fn location(input: &str) -> crate::Result<String> {
    Ok(input.to_string())
}

/**
 * See [3.8.1.8. Percent Complete](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.8)
 */
pub(crate) fn percent_complete(input: &str) -> crate::Result<u8> {
    input.parse().map_err(crate::Error::from)
}

/**
 * See [3.8.1.9. Priority](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.9)
 */
pub(crate) fn priority(input: &str) -> crate::Result<u8> {
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
pub(crate) fn resources(input: &str) -> crate::Result<Vec<String>> {
    Ok(input.split(',').map(String::from).collect())
}

/**
 * See [3.8.1.11. Status](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.11)
 */
pub(crate) fn status(input: &str) -> crate::Result<crate::Status> {
    input.parse()
}

/**
 * See [3.8.1.12. Summary](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.12)
 */
pub(crate) fn summary(input: &str) -> crate::Result<String> {
    Ok(input.to_string())
}

/**
 * See [3.8.2.1. Date-Time Completed](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.1)
 */
pub(crate) fn completed(input: &str) -> crate::Result<crate::DateTime> {
    date(input)
}

/**
 * See [3.8.2.2. Date-Time End](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.2)
 */
pub(crate) fn dtend(input: &str) -> crate::Result<crate::DateTime> {
    date(input)
}

/**
 * See [3.8.2.3. Date-Time Due](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.3)
 */
pub(crate) fn due(input: &str) -> crate::Result<crate::DateTime> {
    date(input)
}

/**
 * See [3.8.2.4. Date-Time Start](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.4)
 */
pub(crate) fn dtstart(input: &str) -> crate::Result<crate::DateTime> {
    date(input)
}

/**
 * See [3.8.2.7. Time Transparency](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.7)
 */
pub(crate) fn transp(input: &str) -> crate::Result<crate::TimeTransparency> {
    input.parse()
}

/**
 * See [3.8.3.1. Time Zone Identifier](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.3.1)
 */
pub(crate) fn tzid(input: &str) -> crate::Result<String> {
    Ok(input.to_string())
}

/**
 * See [3.8.3.2. Time Zone Name](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.3.2)
 */
pub(crate) fn tzname(input: &str) -> crate::Result<String> {
    Ok(input.to_string())
}

/**
 * See [3.8.3.3. Time Zone Offset From](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.3.3)
 */
pub(crate) fn tzoffsetfrom(input: &str) -> crate::Result<chrono::offset::FixedOffset> {
    input.parse().map_err(crate::Error::from)
}

/**
 * See [3.8.3.4. Time Zone Offset To](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.3.4)
 */
pub(crate) fn tzoffsetto(input: &str) -> crate::Result<chrono::offset::FixedOffset> {
    input.parse().map_err(crate::Error::from)
}

/**
 * See [3.8.4.1. Attendee](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.4.1)
 */
pub(crate) fn attendee(input: &str) -> crate::Result<String> {
    Ok(input.to_string())
}

/**
 * See [3.8.4.2. Contact](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.4.2)
 */
pub(crate) fn contact(input: &str) -> crate::Result<String> {
    Ok(input.to_string())
}

/**
 * See [3.8.4.3. Organizer](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.4.3)
 */
pub(crate) fn organizer(input: &str) -> crate::Result<String> {
    // @TODO
    Ok(input.to_string())
}

/**
 * See [3.8.4.4. Recurrence ID](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.4.4)
 */
pub(crate) fn recurid(input: &str) -> crate::Result<String> {
    // @TODO
    Ok(input.to_string())
}

/**
 * See [3.8.4.5. Related To](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.4.5)
 */
pub(crate) fn related_to(input: &str) -> crate::Result<String> {
    Ok(input.to_string())
}

/**
 * See [3.8.4.6. Uniform Resource Locator](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.4.6)
 */
pub(crate) fn url(input: &str) -> crate::Result<String> {
    Ok(input.to_string())
}

/**
 * See [3.8.4.7. Unique Identifier](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.4.7)
 */
pub(crate) fn uid(input: &str) -> crate::Result<String> {
    Ok(input.to_string())
}

/**
 * See [3.8.5.1. Exception Date-Times](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.5.1)
 */
pub(crate) fn exdate(input: &str) -> crate::Result<Vec<crate::DateTime>> {
    input.split(',').map(date).collect()
}

/**
 * See [3.8.5.2. Recurrence Date-Times](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.5.2)
 */
pub(crate) fn rdate(input: &str) -> crate::Result<Vec<String>> {
    // @TODO
    Ok(input.split(',').map(String::from).collect())
}

/**
 * See [3.8.5.3. Recurrence Rule](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.5.3)
 */
pub(crate) fn rrule(input: &str) -> crate::Result<crate::Recur> {
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
            until: map.get("UNTIL").map(|x| date(x.to_string())).transpose()?,
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

/**
 * See [3.8.7.1. Date-Time Created](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.7.1)
 */
pub(crate) fn created(input: &str) -> crate::Result<crate::DateTime> {
    date(input)
}

/**
 * See [3.8.7.2. Date-Time Stamp](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.7.2)
 */
pub(crate) fn dtstamp(input: &str) -> crate::Result<crate::DateTime> {
    date(input)
}

/**
 * See [3.8.7.3. Last Modified](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.7.3)
 */
pub(crate) fn last_modified(input: &str) -> crate::Result<crate::DateTime> {
    date(input)
}

/**
 * See [3.8.7.4. Sequence Number](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.7.4)
 */
pub(crate) fn sequence(input: &str) -> crate::Result<u32> {
    input.parse().map_err(crate::Error::from)
}

/**
 * See [3.8.3.5. Time Zone URL](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.3.5)
 */
pub(crate) fn tzurl(input: &str) -> crate::Result<String> {
    Ok(input.to_string())
}

/**
 * See [3.8.8.3. Request Status](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.8.3)
 */
pub(crate) fn rstatus(input: &str) -> crate::Result<String> {
    // @TODO
    Ok(input.to_string())
}
