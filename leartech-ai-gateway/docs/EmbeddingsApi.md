# \EmbeddingsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_embeddings_post**](EmbeddingsApi.md#v1_embeddings_post) | **POST** /v1/embeddings | Embeddings (OpenAI-shaped)



## v1_embeddings_post

> models::ApiEmbeddingsResponse v1_embeddings_post(request)
Embeddings (OpenAI-shaped)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request** | **serde_json::Value** | embeddings request | [required] |

### Return type

[**models::ApiEmbeddingsResponse**](api.EmbeddingsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

