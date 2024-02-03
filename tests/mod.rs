#[cfg(test)]
mod tests {
    use tachyon::Utc;

    #[test]
    #[cfg(feature = "time_travel")]
    fn arrow_of_time_stopped() {
        use std::{thread::sleep, time::Duration};

        let now = Utc::now();
        sleep(Duration::from_millis(10));

        let next = Utc::now();
        assert_eq!(next, now)
    }

    #[test]
    #[cfg(not(feature = "time_travel"))]
    fn arrow_of_time_not_stopped_without_time_travel() {
        use std::{thread::sleep, time::Duration};

        let now = Utc::now();
        sleep(Duration::from_millis(100));

        let next = Utc::now();
        assert_ne!(next, now)
    }
}
