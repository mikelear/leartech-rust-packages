# \EventRegistrationApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_event_registration_information**](EventRegistrationApi.md#get_event_registration_information) | **GET** /api/event_registration/event_info | Returns information about event registrations
[**get_event_registration_information_for_name**](EventRegistrationApi.md#get_event_registration_information_for_name) | **GET** /api/event_registration/event_info/{eventName} | Returns information about a given event registrations



## get_event_registration_information

> models::GetEventRegistrationInfoResponse get_event_registration_information()
Returns information about event registrations

Returns information about event registrations

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetEventRegistrationInfoResponse**](GetEventRegistrationInfoResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_registration_information_for_name

> models::GetEventRegistrationInfoForNameResponse get_event_registration_information_for_name(event_name)
Returns information about a given event registrations

Returns information about a given event registrations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_name** | **String** | the name of the event to get information for | [required] |

### Return type

[**models::GetEventRegistrationInfoForNameResponse**](GetEventRegistrationInfoForNameResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

