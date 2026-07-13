# \ChatApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_chat_completions_post**](ChatApi.md#v1_chat_completions_post) | **POST** /v1/chat/completions | OpenAI-compatible chat completion



## v1_chat_completions_post

> models::ApiChatCompletionResponse v1_chat_completions_post(request)
OpenAI-compatible chat completion

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request** | [**ApiChatCompletionRequest**](ApiChatCompletionRequest.md) | chat request | [required] |

### Return type

[**models::ApiChatCompletionResponse**](api.ChatCompletionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

