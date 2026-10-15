# BlockchainPlanResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**abi** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**args** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**bytecode** | Option<**String**> |  | [optional]
**calldata** | Option<**String**> |  | [optional]
**chain** | Option<**String**> |  | [optional]
**chain_id** | Option<**i32**> |  | [optional]
**constructor** | Option<**String**> |  | [optional]
**contract** | Option<**String**> |  | [optional]
**dry_run** | Option<**bool**> |  | [optional]
**encoded_args** | Option<**String**> |  | [optional]
**findings** | Option<[**Vec<models::BlockchainFinding>**](blockchain.Finding.md)> | Findings carries the plan's validation results, serialised with no omitempty so a plan with nothing to flag still answers \"findings\": []. A reviewer diffing two plans reads the field's absence as \"the service predates findings\", distinct from an empty list meaning \"nothing flagged\". proven-by: TestPlan_FindingsIsAlwaysAnArray | [optional]
**gas_estimate** | Option<**i32**> |  | [optional]
**gas_source** | Option<**String**> |  | [optional]
**signer** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


