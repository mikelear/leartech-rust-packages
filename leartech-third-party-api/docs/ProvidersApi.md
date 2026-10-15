# \ProvidersApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v1_third_parties_get**](ProvidersApi.md#api_v1_third_parties_get) | **GET** /api/v1/third-parties | List known third-party providers
[**api_v1_third_parties_name_get**](ProvidersApi.md#api_v1_third_parties_name_get) | **GET** /api/v1/third-parties/{name} | Resolve one provider
[**api_v1_third_parties_promote_post**](ProvidersApi.md#api_v1_third_parties_promote_post) | **POST** /api/v1/third-parties/promote | Promote a researched finding



## api_v1_third_parties_get

> std::collections::HashMap<String, serde_json::Value> api_v1_third_parties_get()
List known third-party providers

### Parameters

This endpoint does not need any parameter.

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_third_parties_name_get

> std::collections::HashMap<String, serde_json::Value> api_v1_third_parties_name_get(name)
Resolve one provider

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | provider name to resolve | [required] |

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_third_parties_promote_post

> models::ProvidersPromoteResult api_v1_third_parties_promote_post()
Promote a researched finding

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ProvidersPromoteResult**](providers.PromoteResult.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

