# ApiModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**hosting** | Option<**String**> |  | [optional]
**id** | Option<**String**> |  | [optional]
**interface** | Option<**String**> | Provider is the supplier that answers (anthropic, deepseek, ollama, azure-openai, litellm); ProviderModel is the concrete model it serves.  The catalog is three levels -- supplier, logical alias, concrete model -- and this response published only the middle one. A caller could not tell that \"claude\" means claude-opus-4-8 via anthropic, nor that glm/codestral/mistral-large are one LiteLLM supplier rather than three. owned_by was the only hint and it is the constant \"leartech\" for every row, so it distinguished nothing.  The reviewer already logs provider + model_served per call, so the distinction existed everywhere except here.  RENAMED FROM `provider` IN 00023. It holds the ADAPTER -- how we reach the model -- and calling that the provider is the conflation 00023 removes: glm, codestral and qwen-via-litellm all answer \"litellm\" here and are z.ai, Mistral and our own Ollama. `provider` now means whose model it is, below.  source: model_catalog(logical_model, provider_model, adapter) -- migration 00001 | [optional]
**max_ctx** | Option<**i32**> | Capabilities/limits so callers can cap what they can't otherwise see (INTERFACES.md §4 \"degrade visibly, never silently\"). max_ctx is the model's context window; vision reports image-input support. | [optional]
**object** | Option<**String**> |  | [optional]
**owned_by** | Option<**String**> |  | [optional]
**provider** | Option<**String**> | Provider is WHOSE model it is; Hosting is where the weights run.  Empty when the row is unseeded, so a client can tell \"unknown\" from \"leartech\" rather than defaulting a third party to us. proven-by: TestProvenance_TheLiteLLMModelsAreNotOneSupplier | [optional]
**provider_model** | Option<**String**> |  | [optional]
**surfaces** | Option<**Vec<String>**> | Surfaces is what the model is served AS. Per-MODEL, not per-interface: qwen-embedding is embeddings-only behind the same fireworks interface that serves chat models, and a caller choosing a CHAT model needs to filter it out — which is what 00029 exists for.  omitempty: an older gateway sends nothing and a client reads that as \"unreported\", not \"serves nothing\" — same rule as provider/hosting.  proven-by: TestModels_PublishesTheSurfacesEachModelServes | [optional]
**vision** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


