# \ChatApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_chat_completions_post**](ChatApi.md#v1_chat_completions_post) | **POST** /v1/chat/completions | OpenAI-compatible chat completion
[**v1_messages_post**](ChatApi.md#v1_messages_post) | **POST** /v1/messages | Anthropic Messages passthrough



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


## v1_messages_post

> std::collections::HashMap<String, serde_json::Value> v1_messages_post()
Anthropic Messages passthrough

### Parameters

This endpoint does not need any parameter.

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

