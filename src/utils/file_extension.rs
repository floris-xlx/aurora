use std::path::Path;

/// Extracts the file extension from a given file URL.
///
/// # Arguments
///
/// * `file_url` - A string slice that holds the file URL.
///
/// # Returns
///
/// * A `String` representing the file extension. If no extension is found, returns an empty string.
pub fn get_file_extension(file_url: &str) -> String {
    let path: &Path = Path::new(file_url);
    path.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_string()
}
