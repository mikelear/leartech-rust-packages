# StoreUsageRow

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cache_read_tokens** | Option<**i32**> |  | [optional]
**cache_write_1h_tokens** | Option<**i32**> |  | [optional]
**cache_write_5m_tokens** | Option<**i32**> |  | [optional]
**cacheable_calls** | Option<**i32**> | CacheableCalls is how many of Calls were served by a supplier that reports a cache at all.  THE DENOMINATOR. Without it a hit rate is reads over ALL calls, which counts traffic to suppliers with no prompt cache as traffic that failed to hit one -- so a self-hosted model reads as a broken cache rather than an absent one. Measured 2026-09-18: glm returns no cache information whatsoever, claude returns 2688 cached tokens; in a sum those are indistinguishable without this. | [optional]
**calls** | Option<**i32**> |  | [optional]
**completion_tokens** | Option<**i32**> |  | [optional]
**cost_micros** | Option<**i32**> |  | [optional]
**keyid** | Option<**String**> |  | [optional]
**model** | Option<**String**> |  | [optional]
**prompt_tokens** | Option<**i32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


