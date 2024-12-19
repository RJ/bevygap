# SessionModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**app_name** | **String** | The Name of the App you want to deploy, example:    supermario | 
**version_name** | Option<**String**> | The Name of the App Version you want to deploy, example:    v1.0 | [optional]
**ip_list** | Option<**Vec<String>**> | The List of IP of your user, Array of String, example:     [\"162.254.103.13\",\"198.12.116.39\", \"162.254.135.39\", \"162.254.129.34\"] | [optional]
**geo_ip_list** | Option<[**Vec<models::GeoIpListModel>**](GeoIpListModel.md)> | The list of IP of your user with their location (latitude, longitude) | [optional]
**deployment_request_id** | Option<**String**> | The request id of your deployment. If specified, the session will link to the deployment | [optional]
**location** | Option<[**models::LocationModel**](LocationModel.md)> |  | [optional]
**city** | Option<**String**> | If you want your session in a specific city | [optional]
**country** | Option<**String**> | If you want your session in a specific country | [optional]
**continent** | Option<**String**> | If you want your session in a specific continent | [optional]
**administrative_division** | Option<**String**> | If you want your session in a specific administrative division | [optional]
**region** | Option<**String**> | If you want your session in a specific region | [optional]
**selectors** | Option<[**Vec<models::SelectorModel>**](SelectorModel.md)> | List of Selectors to filter potential Deployment to link and tag the Session | [optional]
**webhook_url** | Option<**String**> | When your Session is Linked, Unprocessable or in Error, we will POST the session's details on the webhook_url  | [optional]
**filters** | Option<[**Vec<models::SessionFilterModel>**](SessionFilterModel.md)> | List of location filters to apply to the session | [optional]
**skip_telemetry** | Option<**bool**> | If system should skip the telemetry and use GeoBase decision only | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


