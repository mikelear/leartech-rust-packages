# ApiTool

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**available** | Option<**bool**> | Available reports whether THIS credential may call it. Unavailable tools are still listed, deliberately: a shortened list makes \"why can this shell not search\" unanswerable without reading scopes by hand.  The client shows the full list and offers only the available ones to the model, so display and capability stay different lists. Offering an unavailable tool would have the model plan around a capability it does not have and then 403. | [optional]
**function** | Option<[**models::ApiToolFunction**](api.ToolFunction.md)> |  | [optional]
**scope** | Option<**String**> | Scope names what a caller would need, so \"unavailable\" is actionable rather than just a closed door. | [optional]
**r#type** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


