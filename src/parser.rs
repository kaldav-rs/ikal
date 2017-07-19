use ::nom::line_ending;
use ::std::convert::TryInto;

fn is_alphabetic(chr: char) -> bool {
    (chr as u8 >= 0x41 && chr as u8 <= 0x5A)
        || (chr as u8 >= 0x61 && chr as u8 <= 0x7A)
}

fn is_digit(chr: char) -> bool {
    chr as u8 >= 0x30 && chr as u8 <= 0x39
}

fn is_sep(chr: char) -> bool {
    chr == '-' || chr == '/'
}

fn is_alphanumeric(chr: char) -> bool {
    is_alphabetic(chr) || is_digit(chr) || is_sep(chr)
}

fn is_line_ending(chr: char) -> bool {
    chr == '\n' || chr == '\r'
}

named!(key<&str, &str>, take_while_s!(is_alphanumeric));
named!(attr<&str, &str>, take_while_s!(is_alphanumeric));
named!(value_line<&str, &str>, take_till_s!(is_line_ending));

named!(value_part<&str, (String)>,
    do_parse!(
        value_part:
            value_line >>
            line_ending >>
            tag!(" ") >>

        (value_part.into())
    )
);

named!(value<&str, (String)>,
    do_parse!(
        value:
            many0!(value_part) >>
        value_end:
            value_line >>

        (value.join("") + value_end)
    )
);

named!(param<&str, (String, String)>,
    do_parse!(
        char!(';') >>
        key:
            key >>
            char!('=') >>
        attr:
            attr >>

        (key.into(), attr.into())
    )
);

named!(pub property<&str, (String, String)>,
    do_parse!(
            not!(tag_s!("BEGIN")) >>
            not!(tag_s!("END")) >>
        key:
            key >>
            many0!(param) >>
            char!(':') >>
        value:
            opt!(value) >>
            line_ending >>

        (key.into(), if let Some(value) = value {
            value
        } else {
            String::new()
        })
    )
);

named!(pub properties<&str, ::std::collections::BTreeMap<String, String>>,
    do_parse!(
        values: many0!(property) >>

        ({
            let mut hash = ::std::collections::BTreeMap::new();

            for (key, value) in values {
                hash.insert(key, value);
            }

            hash
        })
    )
);

named!(pub parse_vevent<&str, (Result<::VEvent, String>)>,
    do_parse!(
            tag_s!("BEGIN:VEVENT") >>
            line_ending >>
        values:
            properties >>
            tag_s!("END:VEVENT") >>
            line_ending >>

        (values.try_into())
    )
);

named!(pub parse_vtodo<&str, (Result<::VTodo, String>)>,
    do_parse!(
            tag_s!("BEGIN:VTODO") >>
            line_ending >>
        values:
            properties >>
            tag_s!("END:VTODO") >>
            line_ending >>

        (values.try_into())
    )
);

named!(pub parse_content<&str, (Result<::Content, String>)>,
    alt!(
        parse_vevent => { |event| match event {
            Ok(event) => Ok(::Content::Event(event)),
            Err(err) => Err(err),
        }} |
        parse_vtodo => { |todo| match todo {
            Ok(todo) => Ok(::Content::Todo(todo)),
            Err(err) => Err(err),
        }}
    )
);

named!(pub parse_vcalendar<&str, (Result<::VCalendar, String>)>,
    do_parse!(
            tag_s!("BEGIN:VCALENDAR") >>
            line_ending >>
        values:
            properties >>
        content:
            parse_content >>
            take_until_and_consume!("END:VCALENDAR") >>

        ({
            let calendar: Result<::VCalendar, String> = values.try_into();

            match calendar {
                Ok(mut calendar) => {
                    match content {
                        Ok(content) => {
                            calendar.content = content;
                            Ok(calendar)
                        },
                        Err(err) => Err(err),
                    }
                },
                Err(err) => Err(err),
            }
        })
    )
);
