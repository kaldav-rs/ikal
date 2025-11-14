#[macro_export]
#[doc(hidden)]
macro_rules! ical_for_tostring {
    ($ty: ty) => {
        impl $crate::ser::Serialize for $ty {
            fn ical(&self) -> String {
                self.to_string()
            }
        }
    };
}

pub(crate) use ical_for_tostring;

pub trait Serialize {
    fn component() -> Option<String> {
        None
    }

    fn attr(&self) -> Option<String> {
        None
    }

    fn ical(&self) -> String;
}

pub fn ical<T: Serialize>(value: &T) -> String {
    let mut s = String::new();

    if let Some(attr) = value.attr() {
        s.push_str(&attr);
        s.push(':');
    }

    s.push_str(&value.ical());

    s
}

ical_for_tostring!(i8);
ical_for_tostring!(u8);
ical_for_tostring!(u32);
ical_for_tostring!(chrono::TimeDelta);

impl Serialize for chrono::FixedOffset {
    fn ical(&self) -> String {
        let offset = self.local_minus_utc();
        let (sign, offset) = if offset < 0 {
            ('-', -offset)
        } else {
            ('+', offset)
        };
        let sec = offset.rem_euclid(60);
        let mins = offset.div_euclid(60);
        let min = mins.rem_euclid(60);
        let hour = mins.div_euclid(60);

        if sec == 0 {
            format!("{sign}{hour:02}{min:02}")
        } else {
            format!("{sign}{hour:02}{min:02}{sec:02}")
        }
    }
}

impl Serialize for String {
    fn ical(&self) -> String {
        escape(self)
    }
}

impl<T: Serialize> Serialize for Vec<T> {
    fn ical(&self) -> String {
        let mut s = String::new();

        for x in self {
            s.push_str(&x.ical());
            s.push(',');
        }
        s.pop();

        s
    }
}

impl<T: Serialize> Serialize for Option<T> {
    fn attr(&self) -> Option<String> {
        if let Some(value) = &self {
            value.attr()
        } else {
            None
        }
    }

    fn ical(&self) -> String {
        if let Some(value) = &self {
            value.ical()
        } else {
            String::new()
        }
    }
}

impl<K: ToString, V: Serialize> Serialize for std::collections::BTreeMap<K, V> {
    fn ical(&self) -> String {
        let mut s = String::new();

        for (k, v) in self {
            s.push_str(&k.to_string());
            s.push('=');
            s.push_str(&v.ical());
            s.push(';');
        }
        s.pop();

        s
    }
}

pub(crate) fn escape(s: &str) -> String {
    s.replace(';', "\\;")
        .replace(',', "\\,")
        .replace('\n', "\\n")
}

pub(crate) fn field<S: Serialize>(name: &str, value: &S) -> String {
    let mut s = String::new();

    let ical = value.ical();

    if S::component().is_some() {
        s.push_str(&ical);
    } else {
        if !ical.is_empty() {
            s.push_str(name);

            if let Some(attr) = value.attr() {
                s.push(';');
                s.push_str(&attr);
            }
            s.push(':');
            s.push_str(&ical);
            s.push_str("\r\n");
        }

        if s.len() > 75 {
            let lines = split(&s, 75);
            s = lines.join("\r\n ");
        }
    }

    s
}

fn split(s: &str, sub_size: usize) -> Vec<&str> {
    let mut v = Vec::with_capacity(s.len() / sub_size);
    let mut cur = s;

    while !cur.is_empty() {
        let (chunk, rest) = cur.split_at(std::cmp::min(sub_size, cur.len()));
        v.push(chunk);
        cur = rest;
    }

    v
}

#[cfg(test)]
mod test {
    #[test]
    fn long_line() {
        let text = crate::Text::from(
            "This is a long description with more than 75 characteres that exists on a long line.",
        );
        let ical = crate::ser::field("DESCRIPTION", &text);

        similar_asserts::assert_eq!(
            ical,
            "DESCRIPTION:This is a long description with more than 75 characteres that e\r
 xists on a long line.\r
"
        );
    }

    #[test]
    fn fixed_offset() {
        use crate::ser::Serialize as _;

        let offset = chrono::FixedOffset::east_opt(3600).unwrap();
        assert_eq!(offset.ical(), "+0100");
    }
}
