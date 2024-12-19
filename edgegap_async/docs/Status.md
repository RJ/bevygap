# Status

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**request_id** | **String** | The Unique ID of the Deployment's request | 
**fqdn** | **String** | The FQDN that allow to connect to your Deployment | 
**app_name** | **String** | The name of the deployed App | 
**app_version** | **String** | The version of the deployed App | 
**current_status** | **String** | The current status of the Deployment | 
**running** | **bool** | True if the current Deployment is ready to be connected and running | 
**whitelisting_active** | **bool** | True if the current Deployment is ACL protected | 
**start_time** | **String** | Timestamp of the Deployment when it is up and running | 
**removal_time** | Option<**String**> | Timestamp of the end of the Deployment | [optional]
**elapsed_time** | **i32** | Time since the Deployment is up and running in seconds | 
**last_status** | Option<**String**> | The last status of the Deployment | [optional]
**error** | **bool** | True if there is an error with the Deployment | 
**error_detail** | Option<**String**> | The error details of the Deployment | [optional]
**ports** | Option<[**std::collections::HashMap<String, models::PortMapping>**](PortMapping.md)> |  | [optional]
**public_ip** | **String** | The public IP | 
**sessions** | Option<[**Vec<models::DeploymentSessionContext>**](DeploymentSessionContext.md)> | List of Active Sessions if Deployment App is Session Based | [optional]
**location** | Option<[**models::DeploymentLocation**](DeploymentLocation.md)> |  | [optional]
**tags** | Option<**Vec<String>**> | List of tags associated with the deployment | [optional]
**sockets** | Option<**i32**> | The Capacity of the Deployment | [optional]
**sockets_usage** | Option<**i32**> | The Capacity Usage of the Deployment | [optional]
**command** | Option<**String**> | The command to use in the container, null mean it will take the default of the container | [optional]
**arguments** | Option<**String**> | The arguments to use in the container, null mean it will take the default of the container | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


