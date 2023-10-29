use std::collections::BTreeMap;

/**
 * See [3.6.1. Event Component](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6.1)
 */
#[derive(Clone, Debug, PartialEq)]
pub struct VEvent {
    pub dtstamp: crate::DateTime,
    pub uid: String,
    pub dt_start: crate::DateTime,
    pub dt_end: crate::DateTime,
    pub class: Option<crate::Class>,
    pub created: Option<crate::DateTime>,
    pub description: Option<String>,
    pub geo: Option<crate::Geo>,
    pub last_modified: Option<crate::DateTime>,
    pub location: Option<String>,
    pub organizer: Option<String>,
    pub priority: Option<u8>,
    pub seq: Option<u32>,
    pub status: Option<crate::Status>,
    pub summary: Option<String>,
    pub transp: Option<crate::TimeTransparency>,
    pub url: Option<String>,
    pub recurid: Option<String>,
    pub rrule: Option<crate::Recur>,
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

impl Default for VEvent {
    fn default() -> Self {
        Self::new()
    }
}

impl VEvent {
    pub fn new() -> Self {
        Self {
            dtstamp: chrono::Local::now(),
            uid: String::new(),
            dt_start: chrono::Local::now(),
            dt_end: chrono::Local::now(),
            class: None,
            created: None,
            description: None,
            geo: None,
            last_modified: None,
            location: None,
            organizer: None,
            priority: None,
            seq: None,
            status: None,
            summary: None,
            transp: None,
            url: None,
            recurid: None,
            rrule: None,
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

impl TryFrom<BTreeMap<String, String>> for VEvent {
    type Error = crate::Error;

    fn try_from(properties: BTreeMap<String, String>) -> crate::Result<Self> {
        let mut vevent = VEvent::new();

        for (key, value) in properties {
            match key.as_str() {
                "DTSTAMP" => vevent.dtstamp = crate::parser::parse_date(value)?,
                "UID" => vevent.uid = value,
                "DTSTART" => vevent.dt_start = crate::parser::parse_date(value)?,
                "DTEND" => vevent.dt_end = crate::parser::parse_date(value)?,
                "DURATION" => {
                    vevent.dt_end = vevent.dt_start + crate::parser::parse_duration(value)?
                }
                "CLASS" => vevent.class = Some(value.into()),
                "CREATED" => vevent.created = Some(crate::parser::parse_date(value)?),
                "DESCRIPTION" => vevent.description = Some(value),
                "GEO" => vevent.geo = Some(crate::parser::parse_geo(value)?),
                "LAST-MODIFIED" => vevent.last_modified = Some(crate::parser::parse_date(value)?),
                "LOCATION" => vevent.location = Some(value),
                "ORGANIZER" => vevent.organizer = Some(crate::parser::parse_organizer(value)?),
                "PRIORITY" => vevent.priority = Some(crate::parser::parse_priority(value)?),
                "SEQ" => vevent.seq = Some(crate::parser::parse_sequence(value)?),
                "STATUS" => vevent.status = Some(value.try_into()?),
                "SUMMARY" => vevent.summary = Some(value),
                "STRANSP" => vevent.transp = Some(value.try_into()?),
                "URL" => vevent.url = Some(value),
                "RECURID" => vevent.recurid = Some(crate::parser::parse_recurid(value)?),
                "RRULE" => vevent.rrule = Some(crate::parser::parse_rrule(value)?),
                "ATTACH" => vevent.attach.push(crate::parser::parse_attach(value)),
                "ATTENDEE" => vevent.attendee.push(crate::parser::parse_attendee(value)),
                "CATEGORIES" => vevent
                    .categories
                    .append(&mut crate::parser::parse_categories(value)),
                "COMMENT" => vevent.comment.push(crate::parser::parse_comment(value)),
                "CONTACT" => vevent.contact.push(crate::parser::parse_contact(value)),
                "EXDATE" => vevent
                    .exdate
                    .append(&mut crate::parser::parse_exdate(value)?),
                "RSTATUS" => vevent.rstatus.push(crate::parser::parse_rstatus(value)?),
                "RELATED-TO" => vevent.related.push(crate::parser::parse_related(value)),
                "RESOURCES" => vevent
                    .resources
                    .append(&mut crate::parser::parse_resources(value)),
                "RDATE" => vevent.rdate.append(&mut crate::parser::parse_rdate(value)?),
                _ => {
                    if key.starts_with("X-") {
                        vevent.x_prop.insert(key, value);
                    } else {
                        vevent.iana_prop.insert(key, value);
                    }
                }
            };
        }

        Ok(vevent)
    }
}
