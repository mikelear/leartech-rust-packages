# ApiPricingModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**hosting** | Option<**String**> |  | [optional]
**id** | Option<**String**> |  | [optional]
**interface** | Option<**String**> |  | [optional]
**provider** | Option<**String**> |  | [optional]
**rates** | Option<**std::collections::HashMap<String, i64>**> | Rates is micros per 1000 tokens, keyed by kind (input, output, cache_read, cache_write_5m, cache_write_1h). A kind ABSENT from this map has no rate on file; it is not zero. The distinction is the point of the endpoint — see Unpriced. | [optional]
**unpriced** | Option<**bool**> | Unpriced reports that this model is callable and has NO input rate.  THIS IS THE FIELD THE ENDPOINT EXISTS FOR. A missing rate resolves to zero in the router's cost ranking, so an unpriced model reads as FREE and `auto` prefers it over every real supplier. Until now the only symptom was traffic silently moving. Reporting it as a boolean rather than as a zero in Rates keeps \"costs nothing\" and \"nobody set a price\" distinguishable, which is exactly the conflation that makes the defect invisible.  proven-by: TestPricing_AnUnpricedModelIsFlagged_NotReportedAsFree | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


