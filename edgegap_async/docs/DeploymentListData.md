# DeploymentListData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**request_id** | **String** | Unique UUID | 
**fqdn** | **String** | The FQDN that allow to connect to your deployment | 
**start_time** | **String** | Timestamp of the deployment when it is up and running | 
**ready** | **bool** | If the deployment is ready | 
**public_ip** | **String** | The public IP | 
**ports** | Option<[**std::collections::HashMap<String, models::PortMapping>**](PortMapping.md)> |  | [optional]
**tags** | Option<**Vec<String>**> | List of tags associated with the deployment | [optional]
**sockets** | Option<**i32**> | The capacity of the deployment | [optional]
**sockets_usage** | Option<**i32**> | The capacity usage of the deployment | [optional]
**is_joinable_by_session** | Option<**bool**> | If the deployment is joinable by sessions | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


