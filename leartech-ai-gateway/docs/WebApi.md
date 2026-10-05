# \WebApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_fetch_post**](WebApi.md#v1_fetch_post) | **POST** /v1/fetch | URL fetch (gated on the web_fetch scope)
[**v1_search_post**](WebApi.md#v1_search_post) | **POST** /v1/search | Web search (gated on the web_search scope)
[**v1_websearch_get**](WebApi.md#v1_websearch_get) | **GET** /v1/websearch | List web-search providers



## v1_fetch_post

> models::WebfetchResult v1_fetch_post()
URL fetch (gated on the web_fetch scope)

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::WebfetchResult**](webfetch.Result.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_search_post

> models::WebsearchResults v1_search_post()
Web search (gated on the web_search scope)

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::WebsearchResults**](websearch.Results.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_websearch_get

> models::ApiWebSearchResponse v1_websearch_get()
List web-search providers

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiWebSearchResponse**](api.WebSearchResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

