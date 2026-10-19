# \SessionsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_sessions_transform_post**](SessionsApi.md#v1_sessions_transform_post) | **POST** /v1/sessions/transform | Rewrite a saved session to resume it on a different model



## v1_sessions_transform_post

> models::ApiTransformSessionResponse v1_sessions_transform_post(request)
Rewrite a saved session to resume it on a different model

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request** | [**ApiTransformSessionRequest**](ApiTransformSessionRequest.md) | the session and the target model | [required] |

### Return type

[**models::ApiTransformSessionResponse**](api.TransformSessionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

