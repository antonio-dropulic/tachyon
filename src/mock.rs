#![cfg(feature = "time_travel")]

use chrono::Duration;

use std::{sync::OnceLock, sync::RwLock};

use crate::{DateTime, Utc};

static MOCK_SYSTEM_TIME: OnceLock<RwLock<DateTime<Utc>>> = OnceLock::new();

pub mod tachyon {
    use super::*;

    /// Get mocked time
    pub(crate) fn current_time() -> DateTime<Utc> {
        let now = chrono::Utc::now();

        let time = MOCK_SYSTEM_TIME
            .get_or_init(|| RwLock::new(DateTime::from_chrono(now)))
            .read()
            .unwrap();

        *time
    }

    /// Advance mocked time by the duration given. New time is returned.
    pub fn advance(time_step: Duration) -> DateTime<Utc> {
        let now = chrono::Utc::now();
        let mut time = MOCK_SYSTEM_TIME
            .get_or_init(|| RwLock::new(DateTime::from_chrono(now)))
            .write()
            .unwrap();

        *time = *time + time_step;

        *time
    }

    /// Set the mock time
    pub fn set_time(date_time: impl Into<DateTime<Utc>>) {
        let date_time = date_time.into();

        let mut time = MOCK_SYSTEM_TIME
            .get_or_init(|| RwLock::new(date_time))
            .write()
            .unwrap();

        *time = date_time;
    }
}

#[cfg(test)]
mod tests {
    use std::thread::sleep;

    use super::*;

    #[test]
    fn stop_time() {
        let time_step = Duration::minutes(1);
        let now = Utc::now();
        let calculate_advanced = now + time_step;
        let tachyon_advanced = tachyon::advance(time_step);
        // guarantee some time has passed between calculating advanced time
        // and advancing the time with tachyon
        sleep(std::time::Duration::from_millis(10));

        let now = Utc::now();
        assert_eq!(tachyon_advanced, now, "arrow of time must be stopped");
        assert_eq!(now, calculate_advanced, "we travel exactly where we want")
    }

    #[test]
    fn travel_to_exact_time() {
        let fmt = "%Y-%m-%d-%H:%M";
        let time_str = "1991-05-16-19:50";

        let time: DateTime<Utc> = chrono::NaiveDateTime::parse_from_str(time_str, fmt)
            .unwrap()
            .and_local_timezone(Utc)
            .unwrap()
            .into();

        tachyon::set_time(time);

        // make sure time is stopped, not only traveled back
        sleep(std::time::Duration::from_millis(10));

        let now = Utc::now();
        dbg!(now);

        assert_eq!(now, time)
    }
}
