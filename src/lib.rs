//! The crate republishes chrono.
//! With default features the crate does nothing.
//! With `time_travel` enabled [DateTime<Utc>::now] and [Utc::now] are
//! replaced with a mocked version. The mock is controled by the functions in the
//! [tachyon] module.
//!
//!

mod date_time;
mod mock;
mod sqlite_ext;
mod utc;

pub use chrono::*;

#[cfg(feature = "time_travel")]
pub use date_time::DateTime;
#[cfg(feature = "time_travel")]
pub use mock::tachyon;
#[cfg(feature = "time_travel")]
pub use utc::Utc;

// WARN:
// the issue with this approach is that the runtime behavior with time-travel enabled
// could, and most likely will, be broken for tests that don't use the feature.
// This means testing with cargo test --all-features will most likely result in errors.
// This lib is meant for a specific usecase with featureless apps, where you can
// run only time-travel tests with time travel feature. I achieve this by feature gating
// the tests with time travel, and adding a prefix to test module containing those (time_travel)
// THen i can run cargo test time_travel --features time_travel.
// Ofc i make sure the tests run sequentially. (this lib does that through cargo/config.toml)

// TODO:
// I like this approach because it allows the app code to not care about time mocking.
// A sort of on/off global mock.
// What i really don't like is the gymnastics needed to execute those tests. I would like to
// figure out a better way to do this.
