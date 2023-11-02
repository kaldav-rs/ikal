/**
 * See [.8.2. Date and Time Component Properties](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2)
 */

/**
 * See [3.8.2.1. Date-Time Completed](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.1)
 */
pub(crate) fn completed(input: &str) -> crate::Result<crate::DateTime> {
    super::date(input)
}

/**
 * See [3.8.2.2. Date-Time End](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.2)
 */
pub(crate) fn dtend(input: &str) -> crate::Result<crate::DateTime> {
    super::date(input)
}

/**
 * See [3.8.2.3. Date-Time Due](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.3)
 */
pub(crate) fn due(input: &str) -> crate::Result<crate::DateTime> {
    super::date(input)
}

/**
 * See [3.8.2.4. Date-Time Start](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.4)
 */
pub(crate) fn dtstart(input: &str) -> crate::Result<crate::DateTime> {
    super::date(input)
}

/**
 * See [3.8.2.6. Free/Busy Time](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.6)
 */
pub(crate) fn freebusy(input: &str) -> crate::Result<String> {
    // @TODO
    Ok(input.to_string())
}

/**
 * See [3.8.2.7. Time Transparency](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.7)
 */
pub(crate) fn transp(input: &str) -> crate::Result<crate::TimeTransparency> {
    input.parse()
}
