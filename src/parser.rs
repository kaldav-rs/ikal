use nom::{character::complete::line_ending, take_while, take_till, tag, do_parse, not, opt, many0, alt, char, take_until};
use std::convert::TryInto;

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
    take_while!(input, is_alphanumeric)
}

fn attr(input: &str) -> nom::IResult<&str, &str>
{
    take_while!(input, is_alphanumeric)
}

fn value_line(input: &str) -> nom::IResult<&str, &str>
{
    take_till!(input, is_line_ending)
}

fn value_part(input: &str) -> nom::IResult<&str, String>
{
    do_parse!(input,
        value_part:
            value_line >>
            line_ending >>
            tag!(" ") >>

        (value_part.into())
    )
}

fn value(input: &str) -> nom::IResult<&str, String>
{
    do_parse!(input,
        value:
            many0!(value_part) >>
        value_end:
            value_line >>

        (value.join("") + value_end)
    )
}

fn param(input: &str) -> nom::IResult<&str, (String, String)>
{
    do_parse!(input,
        char!(';') >>
        key:
            key >>
            char!('=') >>
        attr:
            attr >>

        (key.into(), attr.into())
    )
}

pub fn property(input: &str) -> nom::IResult<&str, (String, String)>
{
    do_parse!(input,
            not!(tag!("BEGIN")) >>
            not!(tag!("END")) >>
        key:
            key >>
            many0!(param) >>
            char!(':') >>
        value:
            opt!(value) >>
            line_ending >>

        (key.into(), if let Some(value) = value {
            value
        } else {
            String::new()
        })
    )
}

pub fn properties(input: &str) -> nom::IResult<&str, std::collections::BTreeMap<String, String>>
{
    do_parse!(input,
        values: many0!(property) >>

        ({
            let mut hash = std::collections::BTreeMap::new();

            for (key, value) in values {
                hash.insert(key, value);
            }

            hash
        })
    )
}

pub fn parse_vevent(input: &str) -> nom::IResult<&str, Result<crate::VEvent, String>>
{
    do_parse!(input,
            tag!("BEGIN:VEVENT") >>
            line_ending >>
        values:
            properties >>
            tag!("END:VEVENT") >>
            line_ending >>

        (values.try_into())
    )
}

pub fn parse_vtodo(input: &str) -> nom::IResult<&str, Result<crate::VTodo, String>>
{
    do_parse!(input,
            tag!("BEGIN:VTODO") >>
            line_ending >>
        values:
            properties >>
            tag!("END:VTODO") >>
            line_ending >>

        (values.try_into())
    )
}

pub fn parse_content(input: &str) -> nom::IResult<&str, Result<crate::Content, String>>
{
    alt!(input,
        parse_vevent => { |event| match event {
            Ok(event) => Ok(crate::Content::Event(event)),
            Err(err) => Err(err),
        }} |
        parse_vtodo => { |todo| match todo {
            Ok(todo) => Ok(crate::Content::Todo(todo)),
            Err(err) => Err(err),
        }}
    )
}

pub fn parse_vcalendar(input: &str) -> nom::IResult<&str, Result<crate::VCalendar, String>>
{
    do_parse!(input,
            tag!("BEGIN:VCALENDAR") >>
            line_ending >>
        values:
            properties >>
        content:
            parse_content >>
            take_until!("END:VCALENDAR") >>
            tag!("END:VCALENDAR") >>

        ({
            let calendar: Result<crate::VCalendar, String> = values.try_into();

            match calendar {
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
            }
        })
    )
}
