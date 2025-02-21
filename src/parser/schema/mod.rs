use serde_json::{json, Map, Value};
use std::collections::HashSet;
use tracing::{info, warn};

/// Enum representing different key sets for various schemas.
pub enum SchemaKeys {
    Revolut,
    ShopifyOrders,
}

impl SchemaKeys {
    pub fn keys(&self) -> HashSet<&'static str> {
        match self {
            SchemaKeys::Revolut => vec![
                "completed_date",
                "amount",
                "fee",
                "description",
                "product",
                "type",
                "state",
                "currency",
                "balance",
                "started_date",
            ]
            .into_iter()
            .collect(),
            SchemaKeys::ShopifyOrders => vec![
                "order_id",
                "customer",
                "total_price",
                "currency",
                "order_date",
                "fulfillment_status",
                "line_items",
                "shipping_address",
                "billing_address",
            ]
            .into_iter()
            .collect(),
        }
    }
}

/// Represents a struct with a name and a set of expected keys.
pub struct RevolutPersonalSchema {
    pub name: String,
    pub keys: Vec<String>,
}

/// Determines the most likely struct match for a given JSON object based on key similarity.
///
/// # Arguments
///
/// * `object` - A reference to a `serde_json::Value` which is expected to be a JSON array of objects.
/// * `schemas` - A slice of `StructSchema` representing the available struct schemas to match against.
///
/// # Returns
///
/// A `Value` representing the JSON array with an added key "document_provider" indicating the name of the struct that best matches the keys in the objects.
pub fn determine_document_provider(object: &Value, schemas: &[RevolutPersonalSchema]) -> Value {
    info!("Determining document provider for object: {:#?}", object);
    let mut result: Value = object.clone();

    if !object.is_array() {
        warn!("Provided value is not an array. Marking document provider as Unknown.");
        return json!([{
            "document_provider": "unknown",
            "data": object
        }]);
    }

    let revolut_keys: HashSet<&str> = SchemaKeys::Revolut.keys();
    let shopify_keys: HashSet<&str> = SchemaKeys::ShopifyOrders.keys();

    let mut document_provider: String = "unknown".to_string();

    if let Some(array) = object.as_array() {
        for item in array {
            if let Some(obj_map) = item.as_object() {
                if obj_map
                    .keys()
                    .all(|key| revolut_keys.contains(key.as_str()))
                {
                    document_provider = "revolut_csv".to_string();
                    break;
                } else if obj_map
                    .keys()
                    .all(|key| shopify_keys.contains(key.as_str()))
                {
                    document_provider = "shopify_orders_csv".to_string();
                    break;
                }
            }
        }
    }

    for schema in schemas {
        if let Some(array) = object.as_array() {
            for item in array {
                if let Some(obj_map) = item.as_object() {
                    if schema.keys.iter().all(|key| obj_map.contains_key(key)) {
                        document_provider = schema.name.clone();
                        break;
                    }
                }
            }
        }
    }

    info!("Document provider determined: {}", document_provider);

    if let Some(array) = result.as_array_mut() {
        for item in array {
            if let Some(obj_map) = item.as_object_mut() {
                let data_value: Map<String, Value> = obj_map.clone();
                obj_map.clear();
                obj_map.insert("data".to_string(), Value::Object(data_value));
                obj_map.insert(
                    "document_provider".to_string(),
                    Value::String(document_provider.clone()),
                );
            }
        }
    }

    result
}
