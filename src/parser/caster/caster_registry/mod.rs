pub mod revolut_csv;

use crate::parser::caster::caster_registry::revolut_csv::{
    RevolutTransactionOld, RevolutTransactionTarget,
};
use serde_json::{from_value, to_value, Map, Value};

pub async fn cast_transactions(json_array: &mut Value) -> Result<Value, String> {
    if let Some(array) = json_array.as_array_mut() {
        for item in array {
            if let Some(obj_map) = item.as_object_mut() {
                process_transaction(obj_map).await?;
            }
        }
    }
    Ok(json_array.clone())
}

async fn process_transaction(obj_map: &mut Map<String, Value>) -> Result<(), String> {
    // Clone the document_provider to avoid borrowing issues
    let document_provider: Option<String> = obj_map
        .get("document_provider")
        .and_then(|v| v.as_str().map(|s| s.to_string()));

    let data: &mut Value = match obj_map.get_mut("data") {
        Some(data) => data,
        None => return Ok(()),
    };

    let data_map: &Map<String, Value> = match data.as_object() {
        Some(data_map) => data_map,
        None => return Ok(()),
    };

    match document_provider.as_deref() {
        Some("revolut_csv") => {
            let revolut_transaction_old: RevolutTransactionOld =
                from_value(Value::Object(data_map.clone()))
                    .map_err(|e| format!("Failed to deserialize RevolutTransactionOld: {}", e))?;

            let revolut_transaction_target: RevolutTransactionTarget =
                revolut_transaction_old.to_target()?;

            let updated_data: Map<String, Value> = to_value(revolut_transaction_target)
                .map_err(|e| format!("Failed to serialize RevolutTransactionTarget: {}", e))?
                .as_object()
                .cloned()
                .ok_or("Failed to convert RevolutTransactionTarget to object")?;

            *data = Value::Object(updated_data);
        }
        // Add more document providers here as needed
        _ => {}
    }

    Ok(())
}
