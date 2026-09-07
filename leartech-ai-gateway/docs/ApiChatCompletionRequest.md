# ApiChatCompletionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**max_tokens** | Option<**i32**> |  | [optional]
**messages** | [**Vec<models::ApiRequestMessage>**](api.RequestMessage.md) |  | 
**model** | **String** |  | 
**stream** | Option<**bool**> |  | [optional]
**temperature** | Option<**f64**> |  | [optional]
**tool_choice** | Option<[**serde_json::Value**](.md)> |  | [optional]
**tools** | Option<[**serde_json::Value**](.md)> | S7b passthrough: forwarded verbatim to OpenAI-compatible providers. | [optional]
**x_leartech** | Option<[**models::ApiLeartechExt**](api.LeartechExt.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


