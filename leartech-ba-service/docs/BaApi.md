# \BaApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v1_ba_last_get**](BaApi.md#api_v1_ba_last_get) | **GET** /api/v1/ba/last | The last BA pass
[**api_v1_ba_tick_post**](BaApi.md#api_v1_ba_tick_post) | **POST** /api/v1/ba/tick | Run one BA loop pass



## api_v1_ba_last_get

> models::HandlersBaPass api_v1_ba_last_get()
The last BA pass

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HandlersBaPass**](handlers.BAPass.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_ba_tick_post

> models::HandlersBaPass api_v1_ba_tick_post()
Run one BA loop pass

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HandlersBaPass**](handlers.BAPass.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

