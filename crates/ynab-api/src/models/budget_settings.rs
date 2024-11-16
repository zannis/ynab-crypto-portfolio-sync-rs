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
pub struct BudgetSettings {
    #[serde(rename = "date_format", deserialize_with = "Option::deserialize")]
    pub date_format: Option<Box<models::DateFormat>>,
    #[serde(rename = "currency_format", deserialize_with = "Option::deserialize")]
    pub currency_format: Option<Box<models::CurrencyFormat>>,
}

impl BudgetSettings {
    pub fn new(date_format: Option<models::DateFormat>, currency_format: Option<models::CurrencyFormat>) -> BudgetSettings {
        BudgetSettings {
            date_format: if let Some(x) = date_format {Some(Box::new(x))} else {None},
            currency_format: if let Some(x) = currency_format {Some(Box::new(x))} else {None},
        }
    }
}

