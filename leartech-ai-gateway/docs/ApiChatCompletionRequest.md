# ApiChatCompletionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**max_tokens** | Option<**i32**> |  | [optional]
**messages** | [**Vec<models::ApiRequestMessage>**](api.RequestMessage.md) |  | 
**model** | **String** |  | 
**stream** | Option<**bool**> |  | [optional]
**stream_options** | Option<[**models::ApiStreamOptions**](api.StreamOptions.md)> | StreamOptions.IncludeUsage asks for a final chunk carrying the token and cache breakdown, the same shape OpenAI emits and the same one openai.go already sends UPSTREAM and parses back.  The gateway received usage on every streamed call, billed with it, and dropped it before the client -- so a streamed turn was the least visible traffic on the system while being the highest volume an agent loop produces. Found by the CLI session building the first streaming consumer. | [optional]
**temperature** | Option<**f64**> |  | [optional]
**tool_choice** | Option<[**serde_json::Value**](.md)> |  | [optional]
**tools** | Option<[**serde_json::Value**](.md)> | S7b passthrough: forwarded verbatim to OpenAI-compatible providers. | [optional]
**x_leartech** | Option<[**models::ApiLeartechExt**](api.LeartechExt.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


