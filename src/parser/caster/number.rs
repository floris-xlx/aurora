use serde_json::{Map, Value};
use tracing::{error, info};

/// Attempts to cast a string value to f64 and updates the JSON value if successful.
///
/// # Arguments
///
/// * `value` - A mutable reference to a `serde_json::Value` which is expected to be a string.
pub fn try_cast_to_f64(value: &mut Value) {
    if let Some(str_value) = value.as_str() {
        if let Ok(num_value) = str_value.parse::<f64>() {
            *value = Value::from(num_value);
        }
    }
}

/// Casts the specified keys in a JSON array of objects to f64 and returns the modified JSON.
///
/// # Arguments
///
/// * `data` - A mutable reference to a `serde_json::Value` which is expected to be an array of JSON objects.
///
/// # Keys to be casted
///
/// * `balance`
/// * `fee`
/// * `amount`
pub fn cast_keys_to_f64(data: &mut Value) -> Value {
    if data.is_array() {
        info!("Starting to cast keys to f64 in JSON array.");
        for (index, obj) in data.as_array_mut().unwrap().iter_mut().enumerate() {
            if !obj.is_object() {
                error!("Element at index {} is not an object, skipping.", index);
                continue;
            }

            let obj_map: &mut Map<String, Value> = obj.as_object_mut().unwrap();
            for key in &["balance", "fee", "amount"] {
                if let Some(value) = obj_map.get_mut(*key) {
                    try_cast_to_f64(value);
                } else {
                    info!(
                        "Key '{}' not found in object at index {}, skipping.",
                        key, index
                    );
                }
            }
        }
        info!("Finished casting keys to f64 in JSON array.");
    } else {
        error!("Provided data is not an array.");
    }
    data.clone()
}
