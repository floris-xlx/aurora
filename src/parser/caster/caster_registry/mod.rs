pub mod revolut_csv;
use crate::parser::caster::caster_registry::revolut_csv::{
    RevolutTransactionOld, RevolutTransactionTarget,
};
use serde_json::{from_value, to_value, Map, Value};
use tracing::info;

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
    info!("{:#?} objmap", obj_map);
    // Clone the document_provider to avoid borrowing issues
    let document_provider: Option<String> = obj_map
        .get("document_provider")
        .and_then(|v| v.as_str().map(|s| s.to_string()));

    info!("document_provider: {:?}", document_provider);

    match document_provider.as_deref() {
        Some("revolut_csv") => {
            let revolut_transaction_old: RevolutTransactionOld =
                from_value(Value::Object(obj_map.clone()))
                    .map_err(|e| format!("Failed to deserialize RevolutTransactionOld: {}", e))?;
            info!("revolut_transaction_old: {:#?}", revolut_transaction_old);

            let revolut_transaction_target: RevolutTransactionTarget =
                revolut_transaction_old.to_target()?;

            info!(
                "revolut_transaction_target: {:#?}",
                revolut_transaction_target
            );

            let updated_data: Map<String, Value> = to_value(revolut_transaction_target)
                .map_err(|e| format!("Failed to serialize RevolutTransactionTarget: {}", e))?
                .as_object()
                .cloned()
                .ok_or("Failed to convert RevolutTransactionTarget to object")?;
            info!("updated_data: {:#?}", updated_data);

            obj_map.clear();
            obj_map.extend(updated_data);
        }
        // Add more document providers here as needed
        _ => {}
    }

    Ok(())
}
