# \CapabilitiesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v1_capabilities_languages_get**](CapabilitiesApi.md#api_v1_capabilities_languages_get) | **GET** /api/v1/capabilities/languages | List supported languages
[**api_v1_capabilities_languages_name_get**](CapabilitiesApi.md#api_v1_capabilities_languages_name_get) | **GET** /api/v1/capabilities/languages/{name} | Resolve one language
[**api_v1_capabilities_promote_post**](CapabilitiesApi.md#api_v1_capabilities_promote_post) | **POST** /api/v1/capabilities/promote | Promote a researched capability
[**api_v1_capabilities_stores_get**](CapabilitiesApi.md#api_v1_capabilities_stores_get) | **GET** /api/v1/capabilities/stores | List supported stores
[**api_v1_capabilities_stores_name_get**](CapabilitiesApi.md#api_v1_capabilities_stores_name_get) | **GET** /api/v1/capabilities/stores/{name} | Resolve one store



## api_v1_capabilities_languages_get

> std::collections::HashMap<String, serde_json::Value> api_v1_capabilities_languages_get()
List supported languages

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


## api_v1_capabilities_languages_name_get

> std::collections::HashMap<String, serde_json::Value> api_v1_capabilities_languages_name_get(name)
Resolve one language

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | capability name to resolve | [required] |

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_capabilities_promote_post

> models::CapabilitiesPromoteResult api_v1_capabilities_promote_post()
Promote a researched capability

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::CapabilitiesPromoteResult**](capabilities.PromoteResult.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_capabilities_stores_get

> std::collections::HashMap<String, serde_json::Value> api_v1_capabilities_stores_get()
List supported stores

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


## api_v1_capabilities_stores_name_get

> std::collections::HashMap<String, serde_json::Value> api_v1_capabilities_stores_name_get(name)
Resolve one store

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | capability name to resolve | [required] |

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

