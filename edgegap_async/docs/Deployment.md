# Deployment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**request_id** | **String** | Unique UUID | 
**public_ip** | **String** | The public IP | 
**status** | **String** | Current status of Deployment | 
**ready** | **bool** | if the deployment is ready | 
**whitelisting_active** | **bool** | if the deployment ACL is active | 
**fqdn** | **String** |  | 
**ports** | Option<[**std::collections::HashMap<String, models::PortMapping>**](PortMapping.md)> |  | [optional]
**location** | Option<[**models::DeploymentLocation**](DeploymentLocation.md)> |  | [optional]
**tags** | Option<**Vec<String>**> | List of tags associated with the deployment | [optional]
**sockets** | Option<**i32**> | The Capacity of the Deployment | [optional]
**sockets_usage** | Option<**i32**> | The Capacity Usage of the Deployment | [optional]
**is_joinable_by_session** | Option<**bool**> | If the deployment is joinable by sessions | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


