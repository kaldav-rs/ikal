/**
 * See [.8.2. Date and Time Component Properties](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2)
 */

/**
 * See [3.8.2.1. Date-Time Completed](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.1)
 */
pub(crate) fn completed(input: &str) -> crate::Result<crate::DateTime> {
    super::datatype::date(input)
}

/**
 * See [3.8.2.2. Date-Time End](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.2)
 */
pub(crate) fn dtend(input: &str) -> crate::Result<crate::DateTime> {
    super::datatype::date(input)
}

/**
 * See [3.8.2.3. Date-Time Due](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.3)
 */
pub(crate) fn due(input: &str) -> crate::Result<crate::DateTime> {
    super::datatype::date(input)
}

/**
 * See [3.8.2.4. Date-Time Start](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.4)
 */
pub(crate) fn dtstart(input: &str) -> crate::Result<crate::DateTime> {
    super::datatype::date(input)
}

/**
 * See [3.8.2.5. Duration](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.5)
 */
pub(crate) fn duration(input: &str) -> crate::Result<chrono::Duration> {
    super::datatype::duration(input)
}

/**
 * See [3.8.2.6. Free/Busy Time](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.6)
 */
pub(crate) fn freebusy(input: &str) -> crate::Result<Vec<crate::Period>> {
    input.split(',')
        .map(super::datatype::period)
        .collect()
}

/**
 * See [3.8.2.7. Time Transparency](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.7)
 */
pub(crate) fn transp(input: &str) -> crate::Result<crate::TimeTransparency> {
    input.parse()
}
