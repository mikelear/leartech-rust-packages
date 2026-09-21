# \ShellApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v1_shell_prompt_get**](ShellApi.md#api_v1_shell_prompt_get) | **GET** /api/v1/shell/prompt | The system prompt for the terminal client



## api_v1_shell_prompt_get

> models::HandlersShellPrompt api_v1_shell_prompt_get()
The system prompt for the terminal client

Served from configuration so a prompt change is a config change rather than a release. The revision is a content hash: it identifies the exact text, so a recorded session can be tied to the guidance that produced it.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HandlersShellPrompt**](handlers.ShellPrompt.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

