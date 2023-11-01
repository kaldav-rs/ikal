/**
 * See [Descriptive Component Properties](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1)
 */

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
    use nom::character::complete::char;
    use nom::combinator::map;
    use nom::number::complete::float;
    use nom::sequence::separated_pair;

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
