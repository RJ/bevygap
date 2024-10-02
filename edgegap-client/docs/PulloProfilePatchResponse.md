# PulloProfilePatchResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the pull profile | [optional]
**source** | Option<**String**> | Source in the S3 bucket to fetch from | [optional]
**source_type** | Option<**String**> | If the source is a File or a Directory | [optional]
**destination** | Option<**String**> | Destination path where your source will be uploaded in your container. Make sure to avoid protected destinations, such as `/etc/`, as this will prevent the files from being copied to your deployment, and will make your deployment fail. Make sure a normal user can write to the destination folder. | [optional]
**create_time** | Option<**String**> | UTC time of pull profile creation | [optional]
**last_updated** | Option<**String**> | UTC time of pull profile last update | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


