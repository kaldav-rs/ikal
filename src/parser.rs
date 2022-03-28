use nom::character::complete::{char, line_ending};
use nom::bytes::complete::{tag, take_while, take_till, take_until};
use nom::combinator::{map, not, opt};
use nom::multi::many0;
use nom::branch::alt;
use nom::sequence::tuple;

fn is_alphabetic(chr: char) -> bool {
    (chr as u8 >= 0x41 && chr as u8 <= 0x5A)
        || (chr as u8 >= 0x61 && chr as u8 <= 0x7A)
}

fn is_digit(chr: char) -> bool {
    chr as u8 >= 0x30 && chr as u8 <= 0x39
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

fn key(input: &str) -> nom::IResult<&str, &str>
{
    take_while(is_alphanumeric)(input)
}

fn attr(input: &str) -> nom::IResult<&str, &str>
{
    take_while(is_alphanumeric)(input)
}

fn value_line(input: &str) -> nom::IResult<&str, &str>
{
    take_till(is_line_ending)(input)
}

fn value_part(input: &str) -> nom::IResult<&str, String>
{
    let (input, (value, _, _)) = tuple((
        value_line,
        line_ending,
        tag(" "),
    ))(input)?;

    Ok((input, value.to_string()))
}

fn value(input: &str) -> nom::IResult<&str, String>
{
    let (input, (value, value_end)) = tuple((
        many0(value_part),
        value_line,
    ))(input)?;

    Ok((input, value.join("") + value_end))
}

fn param(input: &str) -> nom::IResult<&str, (String, String)>
{
    let (input, (_, key, _, attr)) = tuple((
        char(';'),
        key,
        char('='),
        attr,
    ))(input)?;

    Ok((input, (key.into(), attr.into())))
}

pub fn property(input: &str) -> nom::IResult<&str, (String, String)>
{
    let (input, (_, _, key, _, _, value, _)) = tuple((
        not(tag("BEGIN")),
        not(tag("END")),
        key,
        many0(param),
        char(':'),
        opt(value),
        line_ending,
    ))(input)?;

    let result = (key.into(), if let Some(value) = value {
        value
    } else {
        String::new()
    });

    Ok((input, result))
}

pub fn properties(input: &str) -> nom::IResult<&str, std::collections::BTreeMap<String, String>>
{
    let (input, values) = many0(property)(input)?;

    let mut hash = std::collections::BTreeMap::new();

    for (key, value) in values {
        hash.insert(key, value);
    }

    Ok((input, hash))
}

pub fn parse_vevent(input: &str) -> nom::IResult<&str, Result<crate::VEvent, String>>
{
    let (input, (_, _, value, _, _)) = tuple((
        tag("BEGIN:VEVENT"),
        line_ending,
        properties,
        tag("END:VEVENT"),
        line_ending,
    ))(input)?;

    Ok((input, value.try_into()))
}

pub fn parse_vtodo(input: &str) -> nom::IResult<&str, Result<crate::VTodo, String>>
{
    let (input, (_, _, values, _, _)) = tuple((
        tag("BEGIN:VTODO"),
        line_ending,
        properties,
        tag("END:VTODO"),
        line_ending,
    ))(input)?;

    Ok((input, values.try_into()))
}

pub fn parse_content(input: &str) -> nom::IResult<&str, Result<crate::Content, String>>
{
    alt((
        map(parse_vevent, |x| x.map(crate::Content::Event)),
        map(parse_vtodo, |x| x.map(crate::Content::Todo)),
    ))(input)
}

pub fn parse_vcalendar(input: &str) -> nom::IResult<&str, Result<crate::VCalendar, String>>
{
    let (input, (_, _, values, content, _, _)) = tuple((
        tag("BEGIN:VCALENDAR"),
        line_ending,
        properties,
        parse_content,
        take_until("END:VCALENDAR"),
        tag("END:VCALENDAR"),
    ))(input)?;

    let calendar: Result<crate::VCalendar, String> = values.try_into();

    let result = match calendar {
        Ok(mut calendar) => {
            match content {
                Ok(content) => {
                    calendar.content = content;
                    Ok(calendar)
                },
                Err(err) => Err(err),
            }
        },
        Err(err) => Err(err),
    };

    Ok((input, result))
}
