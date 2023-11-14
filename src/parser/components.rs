use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, map_res};
use nom::multi::many0;
use nom::sequence::{delimited, tuple};

macro_rules! component {
    ($name:ident, $ty:ty) => {
        pub(crate) fn $name(input: &str) -> nom::IResult<&str, $ty> {
            let c = stringify!($name).to_uppercase();

            map_res(
                delimited(
                    tag(format!("BEGIN:{c}\r\n").as_str()),
                    super::content_lines,
                    tag(format!("END:{c}\r\n").as_str()),
                ),
                |values| values.try_into(),
            )(input)
        }
    };
}

component!(valarm, crate::VAlarm);
component!(vevent, crate::VEvent);
component!(vfreebusy, crate::VFreebusy);
component!(vtodo, crate::VTodo);
component!(vjournal, crate::VJournal);
component!(standard, crate::vtimezone::Prop);
component!(daylight, crate::vtimezone::Prop);

pub(crate) fn prop(_: &str) -> nom::IResult<&str, crate::vtimezone::Prop> {
    unreachable!()
}

pub(crate) fn audio(_: &str) -> nom::IResult<&str, crate::valarm::Audio> {
    unreachable!()
}

pub(crate) fn display(_: &str) -> nom::IResult<&str, crate::valarm::Display> {
    unreachable!()
}

pub(crate) fn email(_: &str) -> nom::IResult<&str, crate::valarm::Email> {
    unreachable!()
}

pub(crate) fn vtimezone(input: &str) -> nom::IResult<&str, crate::VTimezone> {
    map_res(
        delimited(
            tag("BEGIN:VTIMEZONE\r\n"),
            tuple((
                super::content_lines,
                many0(alt((
                    map(standard, crate::vtimezone::Component::Standard),
                    map(daylight, crate::vtimezone::Component::Daylight),
                ))),
            )),
            tag("END:VTIMEZONE\r\n"),
        ),
        |(values, components)| {
            let mut vtimezone: crate::VTimezone = values.try_into()?;

            for component in components {
                match component {
                    crate::vtimezone::Component::Standard(standard) => {
                        vtimezone.standard.push(standard);
                    }
                    crate::vtimezone::Component::Daylight(daylight) => {
                        vtimezone.daylight.push(daylight);
                    }
                }
            }

            Ok::<_, crate::Error>(vtimezone)
        },
    )(input)
}

pub(crate) fn component(input: &str) -> nom::IResult<&str, crate::Component> {
    alt((
        map(valarm, crate::Component::Alarm),
        map(vevent, crate::Component::Event),
        map(vfreebusy, crate::Component::Freebusy),
        map(vjournal, crate::Component::Journal),
        map(vtimezone, crate::Component::Timezone),
        map(vtodo, crate::Component::Todo),
    ))(input)
}

pub(crate) fn components(input: &str) -> nom::IResult<&str, Vec<crate::Component>> {
    many0(component)(input)
}

pub(crate) fn vcalendar(input: &str) -> nom::IResult<&str, crate::VCalendar> {
    map_res(
        delimited(
            tag("BEGIN:VCALENDAR\r\n"),
            tuple((super::content_lines, components)),
            tag("END:VCALENDAR"),
        ),
        |(content_lines, components)| {
            let mut vcalendar: crate::VCalendar = content_lines.try_into()?;

            for component in components {
                match component {
                    crate::Component::Alarm(alarm) => vcalendar.alarms.push(alarm),
                    crate::Component::Event(event) => vcalendar.events.push(event),
                    crate::Component::Freebusy(freebusy) => vcalendar.freebusy.push(freebusy),
                    crate::Component::Journal(journal) => vcalendar.journals.push(journal),
                    crate::Component::Todo(todo) => vcalendar.todo.push(todo),
                    crate::Component::Timezone(timezone) => vcalendar.timezones.push(timezone),
                }
            }

            Ok::<_, crate::Error>(vcalendar)
        },
    )(input)
}
