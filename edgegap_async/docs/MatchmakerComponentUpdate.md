# MatchmakerComponentUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Matchmaker component name. Must be unique. | [optional]
**repository** | Option<**String**> | Container repository where the component's image is hosted. | [optional]
**image** | Option<**String**> | Container image to use for this component. | [optional]
**tag** | Option<**String**> | Tag of the container image to use for this component. | [optional]
**credentials** | Option<[**models::ComponentCredentials**](ComponentCredentials.md)> | Private repo credentials to use for pulling the image, if applicable. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


