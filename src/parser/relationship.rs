/**
 * See [3.8.4. Relationship Component Properties](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.4)
 */

/**
 * See [3.8.4.1. Attendee](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.4.1)
 */
pub(crate) fn attendee(input: &str) -> crate::Result<String> {
    super::datatype::cal_address(input)
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
    super::datatype::cal_address(input)
}

/**
 * See [3.8.4.4. Recurrence ID](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.4.4)
 */
pub(crate) fn recurid(input: &str) -> crate::Result<crate::Date> {
    super::datatype::date_or_dt(input)
        .map_err(crate::Error::from)
        .map(|(_, x)| x)
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
