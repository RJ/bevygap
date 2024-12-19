# RelaySessionBaseResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**session_id** | **String** | The session ID | 
**authorization_token** | Option<**i32**> | The authorization token for the session | [optional]
**status** | **String** | The status of the session | 
**ready** | **bool** | If the session is ready to be used | 
**linked** | **bool** | If the session is linked to a relay | 
**error** | Option<**String**> | The error message if the session failed | [optional]
**session_users** | Option<[**Vec<models::RelaySessionUserResponse>**](RelaySessionUserResponse.md)> | List Session Users | [optional]
**relay** | Option<[**models::RelayResponse**](RelayResponse.md)> | The relay details the session is linked to | [optional]
**webhook_url** | Option<**String**> | The webhook URL that we will call once the session is ready | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


