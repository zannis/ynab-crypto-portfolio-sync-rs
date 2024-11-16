# SubTransaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**transaction_id** | **String** |  | 
**amount** | **i64** | The subtransaction amount in milliunits format | 
**memo** | Option<**String**> |  | [optional]
**payee_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**payee_name** | Option<**String**> |  | [optional]
**category_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**category_name** | Option<**String**> |  | [optional]
**transfer_account_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | If a transfer, the account_id which the subtransaction transfers to | [optional]
**transfer_transaction_id** | Option<**String**> | If a transfer, the id of transaction on the other side of the transfer | [optional]
**deleted** | **bool** | Whether or not the subtransaction has been deleted.  Deleted subtransactions will only be included in delta requests. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


