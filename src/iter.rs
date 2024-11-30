pub struct Recur {
    event: crate::VEvent,
}

impl Recur {
    pub(crate) fn from(event: &crate::VEvent) -> Self {
        Self {
            event: event.clone(),
        }
    }

    pub fn between<D: Into<crate::Date> + Copy>(
        self,
        start: D,
        end: D,
    ) -> impl Iterator<Item = crate::VEvent> {
        self.skip_while(move |x| x.dtstart < start.into())
            .take_while(move |x| x.dtstart < end.into())
    }

    pub fn at<D: Into<crate::Date> + Copy>(self, date: D) -> impl Iterator<Item = crate::VEvent> {
        let delta = chrono::TimeDelta::days(1);
        self.between(date.into(), date.into() + delta)
    }

    pub fn after<D: Into<crate::Date> + Copy>(
        self,
        date: D,
    ) -> impl Iterator<Item = crate::VEvent> {
        self.skip_while(move |x| x.dtstart < date.into())
    }
}

impl Iterator for Recur {
    type Item = crate::VEvent;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.event.clone();

        let mut next = self.event.clone();

        let rrule = next.rrule.as_mut()?;

        if let Some(until) = &rrule.until {
            if self.event.dtstart.date_naive() > until.date_naive() {
                return None;
            }
        }

        if let Some(count) = &mut rrule.count {
            *count = count.checked_sub(1)?;
        }

        next.dtstart = rrule.clone() + current.dtstart;
        if let Some(dtend) = next.dtend.as_mut() {
            *dtend = rrule.clone() + *dtend;
        }

        self.event = next;

        Some(current)
    }
}

#[cfg(test)]
mod test {
    use crate as ikal;

    #[test]
    fn at() {
        let now = chrono::Local::now().date_naive().into();

        let event = crate::vevent! {
            dtstart: "20240101",
            dtend: "20240101",
            rrule: {
                freq: Daily,
                interval: 1,
            }
        }
        .unwrap();

        let next = event.recurrent().at(now)
            .next()
            .unwrap();

        assert_eq!(next.dtstart, now);
        assert_eq!(next.dtend, Some(now));
    }

    #[test]
    fn count() {
        let event = crate::vevent! {
            rrule: {
                freq: Weekly,
                interval: 1,
                count: 10,
            }
        }
        .unwrap();

        let events = event.recurrent();

        assert_eq!(events.count(), 10);
    }

    #[test]
    fn after() {
        let now: crate::Date = chrono::Local::now().into();

        let event = crate::vevent! {
            rrule: {
                freq: Monthly,
                interval: 1,
                until: now,
            }
        }
        .unwrap();

        let mut events = event.recurrent().after(now);

        assert_eq!(events.next(), None);
    }

    #[test]
    fn between() {
        let now = chrono::Local::now();

        let event = crate::vevent! {
            rrule: {
                freq: Yearly,
                interval: 10,
            }
        }
        .unwrap();

        use chrono::Datelike as _;
        let end = now.with_year(now.year() + 20).unwrap();
        let events = event.recurrent().between(now, end);

        assert_eq!(events.count(), 2);
    }
}
