/**
 * See [3.8.7. Change Management Component Properties](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.7)
 */

/**
 * See [3.8.7.1. Date-Time Created](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.7.1)
 */
pub(crate) fn created(input: crate::ContentLine) -> crate::Result<crate::DateTime> {
    super::datatype::date_time(&input.value)
        .map_err(crate::Error::from)
        .map(|(_, x)| x)
}

/**
 * See [3.8.7.2. Date-Time Stamp](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.7.2)
 */
pub(crate) fn dtstamp(input: crate::ContentLine) -> crate::Result<crate::DateTime> {
    super::datatype::date_time(&input.value)
        .map_err(crate::Error::from)
        .map(|(_, x)| x)
}

/**
 * See [3.8.7.3. Last Modified](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.7.3)
 */
pub(crate) fn last_modified(input: crate::ContentLine) -> crate::Result<crate::DateTime> {
    super::datatype::date_time(&input.value)
        .map_err(crate::Error::from)
        .map(|(_, x)| x)
}

/**
 * See [3.8.7.4. Sequence Number](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.7.4)
 */
pub(crate) fn sequence(input: crate::ContentLine) -> crate::Result<u32> {
    input.value.parse().map_err(crate::Error::from)
}
