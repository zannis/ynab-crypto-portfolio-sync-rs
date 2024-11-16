/*
 * YNAB API Endpoints
 *
 * Our API uses a REST based design, leverages the JSON data format, and relies upon HTTPS for transport. We respond with meaningful HTTP response codes and if an error occurs, we include error details in the response body.  API Documentation is at https://api.ynab.com
 *
 * The version of the OpenAPI document: 1.72.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SaveAccount {
    /// The name of the account
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: models::AccountType,
    /// The current balance of the account in milliunits format
    #[serde(rename = "balance")]
    pub balance: i64,
}

impl SaveAccount {
    pub fn new(name: String, r#type: models::AccountType, balance: i64) -> SaveAccount {
        SaveAccount {
            name,
            r#type,
            balance,
        }
    }
}
