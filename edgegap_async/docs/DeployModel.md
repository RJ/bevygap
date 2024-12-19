# DeployModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**app_name** | **String** | The name of the App you want to deploy | 
**version_name** | Option<**String**> | The name of the App Version you want to deploy, if not present, the last version created is picked | [optional]
**is_public_app** | Option<**bool**> | If the Application is public or private. If not specified, we will look for a private Application | [optional]
**ip_list** | Option<**Vec<String>**> | The List of IP of your user | [optional]
**geo_ip_list** | Option<[**Vec<models::GeoIpListModel>**](GeoIpListModel.md)> | The list of IP of your user with their location (latitude, longitude) | [optional]
**telemetry_profile_uuid_list** | Option<**Vec<String>**> |  | [optional]
**env_vars** | Option<[**Vec<models::DeployEnvModel>**](DeployEnvModel.md)> | A list of deployment variables | [optional]
**skip_telemetry** | Option<**bool**> | If you want to skip the Telemetry and use a geolocations decision only | [optional][default to true]
**location** | Option<[**models::LocationModel**](LocationModel.md)> |  | [optional]
**city** | Option<**String**> | DEPRECATED - See `filters` instead. | [optional]
**country** | Option<**String**> | DEPRECATED - See `filters` instead. | [optional]
**continent** | Option<**String**> | DEPRECATED - See `filters` instead. | [optional]
**region** | Option<**String**> | DEPRECATED - See `filters` instead. | [optional]
**administrative_division** | Option<**String**> | DEPRECATED - See `filters` instead. | [optional]
**webhook_url** | Option<**String**> | A web URL. This url will be called with method POST. The deployment status will be send in JSON format | [optional]
**tags** | Option<**Vec<String>**> | The list of tags for your deployment | [optional]
**container_log_storage** | Option<[**models::ContainerLogStorageModel**](ContainerLogStorageModel.md)> |  | [optional]
**filters** | Option<[**Vec<models::ApiModelDeploymentfilter>**](api-model-deploymentfilter.md)> | Filters to use while choosing the deployment location. | [optional]
**ap_sort_strategy** | Option<**String**> | Algorithm used to select the edge location | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


