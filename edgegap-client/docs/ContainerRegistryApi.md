# \ContainerRegistryApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**image_tag_delete**](ContainerRegistryApi.md#image_tag_delete) | **DELETE** /v1/container-registry/images/{image_name}/tags/{tag_name} | Delete Tag For a Registry Image
[**registry_image_tag_list**](ContainerRegistryApi.md#registry_image_tag_list) | **GET** /v1/container-registry/images/{image_name}/tags | List All Tags for a Registry Image



## image_tag_delete

> models::ApiModelRegistryartifacttagdeleteresponse image_tag_delete(image_name, tag_name)
Delete Tag For a Registry Image

Delete a single tag from a artifact in the registry.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**image_name** | **String** | The name of the image | [required] |
**tag_name** | **String** | The name of the tag | [required] |

### Return type

[**models::ApiModelRegistryartifacttagdeleteresponse**](api-model-registryartifacttagdeleteresponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## registry_image_tag_list

> models::ImageTagList registry_image_tag_list(image_name)
List All Tags for a Registry Image

List all tags of a specific registry image.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**image_name** | **String** | The name of the image | [required] |

### Return type

[**models::ImageTagList**](ImageTagList.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

