use chrono::{NaiveDateTime, ParseError};
use serde_json::Value;
use tracing::{error, info};

/// Attempts to cast a string value to Unix time seconds and updates the JSON value if successful.
///
/// # Arguments
///
/// * `value` - A mutable reference to a `serde_json::Value` which is expected to be a string.
pub fn try_cast_to_unix(value: &mut Value) {
    if let Some(str_value) = value.as_str() {
        match parse_to_unix_time(str_value) {
            Ok(unix_time) => {
                *value = Value::from(unix_time);
                info!("Successfully casted value to Unix time.");
            }
            Err(e) => {
                error!(
                    "Failed to parse value '{}' to Unix time: {:?}",
                    str_value, e
                );
            }
        }
    } else {
        error!("Provided value is not a string, skipping.");
    }
}

/// Parses a date string to Unix time seconds.
///
/// # Arguments
///
/// * `date_str` - A string slice that holds the date in "YYYY-MM-DD HH:MM:SS" format.
///
/// # Returns
///
/// A `Result` which is:
/// - `Ok(i64)` containing the Unix time seconds if parsing is successful.
/// - `Err(ParseError)` if parsing fails.
fn parse_to_unix_time(date_str: &str) -> Result<i64, ParseError> {
    let naive_datetime: NaiveDateTime = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S")?;
    Ok(naive_datetime.timestamp())
}
