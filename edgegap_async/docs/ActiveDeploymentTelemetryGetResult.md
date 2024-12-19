# ActiveDeploymentTelemetryGetResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**retrieval_key** | **String** | Unique retrieval key to get the telemetry response. | 
**scores** | [**Vec<models::ActiveDeploymentTelemetryScore>**](ActiveDeploymentTelemetryScore.md) | Result sorted by best score. Index 0 is the best one. | 
**partial_result** | **bool** | If the score list is incomplete and missing request IDs. Can occur if you request the results before we receive telemetry from every deployment. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


