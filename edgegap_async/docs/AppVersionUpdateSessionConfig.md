# AppVersionUpdateSessionConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | Option<**String**> | The kind of session to create. If 'Default' if chosen, the application will be handled like a normal application. The kind of session must be: Default, Seat, Match | [optional]
**sockets** | Option<**i32**> | The number of game slots on each deployment of this app version. | [optional]
**autodeploy** | Option<**bool**> | If a deployment should be made autonomously if there is not enough sockets open on a new session. | [optional]
**empty_ttl** | Option<**i32**> | The number of minutes a deployment of this app version can spend with no session connected before being terminated. | [optional]
**session_max_duration** | Option<**i32**> | The number of minutes after a session-type deployment has been terminated to remove all the session information connected to your deployment. Minimum and default value is set to 60 minutes so you can manage your session termination before it is removed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


