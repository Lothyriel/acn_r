use anyhow::Result;
use chrono::{DateTime, SecondsFormat, Utc};

pub fn encode_id(value: u64) -> Result<i64> {
    Ok(i64::try_from(value)?)
}

pub fn decode_id(value: i64) -> Result<u64> {
    Ok(u64::try_from(value)?)
}

pub fn encode_optional_id(value: Option<u64>) -> Result<Option<i64>> {
    value.map(encode_id).transpose()
}

pub fn encode_datetime(value: DateTime<Utc>) -> String {
    value.to_rfc3339_opts(SecondsFormat::Millis, true)
}

pub fn decode_datetime(value: &str) -> Result<DateTime<Utc>> {
    Ok(DateTime::parse_from_rfc3339(value)?.with_timezone(&Utc))
}

pub fn encode_bool(value: bool) -> i64 {
    if value { 1 } else { 0 }
}
