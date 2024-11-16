# MonthDetail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**month** | [**String**](string.md) |  | 
**note** | Option<**String**> |  | [optional]
**income** | **i64** | The total amount of transactions categorized to 'Inflow: Ready to Assign' in the month | 
**budgeted** | **i64** | The total amount budgeted in the month | 
**activity** | **i64** | The total amount of transactions in the month, excluding those categorized to 'Inflow: Ready to Assign' | 
**to_be_budgeted** | **i64** | The available amount for 'Ready to Assign' | 
**age_of_money** | Option<**i32**> | The Age of Money as of the month | [optional]
**deleted** | **bool** | Whether or not the month has been deleted.  Deleted months will only be included in delta requests. | 
**categories** | [**Vec<models::Category>**](Category.md) | The budget month categories.  Amounts (budgeted, activity, balance, etc.) are specific to the {month} parameter specified. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


