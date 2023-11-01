use std::collections::BTreeMap;

/**
 * See [3.6.2. To-Do Component](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6.2)
 */
#[derive(Clone, Default, Debug, PartialEq)]
pub struct VTodo {
    pub dtstamp: crate::DateTime,
    pub uid: String,
    pub class: Option<crate::Class>,
    pub completed: Option<crate::DateTime>,
    pub created: Option<crate::DateTime>,
    pub dtstart: Option<crate::DateTime>,
    pub geo: Option<crate::Geo>,
    pub last_modified: Option<crate::DateTime>,
    pub location: Option<String>,
    pub organizer: Option<String>,
    pub percent_complete: Option<u8>,
    pub priority: Option<u8>,
    pub recurid: Option<String>,
    pub seq: Option<u32>,
    pub status: Option<crate::Status>,
    pub summary: Option<String>,
    pub url: Option<String>,
    pub rrule: Option<crate::Recur>,
    pub due: Option<crate::DateTime>,
    pub duration: Option<chrono::Duration>,
    pub attach: Vec<String>,
    pub attendee: Vec<String>,
    pub categories: Vec<String>,
    pub comment: Vec<String>,
    pub contact: Vec<String>,
    pub exdate: Vec<crate::DateTime>,
    pub rstatus: Vec<String>,
    pub related: Vec<String>,
    pub resources: Vec<String>,
    pub rdate: Vec<String>,
    pub x_prop: BTreeMap<String, String>,
    pub iana_prop: BTreeMap<String, String>,
}

impl VTodo {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl TryFrom<BTreeMap<String, String>> for VTodo {
    type Error = crate::Error;

    fn try_from(properties: BTreeMap<String, String>) -> crate::Result<Self> {
        let mut vtodo = Self::new();

        for (key, value) in properties {
            match key.as_str() {
                "DTSTAMP" => vtodo.dtstamp = crate::parser::date(value)?,
                "UID" => vtodo.uid = value,
                "CLASS" => vtodo.class = Some(value.into()),
                "COMPLETED" => vtodo.completed = Some(crate::parser::date(value)?),
                "CREATED" => vtodo.created = Some(crate::parser::date(value)?),
                "DTSTART" => vtodo.dtstart = Some(crate::parser::date(value)?),
                "GEO" => vtodo.geo = Some(value.try_into()?),
                "LAST-MODIFIED" => vtodo.last_modified = Some(crate::parser::date(value)?),
                "LOCATION" => vtodo.location = Some(value),
                "ORGANIZER" => vtodo.organizer = Some(crate::parser::organizer(&value)?),
                "PERCENT-COMPLETE" => vtodo.percent_complete = Some(value.parse()?),
                "PRIORITY" => vtodo.priority = Some(crate::parser::priority(&value)?),
                "RECURID" => vtodo.recurid = Some(crate::parser::recurid(&value)?),
                "SEQ" => vtodo.seq = Some(crate::parser::sequence(&value)?),
                "STATUS" => vtodo.status = Some(value.try_into()?),
                "SUMMARY" => vtodo.summary = Some(value),
                "URL" => vtodo.url = Some(value),
                "RRULE" => vtodo.rrule = Some(value.try_into()?),
                "DUE" => vtodo.due = Some(crate::parser::date(value)?),
                "DURATION" => {
                    vtodo.duration = Some(crate::parser::duration(&value)?)
                }
                "ATTACH" => vtodo.attach.push(crate::parser::attach(&value)),
                "ATTENDEE" => vtodo.attendee.push(crate::parser::attendee(&value)),
                "CATEGORIES" => vtodo
                    .categories
                    .append(&mut crate::parser::categories(&value)),
                "COMMENT" => vtodo.comment.push(crate::parser::comment(&value)),
                "CONTACT" => vtodo.contact.push(crate::parser::contact(&value)),
                "EXDATE" => vtodo
                    .exdate
                    .append(&mut crate::parser::exdate(&value)?),
                "RSTATUS" => vtodo.rstatus.push(crate::parser::rstatus(&value)?),
                "RELATED-TO" => vtodo.related.push(crate::parser::related(&value)),
                "RESOURCES" => vtodo
                    .resources
                    .append(&mut crate::parser::resources(&value)),
                "RDATE" => vtodo.rdate.append(&mut crate::parser::rdate(&value)?),
                _ => {
                    if key.starts_with("X-") {
                        vtodo.x_prop.insert(key, value);
                    } else {
                        vtodo.iana_prop.insert(key, value);
                    }
                }
            };
        }

        Ok(vtodo)
    }
}

impl TryFrom<String> for VTodo {
    type Error = crate::Error;

    fn try_from(raw: String) -> Result<Self, Self::Error> {
        crate::parser::vtodo(&raw)
            .map_err(crate::Error::from)
            .map(|(_, x)| x)
    }
}
