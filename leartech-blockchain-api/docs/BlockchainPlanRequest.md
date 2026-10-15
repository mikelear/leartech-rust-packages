# BlockchainPlanRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**args** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | Args are the constructor arguments, in ABI order. | [optional]
**bytecode** | Option<**String**> | Bytecode is the contract's creation code, 0x-hex. When empty the plan reports the finding and does not estimate gas — deploying a transaction with no init code is a caller error we surface, not one we paper over. | [optional]
**chain** | Option<**String**> | Chain is a label for the deployment row; the chainId comes from the node the service is wired to. | [optional]
**contract** | **String** | Contract is the catalog name whose ABI defines the constructor — \"MockOracle\" / \"MockConsumer\" today. | 
**from** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


