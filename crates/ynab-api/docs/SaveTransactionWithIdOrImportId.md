# SaveTransactionWithIdOrImportId

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**date** | Option<[**String**](string.md)> | The transaction date in ISO format (e.g. 2016-12-01).  Future dates (scheduled transactions) are not permitted.  Split transaction dates cannot be changed and if a different date is supplied it will be ignored. | [optional]
**amount** | Option<**i64**> | The transaction amount in milliunits format.  Split transaction amounts cannot be changed and if a different amount is supplied it will be ignored. | [optional]
**payee_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The payee for the transaction.  To create a transfer between two accounts, use the account transfer payee pointing to the target account.  Account transfer payees are specified as `transfer_payee_id` on the account resource. | [optional]
**payee_name** | Option<**String**> | The payee name.  If a `payee_name` value is provided and `payee_id` has a null value, the `payee_name` value will be used to resolve the payee by either (1) a matching payee rename rule (only if `import_id` is also specified) or (2) a payee with the same name or (3) creation of a new payee. | [optional]
**category_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The category for the transaction.  To configure a split transaction, you can specify null for `category_id` and provide a `subtransactions` array as part of the transaction object.  If an existing transaction is a split, the `category_id` cannot be changed.  Credit Card Payment categories are not permitted and will be ignored if supplied. | [optional]
**memo** | Option<**String**> |  | [optional]
**cleared** | Option<[**models::TransactionClearedStatus**](TransactionClearedStatus.md)> |  | [optional]
**approved** | Option<**bool**> | Whether or not the transaction is approved.  If not supplied, transaction will be unapproved by default. | [optional]
**flag_color** | Option<[**models::TransactionFlagColor**](TransactionFlagColor.md)> |  | [optional]
**subtransactions** | Option<[**Vec<models::SaveSubTransaction>**](SaveSubTransaction.md)> | An array of subtransactions to configure a transaction as a split. Updating `subtransactions` on an existing split transaction is not supported. | [optional]
**id** | Option<**String**> | If specified, this id will be used to lookup a transaction by its `id` for the purpose of updating the transaction itself. If not specified, an `import_id` should be supplied. | [optional]
**import_id** | Option<**String**> | If specified, this id will be used to lookup a transaction by its `import_id` for the purpose of updating the transaction itself. If not specified, an `id` should be supplied.  You may not provide both an `id` and an `import_id` and updating an `import_id` on an existing transaction is not allowed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


