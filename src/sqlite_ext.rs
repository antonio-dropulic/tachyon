#![cfg(feature = "time_travel")]
use std::marker::PhantomData;

use rusqlite::{types::FromSql, ToSql};

use crate::date_time::DateTime;
use crate::utc::Utc;

impl ToSql for DateTime<Utc> {
    fn to_sql(&self) -> ::rusqlite::Result<::rusqlite::types::ToSqlOutput<'_>> {
        self.inner.to_sql()
    }
}

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
