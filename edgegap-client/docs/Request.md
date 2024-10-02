# Request

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**request_id** | **String** | The Unique Identifier of the request | 
**request_dns** | **String** | The URL to connect to the instance | 
**request_app** | **String** | The Name of the App you requested | 
**request_version** | **String** | The name of the App Version you requested | 
**request_user_count** | **i32** | How Many Users your request contain | 
**city** | Option<**String**> | The city where the deployment is located | [optional]
**country** | Option<**String**> | The country where the deployment is located | [optional]
**continent** | Option<**String**> | The continent where the deployment is located | [optional]
**administrative_division** | Option<**String**> | The administrative division where the deployment is located | [optional]
**tags** | Option<**Vec<String>**> | List of tags associated with the deployment | [optional]
**container_log_storage** | [**models::ContainerLogStorageModel**](ContainerLogStorageModel.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


