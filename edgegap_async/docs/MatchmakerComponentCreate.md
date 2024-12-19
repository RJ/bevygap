# MatchmakerComponentCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Matchmaker component name. Must be unique. | 
**repository** | **String** | Container repository where the component's image is hosted. | 
**image** | **String** | Container image to use for this component. | 
**tag** | **String** | Tag of the container image to use for this component. | 
**credentials** | Option<[**models::ComponentCredentials**](ComponentCredentials.md)> | Private repo credentials to use for pulling the image, if applicable. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


