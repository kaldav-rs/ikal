pub trait Recurring: Clone {
    fn dtstart(&self) -> Option<&crate::Date> {
        None
    }

    fn exdate(&self) -> &[crate::Date];

    fn set_dtstart(&mut self, _dtstart: crate::Date) {}

    fn dtend(&self) -> Option<&crate::Date> {
        None
    }

    fn set_dtend(&mut self, _dtend: crate::Date) {}

    fn due(&self) -> Option<&crate::Date> {
        None
    }

    fn set_due(&mut self, _dtend: crate::Date) {}

    fn rrule(&self) -> Option<&crate::Recur>;
    fn set_rrule(&mut self, rrule: crate::Recur);
}

impl Recurring for crate::VEvent {
    fn dtstart(&self) -> Option<&crate::Date> {
        Some(&self.dtstart)
    }

    fn exdate(&self) -> &[crate::Date] {
        &self.exdate
    }

    fn set_dtstart(&mut self, dtstart: crate::Date) {
        self.dtstart = dtstart;
    }

    fn dtend(&self) -> Option<&crate::Date> {
        self.dtend.as_ref()
    }

    fn set_dtend(&mut self, dtend: crate::Date) {
        self.dtend = Some(dtend);
    }

    fn rrule(&self) -> Option<&crate::Recur> {
        self.rrule.as_ref()
    }

    fn set_rrule(&mut self, rrule: crate::Recur) {
        self.rrule = Some(rrule);
    }
}

impl Recurring for crate::VJournal {
    fn dtstart(&self) -> Option<&crate::Date> {
        Some(&self.dtstart)
    }

    fn exdate(&self) -> &[crate::Date] {
        &self.exdate
    }

    fn set_dtstart(&mut self, dtstart: crate::Date) {
        self.dtstart = dtstart;
    }

    fn rrule(&self) -> Option<&crate::Recur> {
        self.rrule.as_ref()
    }

    fn set_rrule(&mut self, rrule: crate::Recur) {
        self.rrule = Some(rrule);
    }
}

impl Recurring for crate::VTodo {
    fn dtstart(&self) -> Option<&crate::Date> {
        self.dtstart.as_ref()
    }

    fn exdate(&self) -> &[crate::Date] {
        &self.exdate
    }

    fn set_dtstart(&mut self, dtstart: crate::Date) {
        self.dtstart = Some(dtstart);
    }

    fn due(&self) -> Option<&crate::Date> {
        self.due.as_ref()
    }

    fn set_due(&mut self, due: crate::Date) {
        self.due = Some(due);
    }

    fn rrule(&self) -> Option<&crate::Recur> {
        self.rrule.as_ref()
    }

    fn set_rrule(&mut self, rrule: crate::Recur) {
        self.rrule = Some(rrule);
    }
}

pub struct Recur<T: Recurring> {
    item: T,
}

impl<T: Recurring> Recur<T> {
    pub(crate) fn from(item: &T) -> Self {
        Self { item: item.clone() }
    }

    pub fn between<D: Into<crate::Date> + Copy>(self, start: D, end: D) -> impl Iterator<Item = T> {
        self.skip_while(move |x| x.dtstart().unwrap() < &start.into())
            .take_while(move |x| x.dtstart().unwrap() < &end.into())
    }

    pub fn at<D: Into<crate::Date> + Copy>(self, date: D) -> impl Iterator<Item = T> {
        let delta = chrono::TimeDelta::days(1);
        self.between(date.into(), date.into() + delta)
    }

    pub fn after<D: Into<crate::Date> + Copy>(self, date: D) -> impl Iterator<Item = T> {
        self.skip_while(move |x| x.dtstart().unwrap() < &date.into())
    }
}

impl<T: Recurring> Iterator for Recur<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.item.clone();

        let mut next = self.item.clone();

        loop {
            let dtstart = next.dtstart()?;
            let mut rrule = next.rrule()?.clone();

            if let Some(until) = &rrule.until
                && dtstart.date_naive() > until.date_naive()
            {
                return None;
            }

            let dtstart = rrule.clone() + *dtstart;
            next.set_dtstart(dtstart);

            if let Some(dtend) = next.dtend() {
                let dtend = rrule.clone() + *dtend;
                next.set_dtend(dtend);
            }

            next.set_rrule(rrule.clone());

            if next.exdate().contains(&dtstart) {
                continue;
            }

            if let Some(count) = rrule.count.as_mut() {
                *count = count.checked_sub(1)?;
                next.set_rrule(rrule.clone());
            }

            break;
        }

        self.item = next;

        Some(current)
    }
}

#[cfg(test)]
mod test {
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

        let next = event.recurrent().at(now).next().unwrap();

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

    #[test]
    fn exdate() {
        let event = crate::vevent! {
            dtstart: "20240101",
            rrule: {
                freq: Yearly,
                interval: 1,
                count: 10,
            },
            exdate: ["20250101"],
        }
        .unwrap();

        let mut events = event.recurrent();

        assert_eq!(events.nth(1).unwrap().dtstart, "20260101".parse().unwrap());
    }

    #[test]
    fn vjournal() {
        let vjournal = crate::vjournal! {
            rrule: {
                freq: Daily,
                count: 2,
            }
        }
        .unwrap();

        let iter = vjournal.recurrent();

        assert_eq!(iter.count(), 2);
    }

    #[test]
    fn vtodo() {
        let vtodo = crate::vtodo! {
            dtstart: "20240101",
            rrule: {
                freq: Daily,
                count: 2,
            }
        }
        .unwrap();

        let iter = vtodo.recurrent();

        assert_eq!(iter.count(), 2);
    }
}
