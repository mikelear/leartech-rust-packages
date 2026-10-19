# ApiTransformSessionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**checkpoint** | [**serde_json::Value**](.md) | Checkpoint is the saved session, exactly as the shell wrote it — the agentloop.Checkpoint shape (version, turn, prev_hash, hash, messages). | 
**reply_headroom_tokens** | Option<**i32**> | ReplyHeadroomTokens is how much of the target window to leave free for the next reply. OPTIONAL; a sensible default is applied. Mirrors the shell's compact headroom so the two paths size the same way. | [optional]
**source_model** | Option<**String**> | SourceModel is the model the session was recorded against, when the caller knows it. OPTIONAL: it is used only to describe the switch in the bridge note and the response; the transform does not depend on it, since the rewrite is driven by the TARGET model's needs. | [optional]
**target_model** | **String** | TargetModel is the logical model the session should be resumed on. It is resolved against the caller's catalogue, so an unknown or disallowed model is refused the same way a chat call would be. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


