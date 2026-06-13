# \EventMaestroApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**announce_event**](EventMaestroApi.md#announce_event) | **POST** /api/event_maestro/announce_event | Announces an event for a case to maestro
[**consume_event**](EventMaestroApi.md#consume_event) | **POST** /api/event_maestro/consume_event | Consumes an event for a case from the maestro
[**get_events_by_annotation**](EventMaestroApi.md#get_events_by_annotation) | **GET** /api/event_maestro/events_by_annotation/{annotation_key}/{annotation_value} | Retrieves events by annotation
[**reprocess_event**](EventMaestroApi.md#reprocess_event) | **PUT** /api/event_maestro/reprocess_event/{event_id} | Reprocesses an event from the maestro events log
[**reprocess_event_for_consumer**](EventMaestroApi.md#reprocess_event_for_consumer) | **PUT** /api/event_maestro/reprocess_event_for_consumer/{event_id}/{consumer_name} | Reprocesses an event for a specific consumer



## announce_event

> models::AnnounceEventResponse announce_event(announce_request)
Announces an event for a case to maestro

Announces an event for a case to maestro

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**announce_request** | [**AnnounceEventRequest**](AnnounceEventRequest.md) | Event to announce | [required] |

### Return type

[**models::AnnounceEventResponse**](AnnounceEventResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## consume_event

> models::ConsumeEventResponse consume_event(consume_request)
Consumes an event for a case from the maestro

This is a place-holder for consistent package generation, and as an example of the consume endpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consume_request** | [**ConsumeEventRequest**](ConsumeEventRequest.md) | Event to consume | [required] |

### Return type

[**models::ConsumeEventResponse**](ConsumeEventResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_events_by_annotation

> models::GetEventsDtoResponse get_events_by_annotation(annotation_key, annotation_value)
Retrieves events by annotation

Retrieves events by annotation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**annotation_key** | **String** | Annotation Key | [required] |
**annotation_value** | **String** | Annotation Value | [required] |

### Return type

[**models::GetEventsDtoResponse**](GetEventsDtoResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reprocess_event

> models::Response reprocess_event(event_id, only_failures)
Reprocesses an event from the maestro events log

Reprocesses an event from the maestro events log, usually for failed events

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_id** | **String** | Event ID | [required] |
**only_failures** | Option<**bool**> | Only failures |  |

### Return type

[**models::Response**](Response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reprocess_event_for_consumer

> models::Response reprocess_event_for_consumer(event_id, consumer_name)
Reprocesses an event for a specific consumer

Reprocesses an event for a specific consumer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_id** | **String** | Event ID | [required] |
**consumer_name** | **String** | Consumer Name | [required] |

### Return type

[**models::Response**](Response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

