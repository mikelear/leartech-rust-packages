# ApiModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**max_ctx** | Option<**i32**> | Capabilities/limits so callers can cap what they can't otherwise see (INTERFACES.md §4 \"degrade visibly, never silently\"). max_ctx is the model's context window; vision reports image-input support. | [optional]
**object** | Option<**String**> |  | [optional]
**owned_by** | Option<**String**> |  | [optional]
**provider** | Option<**String**> | Provider is the supplier that answers (anthropic, deepseek, ollama, azure-openai, litellm); ProviderModel is the concrete model it serves.  The catalog is three levels -- supplier, logical alias, concrete model -- and this response published only the middle one. A caller could not tell that \"claude\" means claude-opus-4-8 via anthropic, nor that glm/codestral/mistral-large are one LiteLLM supplier rather than three. owned_by was the only hint and it is the constant \"leartech\" for every row, so it distinguished nothing.  The reviewer already logs provider + model_served per call, so the distinction existed everywhere except here.  source: model_catalog(logical_model, provider_model, adapter) -- migration 00001 | [optional]
**provider_model** | Option<**String**> |  | [optional]
**vision** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


