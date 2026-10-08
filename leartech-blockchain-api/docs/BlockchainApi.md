# \BlockchainApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v1_chain_status_get**](BlockchainApi.md#api_v1_chain_status_get) | **GET** /api/v1/chain/status | Chain status
[**api_v1_contracts_address_read_post**](BlockchainApi.md#api_v1_contracts_address_read_post) | **POST** /api/v1/contracts/{address}/read | Call a view/pure method on a contract
[**api_v1_contracts_get**](BlockchainApi.md#api_v1_contracts_get) | **GET** /api/v1/contracts | List known contracts
[**api_v1_deployments_get**](BlockchainApi.md#api_v1_deployments_get) | **GET** /api/v1/deployments | Get contract deployments
[**api_v1_events_get**](BlockchainApi.md#api_v1_events_get) | **GET** /api/v1/events | Get contract events
[**api_v1_transactions_hash_get**](BlockchainApi.md#api_v1_transactions_hash_get) | **GET** /api/v1/transactions/{hash} | Get a transaction by hash



## api_v1_chain_status_get

> models::BlockchainChainStatus api_v1_chain_status_get()
Chain status

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::BlockchainChainStatus**](blockchain.ChainStatus.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_contracts_address_read_post

> models::BlockchainReadResult api_v1_contracts_address_read_post(address, body)
Call a view/pure method on a contract

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | deployed contract address | [required] |
**body** | [**HandlersReadContractRequest**](HandlersReadContractRequest.md) | method + args | [required] |

### Return type

[**models::BlockchainReadResult**](blockchain.ReadResult.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_contracts_get

> models::HandlersListResponse api_v1_contracts_get(name, kind)
List known contracts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | filter by contract name |  |
**kind** | Option<**String**> | filter by contract kind (e.g. oracle, consumer) |  |

### Return type

[**models::HandlersListResponse**](handlers.listResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_deployments_get

> models::HandlersListResponse api_v1_deployments_get(contract, chain, address)
Get contract deployments

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract** | Option<**String**> | filter by contract name |  |
**chain** | Option<**String**> | filter by chain |  |
**address** | Option<**String**> | filter by deployed address |  |

### Return type

[**models::HandlersListResponse**](handlers.listResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_events_get

> models::HandlersListResponse api_v1_events_get(address, event, from_block, to_block, tx_hash)
Get contract events

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | Option<**String**> | filter by contract address |  |
**event** | Option<**String**> | filter by event name |  |
**from_block** | Option<**i32**> | first block (inclusive) |  |
**to_block** | Option<**i32**> | last block (inclusive) |  |
**tx_hash** | Option<**String**> | filter by transaction hash |  |

### Return type

[**models::HandlersListResponse**](handlers.listResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_transactions_hash_get

> models::BlockchainTransaction api_v1_transactions_hash_get(hash)
Get a transaction by hash

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hash** | **String** | transaction hash | [required] |

### Return type

[**models::BlockchainTransaction**](blockchain.Transaction.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

