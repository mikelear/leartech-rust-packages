# MaestroConsumeEventResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**error_reason** | Option<**String**> | ErrorReason is a human-readable description when IsErrored is true or when the event was dropped for a non-error reason (unknown event, malformed body). | [optional]
**is_consumed** | Option<**bool**> | IsConsumed is true when the event was successfully handled. Maestro treats IsConsumed=true (regardless of HTTP status) as \"settled, do not retry\". | [optional]
**is_errored** | Option<**bool**> | IsErrored is true when handler execution failed. Distinct from !IsConsumed so we can signal \"no handler registered\" (neither consumed nor errored) vs. \"handler ran and blew up\" (errored). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


