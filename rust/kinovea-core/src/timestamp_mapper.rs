//! Mapping from one timestamp space to another (port of the `TimestampMapper` delegate).

/// A function from timestamp to timestamp, e.g. from video time to display time.
pub type TimestampMapper = Box<dyn Fn(i64) -> i64 + Send + Sync>;

/// The identity mapper.
pub fn identity(time: i64) -> i64 {
    time
}
