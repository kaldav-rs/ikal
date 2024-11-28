/*!
 * See [3.8.6. Alarm Component Properties](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.6)
 */

/**
 * See [3.8.6.1. Action](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.6.1)
 */
pub(crate) fn action(input: crate::ContentLine) -> crate::Result<crate::Text> {
    Ok(input.into())
}

/**
 * See [3.8.6.2. Repeat Count](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.6.2)
 */
pub(crate) fn repeat(input: crate::ContentLine) -> crate::Result<u32> {
    input.value.parse().map_err(crate::Error::from)
}

/**
 * See [3.8.6.3. Trigger](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.6.3)
 */
pub(crate) fn trigger(input: crate::ContentLine) -> crate::Result<crate::Trigger> {
    use nom::branch::alt;
    use nom::combinator::map;

    alt((
        map(super::datatype::duration, crate::Trigger::Duration),
        map(super::datatype::date_time, crate::Trigger::DateTime),
    ))(input.value.as_str())
    .map_err(crate::Error::from)
    .map(|(_, x)| x)
}
