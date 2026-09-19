# ApiToolFunction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | Description is what the model chooses on AND what a user reads at approval time, so it is written for both. | [optional]
**name** | Option<**String**> |  | [optional]
**parameters** | Option<[**serde_json::Value**](.md)> | Parameters is a JSON Schema, kept RAW for the same reason ToolCall.Arguments is: the gateway publishes it without interpreting it, so a schema keyword we do not model survives translation. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


