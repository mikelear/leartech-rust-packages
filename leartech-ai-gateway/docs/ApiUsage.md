# ApiUsage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**completion_tokens** | Option<**i32**> |  | [optional]
**leartech_cache** | Option<[**models::ApiLeartechCache**](api.LeartechCache.md)> |  | [optional]
**prompt_tokens** | Option<**i32**> |  | [optional]
**prompt_tokens_details** | Option<[**models::ApiPromptTokensDetails**](api.PromptTokensDetails.md)> | ONE CONVENTION IN THIS OBJECT: every detail field below is a SUBSET of PromptTokens, which is every prompt token processed. Reads and writes are disjoint subsets — a prefix is either served from cache or written to it, not both in one request.  Subset rather than additive, and the reason is a client we did not write. A naive OpenAI client reads prompt_tokens, ignores every detail field and multiplies by the input rate: under subset that OVER-estimates, because the cached portion actually bills at a fraction; under additive it UNDER-estimates by two orders of magnitude, because the written tokens carry a premium and are invisible. Erring safe for the clients we control least follows the same principle as absence-resolving-to-the-input-rate.  It also keeps total_tokens honest: because PromptTokens is already the whole, total = prompt + completion is simultaneously OpenAI-pure and reconcilable against the cost on the same response. Under an additive extension those two are in conflict and something has to give.  proven-by: TestUsageResponse_IsLosslessAcrossTheConventionFlip proven-by: TestUsageResponse_TotalTokensStaysOpenAIPure | [optional]
**total_tokens** | Option<**i32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


