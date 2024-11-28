/*!
 * See [3.8.4. Relationship Component Properties](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.4)
 */

/**
 * See [3.8.4.1. Attendee](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.4.1)
 */
pub(crate) fn attendee(input: crate::ContentLine) -> crate::Result<crate::Uri> {
    Ok(crate::Uri {
        params: input.params,
        uri: super::datatype::cal_address(&input.value)?,
    })
}

/**
 * See [3.8.4.2. Contact](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.4.2)
 */
pub(crate) fn contact(input: crate::ContentLine) -> crate::Result<crate::Text> {
    Ok(input.into())
}

/**
 * See [3.8.4.3. Organizer](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.4.3)
 */
pub(crate) fn organizer(input: crate::ContentLine) -> crate::Result<crate::Uri> {
    Ok(crate::Uri {
        params: input.params,
        uri: super::datatype::cal_address(&input.value)?,
    })
}

/**
 * See [3.8.4.4. Recurrence ID](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.4.4)
 */
pub(crate) fn recurid(input: crate::ContentLine) -> crate::Result<crate::Date> {
    super::datatype::date_or_dt(&input.value)
        .map_err(crate::Error::from)
        .map(|(_, x)| x)
}

/**
 * See [3.8.4.5. Related To](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.4.5)
 */
pub(crate) fn related_to(input: crate::ContentLine) -> crate::Result<crate::Text> {
    Ok(input.into())
}

/**
 * See [3.8.4.6. Uniform Resource Locator](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.4.6)
 */
pub(crate) fn url(input: crate::ContentLine) -> crate::Result<crate::Uri> {
    Ok(input.into())
}

/**
 * See [3.8.4.7. Unique Identifier](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.4.7)
 */
pub(crate) fn uid(input: crate::ContentLine) -> crate::Result<crate::Text> {
    Ok(input.into())
}
