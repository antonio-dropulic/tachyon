#![cfg(feature = "time_travel")]
//! Used to shadow chrono::Utc, more specificaly its Utc::now constructor.
//! So when using the time travel feature we get the mocked time.

use crate::DateTime;
use chrono::{offset::LocalResult, FixedOffset, NaiveDate, NaiveDateTime, Offset, TimeZone};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Utc;

impl Utc {
    pub fn now() -> DateTime<Utc> {
        DateTime::now()
    }
}

impl TimeZone for Utc {
    type Offset = Utc;

    fn from_offset(_state: &Utc) -> Utc {
        Utc
    }

    fn offset_from_local_date(&self, _local: &NaiveDate) -> LocalResult<Utc> {
        LocalResult::Single(Utc)
    }
    fn offset_from_local_datetime(&self, _local: &NaiveDateTime) -> LocalResult<Utc> {
        LocalResult::Single(Utc)
    }

    fn offset_from_utc_date(&self, _utc: &NaiveDate) -> Utc {
        Utc
    }
    fn offset_from_utc_datetime(&self, _utc: &NaiveDateTime) -> Utc {
        Utc
    }
}

impl Offset for Utc {
    fn fix(&self) -> FixedOffset {
        FixedOffset::east_opt(0).unwrap()
    }
}
