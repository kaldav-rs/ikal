pub(crate) mod datatype;

mod alarm;
mod change;
mod components;
mod datetime;
mod descriptive;
mod misc;
mod properties;
mod recurrence;
mod relationship;
mod timezone;

pub(crate) use alarm::*;
pub(crate) use change::*;
pub(crate) use components::*;
pub(crate) use datetime::*;
pub(crate) use descriptive::*;
pub(crate) use misc::*;
pub(crate) use properties::*;
pub(crate) use recurrence::*;
pub(crate) use relationship::*;
pub(crate) use timezone::*;

use nom::bytes::complete::{tag, take_till, take_while};
use nom::character::complete::{anychar, char, line_ending};
use nom::combinator::{map, map_res, not, opt};
use nom::error::context;
use nom::multi::{count, fold_many0, many0};
use nom::sequence::{preceded, separated_pair, tuple};
use std::collections::BTreeMap;

pub(crate) type NomResult<I, O> = nom::IResult<I, O, nom::error::VerboseError<I>>;

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

fn digits(input: &str) -> NomResult<&str, &str> {
    context(
        "digits",
        take_while(is_digit)
    )(input)
}

fn key(input: &str) -> NomResult<&str, &str> {
    context(
        "key",
        take_while(is_alphanumeric)
    )(input)
}

fn attr(input: &str) -> NomResult<&str, &str> {
    context(
        "attr",
        preceded(
            opt(tag("\r\n ")),
            take_till(|c| c == ';' || c == ':')
        )
    )(input)
}

fn value_line(input: &str) -> NomResult<&str, &str> {
    context(
        "value_line",
        take_till(is_line_ending)
    )(input)
}

fn value_part(input: &str) -> NomResult<&str, &str> {
    context(
        "value_part",
        map(
            tuple((value_line, line_ending, tag(" "))),
            |(value, _, _)| value,
        )
    )(input)
}

fn value(input: &str) -> NomResult<&str, String> {
    context(
        "value",
        map(
            tuple((many0(value_part), value_line)),
            |(value, value_end)| {
                let mut acc = String::new();

                for v in value {
                    acc.push_str(v);
                }
                acc + value_end
            },
        )
    )(input)
}

fn quote(chr: char) -> bool {
    chr == '"'
}

fn quoted_param(input: &str) -> NomResult<&str, (&str, &str)> {
    context(
        "quoted_param",
        separated_pair(key, char('='), take_till(quote))
    )(input)
}

fn param(input: &str) -> NomResult<&str, (&str, &str)> {
    use nom::branch::alt;
    use nom::sequence::delimited;

    context(
        "param",
        alt((
            delimited(tag(";\""), quoted_param, char('"')),
            preceded(char(';'), separated_pair(key, char('='), attr)),
        ))
    )(input)
}

fn params(input: &str) -> NomResult<&str, BTreeMap<String, String>> {
    context(
        "params",
        fold_many0(param, BTreeMap::new, |mut acc, (key, value)| {
            acc.insert(key.to_string(), value.to_string());
            acc
        })
    )(input)
}

/**
 * See [3.1. Content Lines](https://datatracker.ietf.org/doc/html/rfc5545#section-3.1)
 */
pub(crate) fn content_line(input: &str) -> NomResult<&str, (&str, crate::ContentLine)> {
    context(
        "content_line",
        map(
            tuple((
                not(tag("BEGIN:")),
                not(tag("END:")),
                key,
                params,
                char(':'),
                opt(value),
                line_ending,
            )),
            |(_, _, key, params, _, value, _)| {
                (
                    key,
                    crate::ContentLine {
                        params,
                        value: value.unwrap_or_default(),
                    },
                )
            },
        )
    )(input)
}

pub(crate) fn content_lines(
    input: &str,
) -> NomResult<&str, BTreeMap<String, crate::ContentLine>> {
    context(
        "content_lines",
        fold_many0(content_line, BTreeMap::new, |mut acc, (key, value)| {
            acc.insert(key.to_string(), value);
            acc
        })
    )(input)
}

pub(crate) fn weekday(input: &str) -> NomResult<&str, crate::Weekday> {
    use crate::Weekday::*;

    context(
        "weekday",
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
        })
    )(input)
}

pub(crate) fn weekdaynum(input: &str) -> NomResult<&str, crate::WeekdayNum> {
    context(
        "weekdaynum",
        map(
            tuple((nom::character::complete::i8, weekday)),
            |(ord, weekday)| crate::WeekdayNum { weekday, ord },
        )
    )(input)
}
