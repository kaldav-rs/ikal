/*!
 * See [3.7. Calendar Properties](https://datatracker.ietf.org/doc/html/rfc5545#section-3.7)
 */

/**
 * See [3.7.1. Calendar Scale](https://datatracker.ietf.org/doc/html/rfc5545#section-3.7.1)
 */
pub(crate) fn calscale(input: crate::ContentLine) -> crate::Result<crate::Text> {
    Ok(input.into())
}

/**
 * See [3.7.2. Method](https://datatracker.ietf.org/doc/html/rfc5545#section-3.7.2)
 */
pub(crate) fn method(input: crate::ContentLine) -> crate::Result<crate::Text> {
    Ok(input.into())
}

/**
 * See [3.7.3. Product Identifier](https://datatracker.ietf.org/doc/html/rfc5545#section-3.7.3)
 */
pub(crate) fn prodid(input: crate::ContentLine) -> crate::Result<crate::Text> {
    Ok(input.into())
}

/**
 * See [3.7.4. Version](https://datatracker.ietf.org/doc/html/rfc5545#section-3.7.4)
 */
pub(crate) fn version(input: crate::ContentLine) -> crate::Result<crate::Text> {
    Ok(input.into())
}
