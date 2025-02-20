/*!
 * See [Descriptive Component Properties](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1)
 */

/**
 * See [3.8.1.1. Attachment](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.1)
 */
pub(crate) fn attach(input: crate::ContentLine) -> crate::Result<crate::Text> {
    Ok(input.into())
}

/**
 * See [3.8.1.2. Categories](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.2)
 */
pub(crate) fn categories(input: crate::ContentLine) -> crate::Result<Vec<crate::Text>> {
    Ok(input.value.split(',').map(crate::Text::from).collect())
}

/**
 * See [3.8.1.3. Classification](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.3)
 */
pub(crate) fn class(input: crate::ContentLine) -> crate::Result<crate::Class> {
    input.value.parse()
}

/**
 * See [3.8.1.4. Comment](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.4)
 */
pub(crate) fn comment(input: crate::ContentLine) -> crate::Result<crate::Text> {
    Ok(input.into())
}

/**
 * See [3.8.1.5. Description](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.5)
 */
pub(crate) fn description(input: crate::ContentLine) -> crate::Result<crate::Text> {
    super::datatype::text(input)
}

/**
 * See [3.8.1.6. Geographic Position](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.6)
 */
pub(crate) fn geo(input: crate::ContentLine) -> crate::Result<crate::Geo> {
    use nom::Parser as _;
    use nom::character::complete::char;
    use nom::combinator::map;
    use nom::number::complete::float;
    use nom::sequence::separated_pair;

    map(separated_pair(float, char(';'), float), |(lat, lon)| {
        crate::Geo { lat, lon }
    })
    .parse(input.value.as_str())
    .map_err(crate::Error::from)
    .map(|(_, x)| x)
}

/**
 * See [3.8.1.7. Location](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.7)
 */
pub(crate) fn location(input: crate::ContentLine) -> crate::Result<crate::Text> {
    Ok(input.into())
}

/**
 * See [3.8.1.8. Percent Complete](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.8)
 */
pub(crate) fn percent_complete(input: crate::ContentLine) -> crate::Result<u8> {
    input.value.parse().map_err(crate::Error::from)
}

/**
 * See [3.8.1.9. Priority](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.9)
 */
pub(crate) fn priority(input: crate::ContentLine) -> crate::Result<u8> {
    let priority = input.value.parse()?;

    if priority > 9 {
        Err(crate::Error::Priority(priority))
    } else {
        Ok(priority)
    }
}

/**
 * See [3.8.1.10. Resources](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.10)
 */
pub(crate) fn resources(input: crate::ContentLine) -> crate::Result<Vec<crate::Text>> {
    Ok(input.value.split(',').map(crate::Text::from).collect())
}

/**
 * See [3.8.1.11. Status](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.11)
 */
pub(crate) fn status(input: crate::ContentLine) -> crate::Result<crate::Status> {
    input.value.parse()
}

/**
 * See [3.8.1.12. Summary](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.12)
 */
pub(crate) fn summary(input: crate::ContentLine) -> crate::Result<crate::Text> {
    Ok(input.into())
}
