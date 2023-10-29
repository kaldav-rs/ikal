use std::collections::BTreeMap;

/**
 * See [3.6.2. To-Do Component](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6.2)
 */
#[derive(Clone, Debug, PartialEq)]
pub struct VTodo {
    pub dtstamp: crate::DateTime,
    pub uid: String,
    pub class: Option<crate::Class>,
    pub completed: Option<crate::DateTime>,
    pub created: Option<crate::DateTime>,
    pub dt_start: Option<crate::DateTime>,
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

impl Default for VTodo {
    fn default() -> Self {
        Self::new()
    }
}

impl VTodo {
    pub fn new() -> Self {
        Self {
            dtstamp: chrono::Local::now(),
            uid: String::new(),
            class: None,
            completed: None,
            created: None,
            dt_start: None,
            geo: None,
            last_modified: None,
            location: None,
            organizer: None,
            percent_complete: None,
            priority: None,
            recurid: None,
            seq: None,
            status: None,
            summary: None,
            url: None,
            rrule: None,
            due: None,
            duration: None,
            attach: Vec::new(),
            attendee: Vec::new(),
            categories: Vec::new(),
            comment: Vec::new(),
            contact: Vec::new(),
            exdate: Vec::new(),
            rstatus: Vec::new(),
            related: Vec::new(),
            resources: Vec::new(),
            rdate: Vec::new(),
            x_prop: BTreeMap::new(),
            iana_prop: BTreeMap::new(),
        }
    }
}

impl TryFrom<BTreeMap<String, String>> for VTodo {
    type Error = crate::Error;

    fn try_from(
        properties: BTreeMap<String, String>,
    ) -> crate::Result<Self> {
        let mut vtodo = Self::new();

        for (key, value) in properties {
            match key.as_str() {
                "DTSTAMP" => vtodo.dtstamp = crate::parser::parse_date(value)?,
                "UID" => vtodo.uid = value,
                "CLASS" => vtodo.class = Some(value.into()),
                "COMPLETED" => vtodo.completed = Some(crate::parser::parse_date(value)?),
                "CREATED" => vtodo.created = Some(crate::parser::parse_date(value)?),
                "DTSTART" => vtodo.dt_start = Some(crate::parser::parse_date(value)?),
                "GEO" => vtodo.geo = Some(crate::parser::parse_geo(value)?),
                "LAST-MODIFIED" => vtodo.last_modified = Some(crate::parser::parse_date(value)?),
                "LOCATION" => vtodo.location = Some(value),
                "ORGANIZER" => vtodo.organizer = Some(crate::parser::parse_organizer(value)?),
                "PERCENT-COMPLETE" => vtodo.percent_complete = Some(value.parse()?),
                "PRIORITY" => vtodo.priority = Some(crate::parser::parse_priority(value)?),
                "RECURID" => vtodo.recurid = Some(crate::parser::parse_recurid(value)?),
                "SEQ" => vtodo.seq = Some(crate::parser::parse_sequence(value)?),
                "STATUS" => vtodo.status = Some(value.try_into()?),
                "SUMMARY" => vtodo.summary = Some(value),
                "URL" => vtodo.url = Some(value),
                "RRULE" => vtodo.rrule = Some(crate::parser::parse_rrule(value)?),
                "DUE" => vtodo.due = Some(crate::parser::parse_date(value)?),
                "DURATION" => vtodo.duration = Some(crate::parser::parse_duration(value)?),
                "ATTACH" => vtodo.attach.push(crate::parser::parse_attach(value)),
                "ATTENDEE" => vtodo.attendee.push(crate::parser::parse_attendee(value)),
                "CATEGORIES" => vtodo.categories.append(&mut crate::parser::parse_categories(value)),
                "COMMENT" => vtodo.comment.push(crate::parser::parse_comment(value)),
                "CONTACT" => vtodo.contact.push(crate::parser::parse_contact(value)),
                "EXDATE" => vtodo.exdate.append(&mut crate::parser::parse_exdate(value)?),
                "RSTATUS" => vtodo.rstatus.push(crate::parser::parse_rstatus(value)?),
                "RELATED-TO" => vtodo.related.push(crate::parser::parse_related(value)),
                "RESOURCES" => vtodo.resources.append(&mut crate::parser::parse_resources(value)),
                "RDATE" => vtodo.rdate.append(&mut crate::parser::parse_rdate(value)?),
                _ => if key.starts_with("X-") {
                    vtodo.x_prop.insert(key, value);
                } else {
                    vtodo.iana_prop.insert(key, value);
                }
            };
        }

        Ok(vtodo)
    }
}
