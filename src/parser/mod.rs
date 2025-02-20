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

use nom::Parser as _;
use nom::bytes::complete::{tag, take_till, take_while};
use nom::character::complete::{anychar, char, line_ending};
use nom::combinator::{map, map_res, not, opt};
use nom::error::context;
use nom::multi::{count, fold_many0};
use nom::sequence::{preceded, separated_pair};
use std::collections::BTreeMap;

pub(crate) type NomResult<I, O> = nom::IResult<I, O, nom_language::error::VerboseError<I>>;

fn is_alphabetic(chr: char) -> bool {
    nom::AsChar::is_alpha(chr)
}

fn is_digit(chr: char) -> bool {
    nom::AsChar::is_dec_digit(chr)
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
    context("digits", take_while(is_digit)).parse(input)
}

fn key(input: &str) -> NomResult<&str, &str> {
    context("key", take_while(is_alphanumeric)).parse(input)
}

fn attr(input: &str) -> NomResult<&str, &str> {
    context(
        "attr",
        preceded(opt(tag("\r\n ")), take_till(|c| c == ';' || c == ':')),
    )
    .parse(input)
}

fn value(input: &str) -> NomResult<&str, &str> {
    context("value", take_till(is_line_ending)).parse(input)
}

fn quote(chr: char) -> bool {
    chr == '"'
}

fn quoted_param(input: &str) -> NomResult<&str, (&str, &str)> {
    context(
        "quoted_param",
        separated_pair(key, tag("=\""), take_till(quote)),
    )
    .parse(input)
}

fn param(input: &str) -> NomResult<&str, (&str, &str)> {
    use nom::branch::alt;
    use nom::sequence::delimited;

    context(
        "param",
        alt((
            delimited(char(';'), quoted_param, char('"')),
            preceded(char(';'), separated_pair(key, char('='), attr)),
        )),
    )
    .parse(input)
}

fn params(input: &str) -> NomResult<&str, BTreeMap<String, String>> {
    context(
        "params",
        fold_many0(param, BTreeMap::new, |mut acc, (key, value)| {
            acc.insert(key.to_string(), value.to_string());
            acc
        }),
    )
    .parse(input)
}

/**
 * See [3.1. Content Lines](https://datatracker.ietf.org/doc/html/rfc5545#section-3.1)
 */
pub(crate) fn content_line(input: &str) -> NomResult<&str, crate::ContentLine> {
    context(
        "content_line",
        map(
            (
                not(tag("BEGIN:")),
                not(tag("END:")),
                key,
                params,
                char(':'),
                opt(value),
                line_ending,
            ),
            |(_, _, key, params, _, value, _)| crate::ContentLine {
                key: key.to_string(),
                params,
                value: value.unwrap_or_default().to_string(),
            },
        ),
    )
    .parse(input)
}

pub(crate) fn content_lines(input: &str) -> NomResult<&str, Vec<crate::ContentLine>> {
    context(
        "content_lines",
        fold_many0(content_line, Vec::new, |mut acc, value| {
            acc.push(value);
            acc
        }),
    )
    .parse(input)
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
        }),
    )
    .parse(input)
}

pub(crate) fn weekdaynum(input: &str) -> NomResult<&str, crate::WeekdayNum> {
    context(
        "weekdaynum",
        map(
            (opt(nom::character::complete::i8), weekday),
            |(ord, weekday)| crate::WeekdayNum { weekday, ord },
        ),
    )
    .parse(input)
}
