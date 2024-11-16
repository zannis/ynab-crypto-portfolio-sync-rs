# BudgetSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**name** | **String** |  | 
**last_modified_on** | Option<**String**> | The last time any changes were made to the budget from either a web or mobile client | [optional]
**first_month** | Option<[**String**](string.md)> | The earliest budget month | [optional]
**last_month** | Option<[**String**](string.md)> | The latest budget month | [optional]
**date_format** | Option<[**models::DateFormat**](DateFormat.md)> |  | [optional]
**currency_format** | Option<[**models::CurrencyFormat**](CurrencyFormat.md)> |  | [optional]
**accounts** | Option<[**Vec<models::Account>**](Account.md)> | The budget accounts (only included if `include_accounts=true` specified as query parameter) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


