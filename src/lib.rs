mod mock;

use std::{
    marker::PhantomData,
    ops::{Add, Deref, Sub},
};

pub use chrono::*;

#[cfg(feature = "time_travel")]
use ::rusqlite::{types::FromSql, ToSql};
#[cfg(feature = "time_travel")]
pub use mock::tachyon;

#[cfg(feature = "time_travel")]
#[allow(hidden_glob_reexports)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DateTime<Tz: TimeZone> {
    pub(crate) inner: chrono::DateTime<chrono::Utc>,
    _tz_phanatom: PhantomData<Tz>,
}

#[cfg(feature = "time_travel")]
impl DateTime<Utc> {
    pub fn now() -> Self {
        tachyon::now()
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

#[cfg(feature = "time_travel")]
impl Add<Duration> for DateTime<Utc> {
    type Output = DateTime<Utc>;

    fn add(self, rhs: Duration) -> Self::Output {
        Self {
            inner: self.inner + rhs,
            _tz_phanatom: PhantomData,
        }
    }
}

#[cfg(feature = "time_travel")]
impl Sub<Duration> for DateTime<Utc> {
    type Output = DateTime<Utc>;

    fn sub(self, rhs: Duration) -> Self::Output {
        Self {
            inner: self.inner - rhs,
            _tz_phanatom: PhantomData,
        }
    }
}

#[cfg(feature = "time_travel")]
impl Deref for DateTime<Utc> {
    type Target = chrono::DateTime<chrono::Utc>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[cfg(feature = "time_travel")]
impl ToSql for DateTime<Utc> {
    fn to_sql(&self) -> ::rusqlite::Result<::rusqlite::types::ToSqlOutput<'_>> {
        self.inner.to_sql()
    }
}

#[cfg(feature = "time_travel")]
impl FromSql for DateTime<Utc> {
    fn column_result(
        value: ::rusqlite::types::ValueRef<'_>,
    ) -> ::rusqlite::types::FromSqlResult<Self> {
        Ok(Self {
            inner: <chrono::DateTime<chrono::Utc> as FromSql>::column_result(value)?,
            _tz_phanatom: PhantomData,
        })
    }
}

#[cfg(feature = "time_travel")]
#[allow(hidden_glob_reexports)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Utc;

#[cfg(feature = "time_travel")]
impl Utc {
    pub fn now() -> DateTime<Utc> {
        DateTime::now()
    }
}

#[cfg(feature = "time_travel")]
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
#[cfg(feature = "time_travel")]
impl Offset for Utc {
    fn fix(&self) -> FixedOffset {
        FixedOffset::east_opt(0).unwrap()
    }
}
