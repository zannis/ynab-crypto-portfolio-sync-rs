# \PayeesApi

All URIs are relative to *https://api.ynab.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_payee_by_id**](PayeesApi.md#get_payee_by_id) | **GET** /budgets/{budget_id}/payees/{payee_id} | Single payee
[**get_payees**](PayeesApi.md#get_payees) | **GET** /budgets/{budget_id}/payees | List payees
[**update_payee**](PayeesApi.md#update_payee) | **PATCH** /budgets/{budget_id}/payees/{payee_id} | Update a payee



## get_payee_by_id

> models::PayeeResponse get_payee_by_id(budget_id, payee_id)
Single payee

Returns a single payee

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**budget_id** | **String** | The id of the budget. \"last-used\" can be used to specify the last used budget and \"default\" can be used if default budget selection is enabled (see: https://api.ynab.com/#oauth-default-budget). | [required] |
**payee_id** | **String** | The id of the payee | [required] |

### Return type

[**models::PayeeResponse**](PayeeResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_payees

> models::PayeesResponse get_payees(budget_id, last_knowledge_of_server)
List payees

Returns all payees

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**budget_id** | **String** | The id of the budget. \"last-used\" can be used to specify the last used budget and \"default\" can be used if default budget selection is enabled (see: https://api.ynab.com/#oauth-default-budget). | [required] |
**last_knowledge_of_server** | Option<**i64**> | The starting server knowledge.  If provided, only entities that have changed since `last_knowledge_of_server` will be included. |  |

### Return type

[**models::PayeesResponse**](PayeesResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_payee

> models::SavePayeeResponse update_payee(budget_id, payee_id, data)
Update a payee

Update a payee

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**budget_id** | **String** | The id of the budget. \"last-used\" can be used to specify the last used budget and \"default\" can be used if default budget selection is enabled (see: https://api.ynab.com/#oauth-default-budget). | [required] |
**payee_id** | **String** | The id of the payee | [required] |
**data** | [**PatchPayeeWrapper**](PatchPayeeWrapper.md) | The payee to update | [required] |

### Return type

[**models::SavePayeeResponse**](SavePayeeResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

