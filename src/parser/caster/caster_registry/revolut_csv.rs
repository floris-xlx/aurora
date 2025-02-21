use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;

use crate::parser::caster::number::try_cast_to_f64;
use crate::parser::caster::time::try_cast_to_unix;

/// Represents a transaction in the Revolut system.
///
/// This struct is used to deserialize and serialize transaction data
/// from and to JSON format. It includes various fields that describe
/// the details of a transaction.
///
/// # Fields
///
/// * `transaction_type` - A `String` representing the type of the transaction.
/// * `amount` - A `String` representing the amount of the transaction, to be casted to `f64`.
/// * `balance` - A `String` representing the balance after the transaction, to be casted to `f64`.
/// * `product` - A `String` representing the product associated with the transaction.
/// * `state` - A `String` representing the state of the transaction.
/// * `started_date` - A `String` representing the date when the transaction started, to be casted to `i64`.
/// * `currency` - A `String` representing the currency used in the transaction.
/// * `completed_date` - A `String` representing the date when the transaction was completed, to be casted to `i64`.
/// * `fee` - A `String` representing the fee associated with the transaction, to be casted to `f64`.
/// * `description` - A `String` providing a description of the transaction.
#[derive(Serialize, Deserialize, Debug)]
pub struct RevolutTransactionOld {
    pub transaction_type: String,
    pub amount: String,  // To be casted to f64
    pub balance: String, // To be casted to f64
    pub product: String,
    pub state: String,
    pub started_date: String, // To be casted to i64
    pub currency: String,
    pub completed_date: String, // To be casted to i64
    pub fee: String,            // To be casted to f64
    pub description: String,
}

/// Represents a transaction in the Revolut system with fields casted to appropriate types.
///
/// This struct is used to deserialize and serialize transaction data
/// from and to JSON format. It includes various fields that describe
/// the details of a transaction with numeric and date fields casted
/// to `f64` and `i64` respectively.
#[derive(Serialize, Deserialize, Debug)]
pub struct RevolutTransactionTarget {
    /// A `String` representing the type of the transaction.
    pub transaction_type: String,
    /// A `f64` representing the amount of the transaction.
    pub amount: f64,
    /// A `f64` representing the balance after the transaction.
    pub balance: f64,
    /// A `String` representing the product associated with the transaction.
    pub product: String,
    /// A `String` representing the state of the transaction.
    pub state: String,
    /// An `i64` representing the date when the transaction started.
    pub started_date: i64,
    /// A `String` representing the currency used in the transaction.
    pub currency: String,
    /// An `i64` representing the date when the transaction was completed.
    pub completed_date: i64,
    /// A `f64` representing the fee associated with the transaction.
    pub fee: f64,
    /// A `String` providing a description of the transaction.
    pub description: String,
}

impl RevolutTransactionOld {
    /// Converts an instance of `RevolutTransactionOld` to `RevolutTransactionTarget`.
    ///
    /// This method attempts to cast string fields to their respective types
    /// (`f64` for numeric fields and `i64` for date fields) and returns a
    /// `RevolutTransactionTarget` if successful.
    ///
    /// # Returns
    ///
    /// A `Result` which is:
    /// - `Ok(RevolutTransactionTarget)` if all fields are successfully casted.
    /// - `Err(String)` if any field fails to cast.
    pub fn to_target(&self) -> Result<RevolutTransactionTarget, String> {
        let mut amount_value: Value = Value::String(self.amount.clone());
        try_cast_to_f64(&mut amount_value);
        let amount = amount_value
            .as_f64()
            .ok_or("Failed to cast amount to f64")?;

        let mut balance_value: Value = Value::String(self.balance.clone());
        try_cast_to_f64(&mut balance_value);
        let balance = balance_value
            .as_f64()
            .ok_or("Failed to cast balance to f64")?;

        let mut fee_value: Value = Value::String(self.fee.clone());
        try_cast_to_f64(&mut fee_value);
        let fee = fee_value.as_f64().ok_or("Failed to cast fee to f64")?;

        let mut started_date_value: Value = Value::String(self.started_date.clone());
        try_cast_to_unix(&mut started_date_value);
        let started_date = started_date_value
            .as_i64()
            .ok_or("Failed to cast started_date to i64")?;

        let mut completed_date_value: Value = Value::String(self.completed_date.clone());
        try_cast_to_unix(&mut completed_date_value);
        let completed_date = completed_date_value
            .as_i64()
            .ok_or("Failed to cast completed_date to i64")?;

        Ok(RevolutTransactionTarget {
            transaction_type: self.transaction_type.clone(),
            amount,
            balance,
            product: self.product.clone(),
            state: self.state.clone(),
            started_date,
            currency: self.currency.clone(),
            completed_date,
            fee,
            description: self.description.clone(),
        })
    }
}
