# ApiTransformReport

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**after_tokens** | Option<**i32**> |  | [optional]
**before_tokens** | Option<**i32**> | BeforeTokens and AfterTokens are the gateway's own estimate (chars/4) either side, the same arithmetic the context-window gate enforces with. | [optional]
**bridge_added** | Option<**bool**> | BridgeAdded is whether a system \"you are continuing a conversation begun with another model\" note was inserted, so the new model understands it is resuming rather than starting. | [optional]
**compacted** | Option<**i32**> | Compacted is how many tool results were elided to fit the target window, 0 when the session already fit. | [optional]
**fit** | Option<**bool**> | Fit is whether the session reached the target window with reply headroom. proven-by: TestTransformForModel_SaysWhenItCannotFit | [optional]
**notes** | Option<**Vec<String>**> | Notes are human-readable lines describing each change and its reason. | [optional]
**target_max_ctx** | Option<**i32**> | TargetMaxCtx is the target model's advertised context window, the ceiling the transform fit the session under. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


