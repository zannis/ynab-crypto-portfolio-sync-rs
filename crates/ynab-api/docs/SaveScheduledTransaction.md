# SaveScheduledTransaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**date** | [**String**](string.md) | The scheduled transaction date in ISO format (e.g. 2016-12-01).  This should be a future date no more than 5 years into the future. | 
**amount** | Option<**i64**> | The scheduled transaction amount in milliunits format. | [optional]
**payee_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The payee for the scheduled transaction.  To create a transfer between two accounts, use the account transfer payee pointing to the target account.  Account transfer payees are specified as `transfer_payee_id` on the account resource. | [optional]
**payee_name** | Option<**String**> | The payee name for the the scheduled transaction.  If a `payee_name` value is provided and `payee_id` has a null value, the `payee_name` value will be used to resolve the payee by either (1) a payee with the same name or (2) creation of a new payee. | [optional]
**category_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The category for the scheduled transaction. Credit Card Payment categories are not permitted. Creating a split scheduled transaction is not currently supported. | [optional]
**memo** | Option<**String**> |  | [optional]
**flag_color** | Option<[**models::TransactionFlagColor**](TransactionFlagColor.md)> |  | [optional]
**frequency** | Option<[**models::ScheduledTransactionFrequency**](ScheduledTransactionFrequency.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


