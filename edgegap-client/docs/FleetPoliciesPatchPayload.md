# FleetPoliciesPatchPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the Policy | [optional]
**enabled** | Option<**bool**> | If the Policy is enabled. Defaults to true. | [optional]
**r#type** | Option<**String**> | Type of the Policy. See our documentation for more information on the type and it's data. | [optional]
**minimum** | Option<**i32**> | Minimum deployments at all time | [optional]
**maximum** | Option<**i32**> | Maximum deployments for the provided type. | [optional]
**threshold** | Option<**f64**> | Based on the number of sockets connected, how filled should a session deployment be before initiating a scale-up deployment. Float between 0.1 and 1. | [optional]
**data** | Option<[**serde_json::Value**](.md)> | JSON object for your filters. See our documentation for more information. | [optional]
**create_time** | Option<**String**> | UTC time of policy creation | [optional]
**last_updated** | Option<**String**> | UTC time of policy last update | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


