# \ExampleApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**example**](ExampleApi.md#example) | **GET** /api/v1/example | Example endpoint — protected by `AuthLayer`, returns the caller's `user_id`.



## example

> models::ExampleResponse example()
Example endpoint — protected by `AuthLayer`, returns the caller's `user_id`.

The `Claims` extractor pulls the validated token claims out of request extensions where `AuthLayer` placed them. If the layer wasn't applied, the extractor returns 500 (operator misconfig — distinct from a client-side 401/403 the layer itself would have produced for invalid tokens).

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ExampleResponse**](ExampleResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

