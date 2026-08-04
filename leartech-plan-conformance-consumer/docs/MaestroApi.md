# \MaestroApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**confirmed_get**](MaestroApi.md#confirmed_get) | **GET** /confirmed | Check whether an event has been consumed
[**event_post**](MaestroApi.md#event_post) | **POST** /consume_event | Maestro event consumer endpoint
[**events_get**](MaestroApi.md#events_get) | **GET** /events | Debug dump of all recorded events



## confirmed_get

> std::collections::HashMap<String, serde_json::Value> confirmed_get(name)
Check whether an event has been consumed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Event name to check for (e.g. test.release.deploy_failed) | [required] |

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## event_post

> models::MaestroConsumeEventResponse event_post(body)
Maestro event consumer endpoint

Accept a Maestro ConsumeEventRequest body, record it, log a single structured JSON line, return the standard ConsumeEventResponse. Malformed body → 400 + isErrored.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**MaestroConsumeEventRequestDto**](MaestroConsumeEventRequestDto.md) | The event Maestro is delivering | [required] |

### Return type

[**models::MaestroConsumeEventResponse**](maestro.ConsumeEventResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_get

> std::collections::HashMap<String, serde_json::Value> events_get()
Debug dump of all recorded events

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

