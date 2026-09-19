# \MetaApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**version_get**](MetaApi.md#version_get) | **GET** /version | Gateway version and wire-contract level
[**well_known_oauth_protected_resource_get**](MetaApi.md#well_known_oauth_protected_resource_get) | **GET** /.well-known/oauth-protected-resource | OAuth protected-resource metadata (RFC 9728)



## version_get

> models::ApiVersionResponse version_get()
Gateway version and wire-contract level

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiVersionResponse**](api.VersionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## well_known_oauth_protected_resource_get

> models::ApiProtectedResourceMetadata well_known_oauth_protected_resource_get()
OAuth protected-resource metadata (RFC 9728)

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiProtectedResourceMetadata**](api.ProtectedResourceMetadata.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

