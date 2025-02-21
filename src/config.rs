//! ## Config for API
//!
//! This module provides functionality to construct the full path to a script
//! by combining the script directory, obtained from an environment variable,
//! with the script name provided as an argument. It also provides a function
//! to retrieve the API port from an environment variable.

use std::env;
use tracing::{error, info, warn};

/// Constructs the full path to a script by appending the script name to the
/// script directory path obtained from the `AURORA_SCRIPT_DIR`
/// environment variable.
///
/// # Arguments
///
/// * `script_name` - A string slice that holds the name of the script.
///
/// # Returns
///
/// A `String` representing the full path to the script.
///
/// # Panics
///
/// This function will panic if the `AURORA_SCRIPT_DIR` environment
/// variable is not set.
///
pub fn script_path(script_name: &str) -> String {
    if !script_name.ends_with(".sh") {
        error!("Script name must end with .sh");
    }
    // Get the script directory from the environment variable
    let script_dir: String =
        env::var("AURORA_SCRIPT_DIR").expect("AURORA_SCRIPT_DIR environment variable not set");

    // Construct the script path
    format!("{}/{}", script_dir, script_name)
}


/// Retrieves the API port from the `AURORA_API_PORT` environment variable.
///
/// # Returns
///
/// A `u16` representing the API port. Defaults to 7777 if the environment
/// variable is not set or cannot be parsed.
pub fn get_api_port() -> u16 {
    env::var("AURORA_API_PORT")
        .unwrap_or_else(|_| "7777".to_string())
        .parse()
        .unwrap_or(7777)
}
