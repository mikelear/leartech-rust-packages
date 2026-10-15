# BlockchainDeployResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**chain** | Option<**String**> |  | [optional]
**chain_id** | Option<**i32**> |  | [optional]
**contract** | Option<**String**> |  | [optional]
**status** | Option<**String**> |  | [optional]
**transaction** | Option<[**models::BlockchainSubmitResult**](blockchain.SubmitResult.md)> |  | [optional]
**warning** | Option<**String**> | Warning carries a non-fatal post-broadcast problem (the deployment row did not land) so the 202 body can hold BOTH the hash the caller needs and the thing that went wrong around it. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


