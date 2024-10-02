# \DeploymentTagsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**deployments_tags_create**](DeploymentTagsApi.md#deployments_tags_create) | **POST** /v1/deployments/{request_id}/tags | Create Tag for a Deployment
[**deployments_tags_delete**](DeploymentTagsApi.md#deployments_tags_delete) | **DELETE** /v1/deployments/{request_id}/tags/{tag_name} | Delete Tag for a Deployment
[**deployments_tags_list**](DeploymentTagsApi.md#deployments_tags_list) | **GET** /v1/deployments/{request_id}/tags | List tags for a Deployment
[**deployments_tags_read**](DeploymentTagsApi.md#deployments_tags_read) | **GET** /v1/deployments/{request_id}/tags/{tag_name} | Get tag for a Deployment
[**deployments_tags_update**](DeploymentTagsApi.md#deployments_tags_update) | **PATCH** /v1/deployments/{request_id}/tags/{tag_name} | Update Tag for a Deployment



## deployments_tags_create

> models::DeploymentTagResponse deployments_tags_create(request_id, payload)
Create Tag for a Deployment

Create a tag for a deployment. The tag will however not be injected into a running container.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** |  | [required] |
**payload** | [**DeploymentTagPayload**](DeploymentTagPayload.md) |  | [required] |

### Return type

[**models::DeploymentTagResponse**](DeploymentTagResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deployments_tags_delete

> deployments_tags_delete(request_id, tag_name)
Delete Tag for a Deployment

Delete a tag for a deployment. The tag will however not be removed from the environment of a running container.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** |  | [required] |
**tag_name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deployments_tags_list

> models::DeploymentTagListResponse deployments_tags_list(request_id)
List tags for a Deployment

List tags for a deployment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** |  | [required] |

### Return type

[**models::DeploymentTagListResponse**](DeploymentTagListResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deployments_tags_read

> models::DeploymentTagResponse deployments_tags_read(request_id, tag_name)
Get tag for a Deployment

Get tag for a deployment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** |  | [required] |
**tag_name** | **String** |  | [required] |

### Return type

[**models::DeploymentTagResponse**](DeploymentTagResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deployments_tags_update

> models::DeploymentTagResponse deployments_tags_update(request_id, tag_name, payload)
Update Tag for a Deployment

Update a tag for a deployment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** |  | [required] |
**tag_name** | **String** |  | [required] |
**payload** | [**DeploymentTagPayload**](DeploymentTagPayload.md) |  | [required] |

### Return type

[**models::DeploymentTagResponse**](DeploymentTagResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

