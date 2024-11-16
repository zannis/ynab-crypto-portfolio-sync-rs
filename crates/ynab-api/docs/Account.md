# Account

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**name** | **String** |  | 
**r#type** | [**models::AccountType**](AccountType.md) |  | 
**on_budget** | **bool** | Whether this account is on budget or not | 
**closed** | **bool** | Whether this account is closed or not | 
**note** | Option<**String**> |  | [optional]
**balance** | **i64** | The current balance of the account in milliunits format | 
**cleared_balance** | **i64** | The current cleared balance of the account in milliunits format | 
**uncleared_balance** | **i64** | The current uncleared balance of the account in milliunits format | 
**transfer_payee_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The payee id which should be used when transferring to this account | 
**direct_import_linked** | Option<**bool**> | Whether or not the account is linked to a financial institution for automatic transaction import. | [optional]
**direct_import_in_error** | Option<**bool**> | If an account linked to a financial institution (direct_import_linked=true) and the linked connection is not in a healthy state, this will be true. | [optional]
**last_reconciled_at** | Option<**String**> | A date/time specifying when the account was last reconciled. | [optional]
**debt_original_balance** | Option<**i64**> | The original debt/loan account balance, specified in milliunits format. | [optional]
**debt_interest_rates** | Option<**std::collections::HashMap<String, i64>**> |  | [optional]
**debt_minimum_payments** | Option<**std::collections::HashMap<String, i64>**> |  | [optional]
**debt_escrow_amounts** | Option<**std::collections::HashMap<String, i64>**> |  | [optional]
**deleted** | **bool** | Whether or not the account has been deleted.  Deleted accounts will only be included in delta requests. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


