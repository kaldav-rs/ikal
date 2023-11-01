use std::collections::BTreeMap;

#[derive(Debug, Default, PartialEq)]
pub struct VTimezone {
    pub tzid: String,
    pub last_modified: Option<String>,
    pub tzurl: Option<String>,
    pub standard: Vec<Prop>,
    pub daylight: Vec<Prop>,
    pub x_prop: BTreeMap<String, String>,
    pub iana_prop: BTreeMap<String, String>,
}

impl VTimezone {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl TryFrom<std::collections::BTreeMap<String, String>> for VTimezone {
    type Error = crate::Error;

    fn try_from(properties: std::collections::BTreeMap<String, String>) -> crate::Result<Self> {
        let mut vtimezone = Self::new();

        for (key, value) in properties {
            match key.as_str() {
                "TZID" => vtimezone.tzid = value,
                "LAST-MODIFIED" => vtimezone.last_modified = Some(value),
                "TZURL" => vtimezone.tzurl = Some(value),
                _ => {
                    if key.starts_with("X-") {
                        vtimezone.x_prop.insert(key, value);
                    } else {
                        vtimezone.iana_prop.insert(key, value);
                    }
                }
            }
        }

        Ok(vtimezone)
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum Component {
    Standard(Prop),
    Daylight(Prop),
}

#[derive(Debug, PartialEq)]
pub struct Prop {
    pub dtstart: crate::DateTime,
    pub tzoffsetto: chrono::offset::FixedOffset,
    pub tzoffsetfrom: chrono::offset::FixedOffset,
    pub rrule: Option<crate::Recur>,
    pub comment: Vec<String>,
    pub rdate: Vec<String>,
    pub tzname: Vec<String>,
    pub x_prop: BTreeMap<String, String>,
    pub iana_prop: BTreeMap<String, String>,
}

impl Default for Prop {
    fn default() -> Self {
        Self::new()
    }
}

impl Prop {
    #[must_use]
    fn new() -> Self {
        Self {
            dtstart: crate::DateTime::default(),
            tzoffsetto: chrono::offset::FixedOffset::east_opt(0).unwrap(),
            tzoffsetfrom: chrono::offset::FixedOffset::east_opt(0).unwrap(),
            rrule: None,
            comment: Vec::new(),
            rdate: Vec::new(),
            tzname: Vec::new(),
            x_prop: BTreeMap::new(),
            iana_prop: BTreeMap::new(),
        }
    }
}

impl TryFrom<std::collections::BTreeMap<String, String>> for Prop {
    type Error = crate::Error;

    fn try_from(properties: std::collections::BTreeMap<String, String>) -> crate::Result<Self> {
        let mut prop = Self::new();

        for (key, value) in properties {
            match key.as_str() {
                "DTSTART" => prop.dtstart = crate::parser::date(value)?,
                "TZOFFSETTO" => prop.tzoffsetto = crate::parser::tzoffset(&value)?,
                "TZOFFSETFROM" => prop.tzoffsetfrom = crate::parser::tzoffset(&value)?,
                "RRULE" => prop.rrule = Some(value.try_into()?),
                "COMMENT" => prop.comment.push(crate::parser::comment(&value)),
                "TZNAME" => prop.comment.push(value),
                "RDATE" => prop.rdate.append(&mut crate::parser::rdate(&value)?),
                _ => {
                    if key.starts_with("X-") {
                        prop.x_prop.insert(key, value);
                    } else {
                        prop.iana_prop.insert(key, value);
                    }
                }
            }
        }

        Ok(prop)
    }
}
