use std::collections::BTreeMap;

/**
 * See [3.6.3. Journal Component](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6.3)
 */
#[derive(Debug, Default, PartialEq)]
pub struct VJournal {
    pub dtstamp: crate::DateTime,
    pub uid: String,
    pub class: Option<crate::Class>,
    pub created: Option<crate::DateTime>,
    pub dtstart: crate::DateTime,
    pub last_modified: Option<crate::DateTime>,
    pub organizer: Option<String>,
    pub recurid: Option<String>,
    pub seq: Option<u32>,
    pub status: Option<crate::Status>,
    pub summary: Option<String>,
    pub url: Option<String>,
    pub rrule: Option<crate::Recur>,
    pub attach: Vec<String>,
    pub attendee: Vec<String>,
    pub categories: Vec<String>,
    pub comment: Vec<String>,
    pub contact: Vec<String>,
    pub description: Vec<String>,
    pub exdate: Vec<crate::DateTime>,
    pub related: Vec<String>,
    pub rdate: Vec<String>,
    pub rstatus: Vec<String>,
    pub x_prop: BTreeMap<String, String>,
    pub iana_prop: BTreeMap<String, String>,
}

impl VJournal {
    fn new() -> Self {
        Self::default()
    }
}

impl TryFrom<std::collections::BTreeMap<String, String>> for VJournal {
    type Error = crate::Error;

    fn try_from(properties: BTreeMap<String, String>) -> Result<Self, Self::Error> {
        let mut vjournal = Self::new();

        for (key, value) in properties {
            match key.as_str() {
                "DTSTAMP" => vjournal.dtstamp = crate::parser::date(value)?,
                "UID" => vjournal.uid = value,
                "CLASS" => vjournal.class = Some(value.into()),
                "CREATED" => vjournal.created = Some(crate::parser::date(value)?),
                "DTSTART" => vjournal.dtstart = crate::parser::date(value)?,
                "LAST-MODIFIED" => vjournal.last_modified = Some(crate::parser::date(value)?),
                "ORGANIZER" => vjournal.organizer = Some(crate::parser::organizer(&value)?),
                "RECURID" => vjournal.recurid = Some(crate::parser::recurid(&value)?),
                "SEQ" => vjournal.seq = Some(crate::parser::sequence(&value)?),
                "STATUS" => vjournal.status = Some(value.try_into()?),
                "SUMMARY" => vjournal.summary = Some(value),
                "URL" => vjournal.url = Some(value),
                "RRULE" => vjournal.rrule = Some(value.try_into()?),
                "ATTACH" => vjournal.attach.push(crate::parser::attach(&value)),
                "ATTENDEE" => vjournal.attendee.push(crate::parser::attendee(&value)),
                "CATEGORIES" => vjournal
                    .categories
                    .append(&mut crate::parser::categories(&value)),
                "COMMENT" => vjournal.comment.push(crate::parser::comment(&value)),
                "CONTACT" => vjournal.contact.push(crate::parser::contact(&value)),
                "DESCRIPTION" => vjournal.description.push(value),
                "EXDATE" => vjournal
                    .exdate
                    .append(&mut crate::parser::exdate(&value)?),
                "RELATED-TO" => vjournal.related.push(crate::parser::related(&value)),
                "RDATE" => vjournal
                    .rdate
                    .append(&mut crate::parser::rdate(&value)?),
                "RSTATUS" => vjournal.rstatus.push(crate::parser::rstatus(&value)?),
                _ => {
                    if key.starts_with("X-") {
                        vjournal.x_prop.insert(key, value);
                    } else {
                        vjournal.iana_prop.insert(key, value);
                    }
                }
            };
        }

        Ok(vjournal)
    }
}

impl TryFrom<String> for VJournal {
    type Error = crate::Error;

    fn try_from(raw: String) -> Result<Self, Self::Error> {
        crate::parser::vjournal(&raw)
            .map_err(crate::Error::from)
            .map(|(_, x)| x)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn parse() {
        crate::test::test_files::<crate::VJournal>("journals");
    }
}
