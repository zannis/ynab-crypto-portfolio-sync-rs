/*
 * Binance Public Spot API
 *
 * OpenAPI Specifications for the Binance Public Spot API  API documents:   - [https://github.com/binance/binance-spot-api-docs](https://github.com/binance/binance-spot-api-docs)   - [https://binance-docs.github.io/apidocs/spot/en](https://binance-docs.github.io/apidocs/spot/en)
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    /// Error code
    #[serde(rename = "code")]
    pub code: i64,
    /// Error message
    #[serde(rename = "msg")]
    pub msg: String,
}

impl Error {
    pub fn new(code: i64, msg: String) -> Error {
        Error {
            code,
            msg,
        }
    }
}

