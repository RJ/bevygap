# SessionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**session_id** | **String** | The Unique Identifier of the Session | 
**custom_id** | Option<**String**> | Custom ID if Available | [optional]
**app** | **String** | The Name of the App you requested | 
**version** | **String** | The name of the App Version you requested | 
**deployment_request_id** | Option<**String**> | Unique UUID | [optional]
**selectors** | Option<[**Vec<models::SelectorModel>**](SelectorModel.md)> | List of Selectors to filter potential Deployment to link and tag the Session | [optional]
**webhook_url** | Option<**String**> | When your Session is Linked, Unprocessable or in Error, we will POST the session's details on the webhook_url  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


