# ApiTransformSessionResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**checkpoint** | Option<[**serde_json::Value**](.md)> | Checkpoint is the rewritten session, ready for the shell to save to a new resume slot and continue on TargetModel. It re-chains onto the source (prev_hash = source hash), exactly as compact does, so the switch is one recorded link rather than a hidden edit. | [optional]
**object** | Option<**String**> | \"session.transform\" | [optional]
**report** | Option<[**models::ApiTransformReport**](api.TransformReport.md)> | Report names every change, so the switch is auditable. | [optional]
**target_model** | Option<**String**> | TargetModel echoes what the session was transformed FOR. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


