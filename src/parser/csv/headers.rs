use csv::StringRecord;

/// Converts CSV headers to lowercase and replaces spaces with underscores.
///
/// # Arguments
///
/// * `headers` - A `StringRecord` containing the CSV headers.
///
/// # Returns
///
/// A `StringRecord` with headers converted to lowercase and spaces replaced with underscores.
pub fn normalize_headers(headers: &StringRecord) -> StringRecord {
    headers
        .iter()
        .map(|header| header.to_lowercase().replace(' ', "_"))
        .collect()
}
