use serde_json::{Map, Value};
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
/// * `object` - A reference to a `serde_json::Value` which is expected to be a JSON object.
/// * `schemas` - A slice of `StructSchema` representing the available struct schemas to match against.
///
/// # Returns
///
/// A `Value` representing the JSON object with an added key "document_provider" indicating the name of the struct that best matches the keys in the object.
pub fn determine_document_provider(object: &Value, schemas: &[RevolutPersonalSchema]) -> Value {
    info!("Determining document provider for object: {:#?}", object);
    let mut result: Value = object.clone();

    if !object.is_object() {
        warn!("Provided value is not an object. Marking document provider as Unknown.");
        result.as_object_mut().unwrap().insert(
            "document_provider".to_string(),
            Value::String("Unknown".to_string()),
        );
        return result;
    }

    let obj_map: &Map<String, Value> = object.as_object().unwrap();
    let revolut_keys: HashSet<&str> = SchemaKeys::Revolut.keys();
    let shopify_keys: HashSet<&str> = SchemaKeys::ShopifyOrders.keys();

    // Check if the object contains only the specific keys for Revolut
    if obj_map
        .keys()
        .all(|key| revolut_keys.contains(key.as_str()))
    {
        info!("Object matches all Revolut keys.");
        result.as_object_mut().unwrap().insert(
            "document_provider".to_string(),
            Value::String("Revolut".to_string()),
        );
        return result;
    }

    // Check if the object contains only the specific keys for ShopifyOrders
    if obj_map
        .keys()
        .all(|key| shopify_keys.contains(key.as_str()))
    {
        info!("Object matches all ShopifyOrders keys.");
        result.as_object_mut().unwrap().insert(
            "document_provider".to_string(),
            Value::String("ShopifyOrders".to_string()),
        );
        return result;
    }

    let mut best_match: Option<&RevolutPersonalSchema> = None;
    let mut max_matches: usize = 0;

    for schema in schemas {
        let matches: usize = schema
            .keys
            .iter()
            .filter(|key| obj_map.contains_key(*key))
            .count();
        info!("Schema '{}' has {} matching keys.", schema.name, matches);
        if matches > max_matches {
            max_matches = matches;
            best_match = Some(schema);
        }
    }

    // Check all schemas for a complete match
    for schema in schemas {
        if schema.keys.iter().all(|key| obj_map.contains_key(key)) {
            info!("Object completely matches schema '{}'.", schema.name);
            result.as_object_mut().unwrap().insert(
                "document_provider".to_string(),
                Value::String(schema.name.clone()),
            );
            return result;
        }
    }

    let provider_name: String =
        best_match.map_or("Unknown".to_string(), |schema| schema.name.clone());
    info!("Best match for document provider is '{}'.", provider_name);
    result.as_object_mut().unwrap().insert(
        "document_provider".to_string(),
        Value::String(provider_name),
    );
    result
}
