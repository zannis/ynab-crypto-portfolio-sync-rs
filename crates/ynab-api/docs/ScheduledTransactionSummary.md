# ScheduledTransactionSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**date_first** | [**String**](string.md) | The first date for which the Scheduled Transaction was scheduled. | 
**date_next** | [**String**](string.md) | The next date for which the Scheduled Transaction is scheduled. | 
**frequency** | **String** |  | 
**amount** | **i64** | The scheduled transaction amount in milliunits format | 
**memo** | Option<**String**> |  | [optional]
**flag_color** | Option<[**models::TransactionFlagColor**](TransactionFlagColor.md)> |  | [optional]
**flag_name** | Option<**String**> | The customized name of a transaction flag | [optional]
**account_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**payee_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**category_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**transfer_account_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | If a transfer, the account_id which the scheduled transaction transfers to | [optional]
**deleted** | **bool** | Whether or not the scheduled transaction has been deleted.  Deleted scheduled transactions will only be included in delta requests. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


