/**
 * See [3.6.6. Alarm Component](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6.6)
 */
#[derive(Clone, Debug, PartialEq)]
pub enum VAlarm {
    Audio(Audio),
    Display(Display),
    Email(Email),
}

impl TryFrom<std::collections::BTreeMap<String, crate::ContentLine>> for VAlarm {
    type Error = crate::Error;

    fn try_from(
        properties: std::collections::BTreeMap<String, crate::ContentLine>,
    ) -> crate::Result<Self> {
        let component = match properties["ACTION"].value.as_str() {
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
        crate::parser::valarm(s)
            .map_err(crate::Error::from)
            .map(|(_, x)| x)
    }
}

#[derive(Clone, Debug, Default, PartialEq, crate::Component)]
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

#[derive(Clone, Debug, Default, PartialEq, crate::Component)]
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

#[derive(Clone, Debug, Default, PartialEq, crate::Component)]
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

#[cfg(test)]
mod test {
    #[test]
    fn parse() {
        crate::test::test_files::<crate::VAlarm>("alarms");
    }
}
