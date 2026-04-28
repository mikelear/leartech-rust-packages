# \HealthApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**health_live_get**](HealthApi.md#health_live_get) | **GET** /health/live | Liveness probe
[**health_ready_get**](HealthApi.md#health_ready_get) | **GET** /health/ready | Readiness probe



## health_live_get

> std::collections::HashMap<String, String> health_live_get()
Liveness probe

### Parameters

This endpoint does not need any parameter.

### Return type

**std::collections::HashMap<String, String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## health_ready_get

> std::collections::HashMap<String, String> health_ready_get()
Readiness probe

### Parameters

This endpoint does not need any parameter.

### Return type

**std::collections::HashMap<String, String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

