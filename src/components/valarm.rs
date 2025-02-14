/**
 * See [3.6.6. Alarm Component](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6.6)
 */
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VAlarm {
    Audio(Audio),
    Display(Display),
    Email(Email),
}

impl TryFrom<Vec<crate::ContentLine>> for VAlarm {
    type Error = crate::Error;

    fn try_from(properties: Vec<crate::ContentLine>) -> crate::Result<Self> {
        let action = properties.iter().find(|x| x.key == "ACTION").unwrap();

        let component = match action.value.as_str() {
            "AUDIO" => Self::Audio(Audio::try_from(properties)?),
            "DISPLAY" => Self::Display(Display::try_from(properties)?),
            "EMAIL" => Self::Email(Email::try_from(properties)?),

            action => return Err(crate::Error::Alarm(action.to_string())),
        };

        Ok(component)
    }
}

impl TryFrom<String> for VAlarm {
    type Error = crate::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<&str> for VAlarm {
    type Error = crate::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl std::str::FromStr for VAlarm {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        crate::parser::valarm(&s.replace("\r\n ", ""))
            .map_err(crate::Error::from)
            .map(|(_, x)| x)
    }
}

impl crate::ser::Serialize for VAlarm {
    fn ical(&self) -> crate::Result<String> {
        let s = match self {
            Self::Audio(audio) => audio.ical(),
            Self::Display(display) => display.ical(),
            Self::Email(email) => email.ical(),
        }?;

        let mut lines = s.split("\n").collect::<Vec<_>>();
        lines[0] = "BEGIN:VALARM\r";
        lines.pop();
        lines.pop();
        lines.push("END:VALARM\r\n");

        Ok(lines.join("\n"))
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, crate::Component)]
pub struct Audio {
    pub action: crate::Text,
    pub trigger: crate::Trigger,
    pub duration: Option<chrono::Duration>,
    pub repeat: Option<u32>,
    pub attach: Vec<crate::Text>,
    #[component(ignore)]
    pub x_prop: std::collections::BTreeMap<String, crate::ContentLine>,
    #[component(ignore)]
    pub iana_prop: std::collections::BTreeMap<String, crate::ContentLine>,
}

impl Audio {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl From<Audio> for VAlarm {
    fn from(value: Audio) -> Self {
        let mut value = value.clone();
        value.action = "AUDIO".into();
        Self::Audio(value)
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, crate::Component)]
pub struct Display {
    pub action: crate::Text,
    pub trigger: crate::Trigger,
    pub description: crate::Text,
    pub duration: Option<chrono::Duration>,
    pub repeat: Option<u32>,
    #[component(ignore)]
    pub x_prop: std::collections::BTreeMap<String, crate::ContentLine>,
    #[component(ignore)]
    pub iana_prop: std::collections::BTreeMap<String, crate::ContentLine>,
}

impl Display {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl From<Display> for VAlarm {
    fn from(value: Display) -> Self {
        let mut value = value.clone();
        value.action = "DISPLAY".into();
        Self::Display(value)
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, crate::Component)]
pub struct Email {
    pub action: crate::Text,
    pub trigger: crate::Trigger,
    pub description: crate::Text,
    pub summary: crate::Text,
    pub attendee: Vec<crate::Uri>,
    pub duration: Option<chrono::Duration>,
    pub repeat: Option<u32>,
    pub attach: Vec<crate::Text>,
    #[component(ignore)]
    pub x_prop: std::collections::BTreeMap<String, crate::ContentLine>,
    #[component(ignore)]
    pub iana_prop: std::collections::BTreeMap<String, crate::ContentLine>,
}

impl Email {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl From<Email> for VAlarm {
    fn from(value: Email) -> Self {
        let mut value = value.clone();
        value.action = "EMAIL".into();
        Self::Email(value)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn parse() {
        crate::test::test_files::<crate::VAlarm>("alarms");
    }

    #[test]
    fn ser() -> crate::Result {
        let mut attach = crate::Text::from("ftp://example.com/pub/sounds/bell-01.aud");
        attach
            .params
            .insert("FMTTYPE".to_string(), "audio/basic".to_string());

        let valarm = crate::valarm! {
            @audio,
            trigger: "19970317T133000Z",
            duration: "P15DT",
            repeat: 4,
            attach: [attach],
        }?;

        let ical = crate::ser::ical(&valarm)?;

        similar_asserts::assert_eq!(
            ical,
            "BEGIN:VALARM\r
ACTION:AUDIO\r
TRIGGER;VALUE=DATE-TIME:19970317T133000Z\r
DURATION:PT1296000S\r
REPEAT:4\r
ATTACH;FMTTYPE=audio/basic:ftp://example.com/pub/sounds/bell-01.aud\r
END:VALARM\r
"
        );

        Ok(())
    }

    #[test]
    fn macros() -> crate::Result {
        let duration = Some(chrono::TimeDelta::zero());

        let _audio = crate::valarm! {
            @audio,
            trigger: "19970317T133000Z",
            duration,
            repeat: 4,
            attach: ["ftp://example.com/pub/sounds/bell-01.aud"],
        }?;

        let _display = crate::valarm! {
            @display,
            trigger: "-PT30M",
            description: "Breakfast meeting with executive team at 8:30 AM EST."
            duration,
            repeat: 2,
        }?;

        let _email = crate::valarm! {
            @email,
            trigger: "-P2D",
            description: "A draft agenda needs to be sent out to the attendees
to the weekly managers meeting (MGR-LIST). Attached is a
pointer the document template for the agenda file.",
            summary: "*** REMINDER: SEND AGENDA FOR WEEKLY STAFF MEETING ***",
            attendee: ["mailto:john_doe@example.com"],
            duration,
            repeat: 2,
            attach: ["http://example.com/templates/agenda.doc"],
        }?;

        Ok(())
    }
}
