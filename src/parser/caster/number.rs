use serde_json::{Map, Value};
use tracing::{error, info};

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
                let value: &mut Value = match obj_map.get_mut(*key) {
                    Some(v) => v,
                    None => {
                        info!(
                            "Key '{}' not found in object at index {}, skipping.",
                            key, index
                        );
                        continue;
                    }
                };
                let str_value: &str = match value.as_str() {
                    Some(s) => s,
                    None => {
                        error!(
                            "Value for key '{}' in object at index {} is not a string, skipping.",
                            key, index
                        );
                        continue;
                    }
                };
                match str_value.parse::<f64>() {
                    Ok(num_value) => {
                        *value = Value::from(num_value);
                        info!(
                            "Successfully casted key '{}' to f64 in object at index {}.",
                            key, index
                        );
                    }
                    Err(e) => {
                        error!(
                            "Failed to parse value '{}' for key '{}' in object at index {}: {:?}",
                            str_value, key, index, e
                        );
                    }
                }
            }
        }
        info!("Finished casting keys to f64 in JSON array.");
    } else {
        error!("Provided data is not an array.");
    }
    data.clone()
}
