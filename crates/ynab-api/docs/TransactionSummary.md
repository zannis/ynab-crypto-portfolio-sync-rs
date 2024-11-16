# TransactionSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**date** | [**String**](string.md) | The transaction date in ISO format (e.g. 2016-12-01) | 
**amount** | **i64** | The transaction amount in milliunits format | 
**memo** | Option<**String**> |  | [optional]
**cleared** | [**models::TransactionClearedStatus**](TransactionClearedStatus.md) |  | 
**approved** | **bool** | Whether or not the transaction is approved | 
**flag_color** | Option<[**models::TransactionFlagColor**](TransactionFlagColor.md)> |  | [optional]
**flag_name** | Option<**String**> | The customized name of a transaction flag | [optional]
**account_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**payee_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**category_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**transfer_account_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | If a transfer transaction, the account to which it transfers | [optional]
**transfer_transaction_id** | Option<**String**> | If a transfer transaction, the id of transaction on the other side of the transfer | [optional]
**matched_transaction_id** | Option<**String**> | If transaction is matched, the id of the matched transaction | [optional]
**import_id** | Option<**String**> | If the transaction was imported, this field is a unique (by account) import identifier.  If this transaction was imported through File Based Import or Direct Import and not through the API, the import_id will have the format: 'YNAB:[milliunit_amount]:[iso_date]:[occurrence]'.  For example, a transaction dated 2015-12-30 in the amount of -$294.23 USD would have an import_id of 'YNAB:-294230:2015-12-30:1'.  If a second transaction on the same account was imported and had the same date and same amount, its import_id would be 'YNAB:-294230:2015-12-30:2'. | [optional]
**import_payee_name** | Option<**String**> | If the transaction was imported, the payee name that was used when importing and before applying any payee rename rules | [optional]
**import_payee_name_original** | Option<**String**> | If the transaction was imported, the original payee name as it appeared on the statement | [optional]
**debt_transaction_type** | Option<**String**> | If the transaction is a debt/loan account transaction, the type of transaction | [optional]
**deleted** | **bool** | Whether or not the transaction has been deleted.  Deleted transactions will only be included in delta requests. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


