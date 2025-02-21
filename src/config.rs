//! ## Config for API
//!
//! This module provides functionality to construct the full path to a script
//! by combining the script directory, obtained from an environment variable,
//! with the script name provided as an argument.

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
