# MaestroConsumeEventRequestDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**actioned_by** | Option<**String**> | ActionedBy is the userId (or system-actor id) from the announcement, when present. | [optional]
**annotations** | Option<**std::collections::HashMap<String, String>**> | Annotations are the string k/v pairs from the announcement. Same vocabulary as pkg/maestro.Event.Annotations on the producer side. | [optional]
**id** | Option<**String**> | ID is the unique event id Maestro assigned at announce time. | [optional]
**name** | Option<**String**> | Name is the dotted event name — the dispatch key. | [optional]
**produced_time** | Option<**String**> | ProducedTime is the RFC3339 timestamp of the original announcement. Maestro emits an RFC3339 string; we decode with time.Time so downstream handlers get a native value. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


