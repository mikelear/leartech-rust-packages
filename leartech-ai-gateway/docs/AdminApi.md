# \AdminApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admin_v1_keys_get**](AdminApi.md#admin_v1_keys_get) | **GET** /admin/v1/keys | List virtual keys for the caller's tenant
[**admin_v1_keys_keyid_delete**](AdminApi.md#admin_v1_keys_keyid_delete) | **DELETE** /admin/v1/keys/{keyid} | Revoke a virtual key (soft; never deleted)
[**admin_v1_keys_keyid_rotate_post**](AdminApi.md#admin_v1_keys_keyid_rotate_post) | **POST** /admin/v1/keys/{keyid}/rotate | Rotate a key's secret (returned once)
[**admin_v1_keys_post**](AdminApi.md#admin_v1_keys_post) | **POST** /admin/v1/keys | Mint a virtual key (secret returned once)
[**admin_v1_usage_get**](AdminApi.md#admin_v1_usage_get) | **GET** /admin/v1/usage | Usage and spend for the caller's tenant this month



## admin_v1_keys_get

> models::ApiListKeysResponse admin_v1_keys_get()
List virtual keys for the caller's tenant

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiListKeysResponse**](api.ListKeysResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_v1_keys_keyid_delete

> admin_v1_keys_keyid_delete(keyid)
Revoke a virtual key (soft; never deleted)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**keyid** | **String** | key id | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_v1_keys_keyid_rotate_post

> models::ApiCreateKeyResponse admin_v1_keys_keyid_rotate_post(keyid)
Rotate a key's secret (returned once)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**keyid** | **String** | key id | [required] |

### Return type

[**models::ApiCreateKeyResponse**](api.CreateKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_v1_keys_post

> models::ApiCreateKeyResponse admin_v1_keys_post(request)
Mint a virtual key (secret returned once)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request** | [**ApiCreateKeyRequest**](ApiCreateKeyRequest.md) | key policy | [required] |

### Return type

[**models::ApiCreateKeyResponse**](api.CreateKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_v1_usage_get

> models::ApiUsageResponse admin_v1_usage_get()
Usage and spend for the caller's tenant this month

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiUsageResponse**](api.UsageResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

