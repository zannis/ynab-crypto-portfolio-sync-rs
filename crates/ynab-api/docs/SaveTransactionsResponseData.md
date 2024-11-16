# SaveTransactionsResponseData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transaction_ids** | **Vec<String>** | The transaction ids that were saved | 
**transaction** | Option<[**models::TransactionDetail**](TransactionDetail.md)> |  | [optional]
**transactions** | Option<[**Vec<models::TransactionDetail>**](TransactionDetail.md)> | If multiple transactions were specified, the transactions that were saved | [optional]
**duplicate_import_ids** | Option<**Vec<String>**> | If multiple transactions were specified, a list of import_ids that were not created because of an existing `import_id` found on the same account | [optional]
**server_knowledge** | **i64** | The knowledge of the server | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


