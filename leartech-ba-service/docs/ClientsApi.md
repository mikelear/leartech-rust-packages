# \ClientsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**clients_get**](ClientsApi.md#clients_get) | **GET** /clients | List the terminal clients this build carries
[**clients_name_get**](ClientsApi.md#clients_name_get) | **GET** /clients/{name} | Download one terminal client



## clients_get

> models::HandlersClientsResponse clients_get()
List the terminal clients this build carries

Cross-compiled leartech clients, built from the same commit as this service. The response reports the version served and a SHA256 for each binary.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HandlersClientsResponse**](handlers.ClientsResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clients_name_get

> std::path::PathBuf clients_name_get(name)
Download one terminal client

Serves the named binary. The name is matched against the directory listing, so only what exists can be requested. X-Leartech-Version reports the build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | client filename, e.g. leartech-darwin-arm64 | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

