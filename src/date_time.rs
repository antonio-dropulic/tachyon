#![cfg(feature = "time_travel")]

use crate::{tachyon, Utc};
use chrono::{Duration, TimeZone};
use std::{
    fmt::Display,
    marker::PhantomData,
    ops::{Add, Deref, Sub},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DateTime<Tz: TimeZone> {
    pub(crate) inner: chrono::DateTime<chrono::Utc>,
    pub(crate) _tz_phanatom: PhantomData<Tz>,
}

impl DateTime<Utc> {
    /// The Unix Epoch, 1970-01-01 00:00:00 UTC.
    pub const UNIX_EPOCH: Self = Self {
        inner: chrono::DateTime::UNIX_EPOCH,
        _tz_phanatom: PhantomData,
    };

    pub fn now() -> Self {
        tachyon::current_time()
    }

    pub(crate) fn from_chrono(dt: chrono::DateTime<chrono::Utc>) -> Self {
        Self {
            inner: dt,
            _tz_phanatom: PhantomData,
        }
    }

    #[must_use]
    pub fn from_timestamp(secs: i64, nsecs: u32) -> Option<Self> {
        let inner = chrono::DateTime::<chrono::Utc>::from_timestamp(secs, nsecs);

        inner.map(|dt| Self {
            inner: dt,
            _tz_phanatom: PhantomData,
        })
    }
}

impl Display for DateTime<Utc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl Add<Duration> for DateTime<Utc> {
    type Output = DateTime<Utc>;

    fn add(self, rhs: Duration) -> Self::Output {
        Self {
            inner: self.inner + rhs,
            _tz_phanatom: PhantomData,
        }
    }
}

impl Sub<Duration> for DateTime<Utc> {
    type Output = DateTime<Utc>;

    fn sub(self, rhs: Duration) -> Self::Output {
        Self {
            inner: self.inner - rhs,
            _tz_phanatom: PhantomData,
        }
    }
}

impl Deref for DateTime<Utc> {
    type Target = chrono::DateTime<chrono::Utc>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<chrono::DateTime<chrono::Utc>> for DateTime<Utc> {
    fn from(inner: chrono::DateTime<chrono::Utc>) -> Self {
        Self {
            inner,
            _tz_phanatom: PhantomData,
        }
    }
}

impl From<chrono::DateTime<Utc>> for DateTime<Utc> {
    fn from(inner: chrono::DateTime<Utc>) -> Self {
        let secs = inner.timestamp();
        let nanos = inner.timestamp_subsec_nanos();
        Self::from_timestamp(secs, nanos).unwrap()
    }
}
