# \TransactionsApi

All URIs are relative to *https://api.ynab.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_transaction**](TransactionsApi.md#create_transaction) | **POST** /budgets/{budget_id}/transactions | Create a single transaction or multiple transactions
[**delete_transaction**](TransactionsApi.md#delete_transaction) | **DELETE** /budgets/{budget_id}/transactions/{transaction_id} | Deletes an existing transaction
[**get_transaction_by_id**](TransactionsApi.md#get_transaction_by_id) | **GET** /budgets/{budget_id}/transactions/{transaction_id} | Single transaction
[**get_transactions**](TransactionsApi.md#get_transactions) | **GET** /budgets/{budget_id}/transactions | List transactions
[**get_transactions_by_account**](TransactionsApi.md#get_transactions_by_account) | **GET** /budgets/{budget_id}/accounts/{account_id}/transactions | List account transactions
[**get_transactions_by_category**](TransactionsApi.md#get_transactions_by_category) | **GET** /budgets/{budget_id}/categories/{category_id}/transactions | List category transactions, excluding any pending transactions
[**get_transactions_by_month**](TransactionsApi.md#get_transactions_by_month) | **GET** /budgets/{budget_id}/months/{month}/transactions | List transactions in month, excluding any pending transactions
[**get_transactions_by_payee**](TransactionsApi.md#get_transactions_by_payee) | **GET** /budgets/{budget_id}/payees/{payee_id}/transactions | List payee transactions, excluding any pending transactions
[**import_transactions**](TransactionsApi.md#import_transactions) | **POST** /budgets/{budget_id}/transactions/import | Import transactions
[**update_transaction**](TransactionsApi.md#update_transaction) | **PUT** /budgets/{budget_id}/transactions/{transaction_id} | Updates an existing transaction
[**update_transactions**](TransactionsApi.md#update_transactions) | **PATCH** /budgets/{budget_id}/transactions | Update multiple transactions



## create_transaction

> models::SaveTransactionsResponse create_transaction(budget_id, data)
Create a single transaction or multiple transactions

Creates a single transaction or multiple transactions.  If you provide a body containing a `transaction` object, a single transaction will be created and if you provide a body containing a `transactions` array, multiple transactions will be created.  Scheduled transactions (transactions with a future date) cannot be created on this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**budget_id** | **String** | The id of the budget. \"last-used\" can be used to specify the last used budget and \"default\" can be used if default budget selection is enabled (see: https://api.ynab.com/#oauth-default-budget). | [required] |
**data** | [**PostTransactionsWrapper**](PostTransactionsWrapper.md) | The transaction or transactions to create.  To create a single transaction you can specify a value for the `transaction` object and to create multiple transactions you can specify an array of `transactions`.  It is expected that you will only provide a value for one of these objects. | [required] |

### Return type

[**models::SaveTransactionsResponse**](SaveTransactionsResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_transaction

> models::TransactionResponse delete_transaction(budget_id, transaction_id)
Deletes an existing transaction

Deletes a transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**budget_id** | **String** | The id of the budget. \"last-used\" can be used to specify the last used budget and \"default\" can be used if default budget selection is enabled (see: https://api.ynab.com/#oauth-default-budget). | [required] |
**transaction_id** | **String** | The id of the transaction | [required] |

### Return type

[**models::TransactionResponse**](TransactionResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transaction_by_id

> models::TransactionResponse get_transaction_by_id(budget_id, transaction_id)
Single transaction

Returns a single transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**budget_id** | **String** | The id of the budget. \"last-used\" can be used to specify the last used budget and \"default\" can be used if default budget selection is enabled (see: https://api.ynab.com/#oauth-default-budget). | [required] |
**transaction_id** | **String** | The id of the transaction | [required] |

### Return type

[**models::TransactionResponse**](TransactionResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transactions

> models::TransactionsResponse get_transactions(budget_id, since_date, r#type, last_knowledge_of_server)
List transactions

Returns budget transactions, excluding any pending transactions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**budget_id** | **String** | The id of the budget. \"last-used\" can be used to specify the last used budget and \"default\" can be used if default budget selection is enabled (see: https://api.ynab.com/#oauth-default-budget). | [required] |
**since_date** | Option<**String**> | If specified, only transactions on or after this date will be included.  The date should be ISO formatted (e.g. 2016-12-30). |  |
**r#type** | Option<**String**> | If specified, only transactions of the specified type will be included. \"uncategorized\" and \"unapproved\" are currently supported. |  |
**last_knowledge_of_server** | Option<**i64**> | The starting server knowledge.  If provided, only entities that have changed since `last_knowledge_of_server` will be included. |  |

### Return type

[**models::TransactionsResponse**](TransactionsResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transactions_by_account

> models::TransactionsResponse get_transactions_by_account(budget_id, account_id, since_date, r#type, last_knowledge_of_server)
List account transactions

Returns all transactions for a specified account, excluding any pending transactions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**budget_id** | **String** | The id of the budget. \"last-used\" can be used to specify the last used budget and \"default\" can be used if default budget selection is enabled (see: https://api.ynab.com/#oauth-default-budget). | [required] |
**account_id** | **String** | The id of the account | [required] |
**since_date** | Option<**String**> | If specified, only transactions on or after this date will be included.  The date should be ISO formatted (e.g. 2016-12-30). |  |
**r#type** | Option<**String**> | If specified, only transactions of the specified type will be included. \"uncategorized\" and \"unapproved\" are currently supported. |  |
**last_knowledge_of_server** | Option<**i64**> | The starting server knowledge.  If provided, only entities that have changed since `last_knowledge_of_server` will be included. |  |

### Return type

[**models::TransactionsResponse**](TransactionsResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transactions_by_category

> models::HybridTransactionsResponse get_transactions_by_category(budget_id, category_id, since_date, r#type, last_knowledge_of_server)
List category transactions, excluding any pending transactions

Returns all transactions for a specified category

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**budget_id** | **String** | The id of the budget. \"last-used\" can be used to specify the last used budget and \"default\" can be used if default budget selection is enabled (see: https://api.ynab.com/#oauth-default-budget). | [required] |
**category_id** | **String** | The id of the category | [required] |
**since_date** | Option<**String**> | If specified, only transactions on or after this date will be included.  The date should be ISO formatted (e.g. 2016-12-30). |  |
**r#type** | Option<**String**> | If specified, only transactions of the specified type will be included. \"uncategorized\" and \"unapproved\" are currently supported. |  |
**last_knowledge_of_server** | Option<**i64**> | The starting server knowledge.  If provided, only entities that have changed since `last_knowledge_of_server` will be included. |  |

### Return type

[**models::HybridTransactionsResponse**](HybridTransactionsResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transactions_by_month

> models::HybridTransactionsResponse get_transactions_by_month(budget_id, month, since_date, r#type, last_knowledge_of_server)
List transactions in month, excluding any pending transactions

Returns all transactions for a specified month

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**budget_id** | **String** | The id of the budget. \"last-used\" can be used to specify the last used budget and \"default\" can be used if default budget selection is enabled (see: https://api.ynab.com/#oauth-default-budget). | [required] |
**month** | **String** | The budget month in ISO format (e.g. 2016-12-01) (\"current\" can also be used to specify the current calendar month (UTC)) | [required] |
**since_date** | Option<**String**> | If specified, only transactions on or after this date will be included.  The date should be ISO formatted (e.g. 2016-12-30). |  |
**r#type** | Option<**String**> | If specified, only transactions of the specified type will be included. \"uncategorized\" and \"unapproved\" are currently supported. |  |
**last_knowledge_of_server** | Option<**i64**> | The starting server knowledge.  If provided, only entities that have changed since `last_knowledge_of_server` will be included. |  |

### Return type

[**models::HybridTransactionsResponse**](HybridTransactionsResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transactions_by_payee

> models::HybridTransactionsResponse get_transactions_by_payee(budget_id, payee_id, since_date, r#type, last_knowledge_of_server)
List payee transactions, excluding any pending transactions

Returns all transactions for a specified payee

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**budget_id** | **String** | The id of the budget. \"last-used\" can be used to specify the last used budget and \"default\" can be used if default budget selection is enabled (see: https://api.ynab.com/#oauth-default-budget). | [required] |
**payee_id** | **String** | The id of the payee | [required] |
**since_date** | Option<**String**> | If specified, only transactions on or after this date will be included.  The date should be ISO formatted (e.g. 2016-12-30). |  |
**r#type** | Option<**String**> | If specified, only transactions of the specified type will be included. \"uncategorized\" and \"unapproved\" are currently supported. |  |
**last_knowledge_of_server** | Option<**i64**> | The starting server knowledge.  If provided, only entities that have changed since `last_knowledge_of_server` will be included. |  |

### Return type

[**models::HybridTransactionsResponse**](HybridTransactionsResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## import_transactions

> models::TransactionsImportResponse import_transactions(budget_id)
Import transactions

Imports available transactions on all linked accounts for the given budget.  Linked accounts allow transactions to be imported directly from a specified financial institution and this endpoint initiates that import.  Sending a request to this endpoint is the equivalent of clicking \"Import\" on each account in the web application or tapping the \"New Transactions\" banner in the mobile applications.  The response for this endpoint contains the transaction ids that have been imported.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**budget_id** | **String** | The id of the budget. \"last-used\" can be used to specify the last used budget and \"default\" can be used if default budget selection is enabled (see: https://api.ynab.com/#oauth-default-budget). | [required] |

### Return type

[**models::TransactionsImportResponse**](TransactionsImportResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_transaction

> models::TransactionResponse update_transaction(budget_id, transaction_id, data)
Updates an existing transaction

Updates a single transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**budget_id** | **String** | The id of the budget. \"last-used\" can be used to specify the last used budget and \"default\" can be used if default budget selection is enabled (see: https://api.ynab.com/#oauth-default-budget). | [required] |
**transaction_id** | **String** | The id of the transaction | [required] |
**data** | [**PutTransactionWrapper**](PutTransactionWrapper.md) | The transaction to update | [required] |

### Return type

[**models::TransactionResponse**](TransactionResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_transactions

> models::SaveTransactionsResponse update_transactions(budget_id, data)
Update multiple transactions

Updates multiple transactions, by `id` or `import_id`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**budget_id** | **String** | The id of the budget. \"last-used\" can be used to specify the last used budget and \"default\" can be used if default budget selection is enabled (see: https://api.ynab.com/#oauth-default-budget). | [required] |
**data** | [**PatchTransactionsWrapper**](PatchTransactionsWrapper.md) | The transactions to update. Each transaction must have either an `id` or `import_id` specified. If `id` is specified as null an `import_id` value can be provided which will allow transaction(s) to be updated by its `import_id`. If an `id` is specified, it will always be used for lookup.  You should not specify both `id` and `import_id`.  Updating an `import_id` on an existing transaction is not allowed; if an `import_id` is specified, it will only be used to lookup the transaction. | [required] |

### Return type

[**models::SaveTransactionsResponse**](SaveTransactionsResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

