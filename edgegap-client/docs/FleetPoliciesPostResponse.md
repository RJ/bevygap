# FleetPoliciesPostResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name of the Policy | 
**enabled** | **bool** | If the Policy is enabled. Defaults to true. | 
**r#type** | **String** | Type of the Policy. See our documentation for more information on the type and it's data. | 
**minimum** | **i32** | Minimum deployments at all time | 
**maximum** | **i32** | Maximum deployments for the provided type. | 
**threshold** | **f64** | Based on the number of sockets connected, how filled should a session deployment be before initiating a scale-up deployment. Float between 0.1 and 1. | 
**data** | [**serde_json::Value**](.md) | JSON object for your filters. See our documentation for more information. | 
**create_time** | Option<**String**> | UTC time of policy creation | [optional]
**last_updated** | Option<**String**> | UTC time of policy last update | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


