# SessionContext

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**session_id** | **String** | Unique UUID | 
**custom_id** | Option<**String**> | Custom ID if Available | [optional]
**status** | **String** | Current status of the session | 
**ready** | **bool** | If the session is linked to a Ready deployment | 
**linked** | **bool** | If the session is linked to a deployment | 
**kind** | **String** | Type of session created | 
**user_count** | **i32** | Count of user this session currently have | 
**deployment_request_id** | Option<**String**> | Unique UUID | [optional]
**webhook_url** | Option<**String**> | When your Session is Linked, Unprocessable or in Error, we will POST the session's details on the webhook_url  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


