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

Discovery for this gateway: which authorization server governs it, the scopes it publishes, and the bare audience string its verifier requires.  DYNAMIC CLIENT REGISTRATION DOES NOT REACH THIS RESOURCE. A client that registers itself via RFC 7591 against the issuer named here cannot obtain a token this gateway accepts: the estate's DCR policy stamps a fixed audience allow-list that does not include this gateway, and its allowed-scope list contains no gateway scopes. That is deliberate, not a gap - the credentials this API issues are authority rather than data, so registering a client is itself the authorisation and wants a person on it. Use a pre-registered client.  The audience field is a deviation from RFC 9728, named as one: this estate issues tokens carrying a bare audience string rather than honouring RFC 8707 resource indicators, so the value a client actually needs is published here instead of being guessed from a 401.

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

