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

        self.event = next;

        Some(current)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn at() {
        let now = chrono::Local::now()
            .with_time(chrono::NaiveTime::default())
            .unwrap();

        let mut event = crate::VEvent::new();
        event.rrule = crate::Recur {
            freq: crate::Freq::Daily,
            interval: 1,

            ..Default::default()
        }
        .into();

        let mut events = event.recurrent().at(now);

        assert_eq!(events.next().unwrap().dtstart, now.naive_local().into());
    }

    #[test]
    fn count() {
        let mut event = crate::VEvent::new();
        event.rrule = crate::Recur {
            freq: crate::Freq::Weekly,
            interval: 1,
            count: Some(10),

            ..Default::default()
        }
        .into();

        let events = event.recurrent();

        assert_eq!(events.count(), 10);
    }

    #[test]
    fn after() {
        let now = chrono::Local::now().into();

        let mut event = crate::VEvent::new();
        event.rrule = crate::Recur {
            freq: crate::Freq::Monthly,
            interval: 1,
            until: Some(now),

            ..Default::default()
        }
        .into();

        let mut events = event.recurrent().after(now);

        assert_eq!(events.next(), None);
    }

    #[test]
    fn between() {
        let now = chrono::Local::now();

        let mut event = crate::VEvent::new();
        event.rrule = crate::Recur {
            freq: crate::Freq::Yearly,
            interval: 10,

            ..Default::default()
        }
        .into();

        use chrono::Datelike as _;
        let end = now.with_year(now.year() + 20).unwrap();
        let events = event.recurrent().between(now, end);

        assert_eq!(events.count(), 2);
    }
}
