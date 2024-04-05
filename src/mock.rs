#![cfg(feature = "time_travel")]

use chrono::Duration;

use std::{sync::OnceLock, sync::RwLock};

use crate::{DateTime, Utc};

static MOCK_SYSTEM_TIME: OnceLock<RwLock<DateTime<Utc>>> = OnceLock::new();

pub mod tachyon {
    use super::*;

    pub(crate) fn current_time() -> DateTime<Utc> {
        let now = chrono::Utc::now();

        let time = MOCK_SYSTEM_TIME
            .get_or_init(|| RwLock::new(DateTime::from_chrono(now)))
            .read()
            .unwrap();

        *time
    }
    
    pub fn advance(time_step: Duration) -> DateTime<Utc> {
        let now = chrono::Utc::now();
        let mut time = MOCK_SYSTEM_TIME
            .get_or_init(|| RwLock::new(DateTime::from_chrono(now)))
            .write()
            .unwrap();

        *time = *time + time_step;

        *time
    }

    // time-travel makes date-time now hidden from the user
    // and Utc::now. But there are many ways to construct
    // those two types. should use Into<DateTime>
    pub fn set_time(date_time: DateTime<Utc>) {
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
    fn smoke() {
        let time_step = Duration::minutes(1);
        let now = Utc::now();
        let calculate_advanced = now + time_step;
        let tachyon_advanced = tachyon::advance(time_step);
        // guarantee some time has passed between calculating advanced time
        // and advancing the time with tachyon
        sleep(std::time::Duration::from_millis(10));

        let now = Utc::now();
        assert_eq!(tachyon_advanced, now);
        assert_eq!(now, calculate_advanced)
    }
}
