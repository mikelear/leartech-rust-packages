# HandlersShellPrompt

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**label** | Option<**String**> | Label is the human name for this prompt, if the cluster set one. Free text for a reader; Revision is what identifies it. | [optional]
**prompt** | Option<**String**> | Prompt is the system prompt, or empty when this cluster sets none. | [optional]
**revision** | Option<**String**> | Revision identifies this exact text.  A content hash, not a hand-set number. // proven-by: TestShellPrompt_RevisionIsDerivedFromTheContent A version someone has to remember to bump goes stale quietly, and two different prompts claiming one revision make every session recorded against it unreadable. Derived from the bytes, so it agrees with what was served.  proven-by: TestShellPrompt_RevisionIsDerivedFromTheContent proven-by: TestShellPrompt_DifferentPromptsNeverShareARevision | [optional]
**source** | Option<**String**> | Source names where the value came from, for a client that has to tell an operator which prompt it is running. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


