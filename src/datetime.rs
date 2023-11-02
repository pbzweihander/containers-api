//! Utility functions to handle dates and time.

use chrono::{DateTime, TimeZone, Utc};
use serde::Deserialize;

/// Used for deserialization of UNIX timestamp as chrono DateTime.
pub fn datetime_from_unix_timestamp<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let timestamp = chrono::NaiveDateTime::from_timestamp_opt(i64::deserialize(deserializer)?, 0)
        .unwrap_or_default();
    Ok(Utc.from_utc_datetime(&timestamp))
}

/// Used for deserialization of nano second timestamp as chrono DateTime.
pub fn datetime_from_nano_timestamp<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let timestamp_nano = u64::deserialize(deserializer)?;
    let timestamp = chrono::NaiveDateTime::from_timestamp_opt(
        (timestamp_nano / 1_000_000_000) as i64,
        (timestamp_nano % 1_000_000_000) as u32,
    )
    .unwrap_or_default();
    Ok(Utc.from_utc_datetime(&timestamp))
}
