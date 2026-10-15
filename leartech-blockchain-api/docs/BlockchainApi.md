# \BlockchainApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v1_chain_status_get**](BlockchainApi.md#api_v1_chain_status_get) | **GET** /api/v1/chain/status | Chain status
[**api_v1_contracts_address_read_post**](BlockchainApi.md#api_v1_contracts_address_read_post) | **POST** /api/v1/contracts/{address}/read | Call a view/pure method on a contract
[**api_v1_contracts_get**](BlockchainApi.md#api_v1_contracts_get) | **GET** /api/v1/contracts | List known contracts
[**api_v1_deployments_get**](BlockchainApi.md#api_v1_deployments_get) | **GET** /api/v1/deployments | Get contract deployments
[**api_v1_deployments_plan_post**](BlockchainApi.md#api_v1_deployments_plan_post) | **POST** /api/v1/deployments/plan | Plan a deployment (dry run — never broadcasts, needs no signer)
[**api_v1_deployments_post**](BlockchainApi.md#api_v1_deployments_post) | **POST** /api/v1/deployments | Deploy a contract (signs + broadcasts; returns pending, never blocks on mining)
[**api_v1_events_get**](BlockchainApi.md#api_v1_events_get) | **GET** /api/v1/events | Get contract events
[**api_v1_transactions_hash_get**](BlockchainApi.md#api_v1_transactions_hash_get) | **GET** /api/v1/transactions/{hash} | Get a transaction by hash
[**api_v1_transactions_post**](BlockchainApi.md#api_v1_transactions_post) | **POST** /api/v1/transactions | Submit a transaction (signs + broadcasts; returns pending, never blocks on mining)
[**api_v1_transactions_simulate_post**](BlockchainApi.md#api_v1_transactions_simulate_post) | **POST** /api/v1/transactions/simulate | Simulate a transaction (eth_call; nothing broadcast, no signer needed)



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


## api_v1_deployments_plan_post

> models::BlockchainPlanResult api_v1_deployments_plan_post(body)
Plan a deployment (dry run — never broadcasts, needs no signer)

Everything a review needs about a proposed deployment: the constructor signature, the ABI-encoded args, the exact calldata, a gas estimate and validation findings. Touches the chain only for an eth_estimateGas. No signer required, nothing broadcast.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**BlockchainPlanRequest**](BlockchainPlanRequest.md) | contract + args + bytecode | [required] |

### Return type

[**models::BlockchainPlanResult**](blockchain.PlanResult.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_deployments_post

> models::BlockchainDeployResult api_v1_deployments_post(body)
Deploy a contract (signs + broadcasts; returns pending, never blocks on mining)

Runs the plan, refuses on error-level findings, then signs and broadcasts the creation transaction and records a pending deployment row for the caller's tenant. Returns immediately with the transaction hash and status pending — get_transaction is how a caller follows up. Requires leartechapi:mcp:write and a funded signer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**BlockchainPlanRequest**](BlockchainPlanRequest.md) | contract + args + bytecode | [required] |

### Return type

[**models::BlockchainDeployResult**](blockchain.DeployResult.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
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


## api_v1_transactions_post

> models::BlockchainSubmitResult api_v1_transactions_post(body)
Submit a transaction (signs + broadcasts; returns pending, never blocks on mining)

Signs one transaction with the service key and broadcasts it, returning immediately with the hash and status pending. get_transaction is how a caller follows up. Requires leartechapi:mcp:write and a funded signer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**BlockchainSubmitRequest**](BlockchainSubmitRequest.md) | contract+method+args, or to+data+value | [required] |

### Return type

[**models::BlockchainSubmitResult**](blockchain.SubmitResult.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_transactions_simulate_post

> models::BlockchainSimulateResult api_v1_transactions_simulate_post(body)
Simulate a transaction (eth_call; nothing broadcast, no signer needed)

Runs an eth_call (with state override where the node accepts one) plus an eth_estimateGas against the latest state and returns the decoded result, the gas estimate and the revert reason if the call would revert. A revert is a RESULT here, not an error. Nothing is broadcast and no signer is required.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** | contract+method+args, or to+data | [required] |

### Return type

[**models::BlockchainSimulateResult**](blockchain.SimulateResult.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

