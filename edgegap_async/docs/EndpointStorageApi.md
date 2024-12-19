# \EndpointStorageApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**endpoint_create**](EndpointStorageApi.md#endpoint_create) | **POST** /v1/storage/endpoint | Create a New Endpoint Storage
[**endpoint_delete**](EndpointStorageApi.md#endpoint_delete) | **DELETE** /v1/storage/endpoint/{endpoint_name} | Delete an Endpoint Storage
[**endpoint_get**](EndpointStorageApi.md#endpoint_get) | **GET** /v1/storage/endpoint/{endpoint_name} | Get an Endpoint Storage
[**endpoint_update**](EndpointStorageApi.md#endpoint_update) | **PATCH** /v1/storage/endpoint/{endpoint_name} | Update an Endpoint Storage
[**endpoints_list**](EndpointStorageApi.md#endpoints_list) | **GET** /v1/storage/endpoints | List All Endpoint Storage
[**pull_profile_create**](EndpointStorageApi.md#pull_profile_create) | **POST** /v1/storage/endpoint/{endpoint_name}/pull-profile | Create a New Pull Profile
[**pull_profile_delete**](EndpointStorageApi.md#pull_profile_delete) | **DELETE** /v1/storage/endpoint/{endpoint_name}/pull-profile/{pull_profile_name} | Delete a Pull Profile
[**pull_profile_get**](EndpointStorageApi.md#pull_profile_get) | **GET** /v1/storage/endpoint/{endpoint_name}/pull-profile/{pull_profile_name} | Get a Pull Profile
[**pull_profile_link_app_version**](EndpointStorageApi.md#pull_profile_link_app_version) | **PUT** /v1/storage/endpoint/{endpoint_name}/pull-profile/{pull_profile_name}/app/{app_name}/version/{version_name} | Link a Pull Profile to an Application Version
[**pull_profile_list**](EndpointStorageApi.md#pull_profile_list) | **GET** /v1/storage/endpoint/{endpoint_name}/pull-profiles | List All Pull Profile of an Endpoint Storage
[**pull_profile_unlink_app_version**](EndpointStorageApi.md#pull_profile_unlink_app_version) | **DELETE** /v1/storage/endpoint/{endpoint_name}/pull-profile/{pull_profile_name}/app/{app_name}/version/{version_name} | Unlink a Pull Profile From an Application Version
[**pull_profile_update**](EndpointStorageApi.md#pull_profile_update) | **PATCH** /v1/storage/endpoint/{endpoint_name}/pull-profile/{pull_profile_name} | Update a Pull Profile



## endpoint_create

> models::EndpointStoragePostResponse endpoint_create(payload)
Create a New Endpoint Storage

Create an endpoint storage to store your container logs at the end of a deployment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | [**EndpointStoragePostPayload**](EndpointStoragePostPayload.md) |  | [required] |

### Return type

[**models::EndpointStoragePostResponse**](EndpointStoragePostResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoint_delete

> models::EndpointStorageDeleteResponse endpoint_delete(endpoint_name)
Delete an Endpoint Storage

Delete an endpoint storage. All the application versions linked to it won't be able to store logs anymore.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_name** | **String** |  | [required] |

### Return type

[**models::EndpointStorageDeleteResponse**](EndpointStorageDeleteResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoint_get

> models::EndpointStorageGetResponse endpoint_get(endpoint_name)
Get an Endpoint Storage

Retrieve an endpoint storage. The ```secret_access_key``` won't be displayed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_name** | **String** |  | [required] |

### Return type

[**models::EndpointStorageGetResponse**](EndpointStorageGetResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoint_update

> models::EndpointStoragePatchResponse endpoint_update(endpoint_name, payload)
Update an Endpoint Storage

Update an Endpoint Storage with new specifications.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_name** | **String** |  | [required] |
**payload** | [**EndpointStoragePatchPayload**](EndpointStoragePatchPayload.md) |  | [required] |

### Return type

[**models::EndpointStoragePatchResponse**](EndpointStoragePatchResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endpoints_list

> models::EndpointStorageListResponse endpoints_list(page, limit, x_fields)
List All Endpoint Storage

List all endpoint storage.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number for pagination |  |[default to 1]
**limit** | Option<**i32**> | Limit of Fleet for each page |  |[default to 10]
**x_fields** | Option<**String**> | An optional fields mask |  |

### Return type

[**models::EndpointStorageListResponse**](EndpointStorageListResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pull_profile_create

> models::PullProfilePostResponse pull_profile_create(endpoint_name, payload)
Create a New Pull Profile

Create a pull profile. Pull profile will upload data from an endpoint storage to a deployment container on boot. You must link the application version to the pull profile first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_name** | **String** |  | [required] |
**payload** | [**PullProfilePostPayload**](PullProfilePostPayload.md) |  | [required] |

### Return type

[**models::PullProfilePostResponse**](PullProfilePostResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pull_profile_delete

> pull_profile_delete(endpoint_name, pull_profile_name)
Delete a Pull Profile

Delete a pull profile. All the application versions linked won't receive the data upload anymore. It will not delete your endpoint storage.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_name** | **String** |  | [required] |
**pull_profile_name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pull_profile_get

> models::PullProfileGetResponse pull_profile_get(endpoint_name, pull_profile_name)
Get a Pull Profile

Retrieve a pull profile and its specifications.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_name** | **String** |  | [required] |
**pull_profile_name** | **String** |  | [required] |

### Return type

[**models::PullProfileGetResponse**](PullProfileGetResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pull_profile_link_app_version

> models::PullProfileAppVersionLinkResponse pull_profile_link_app_version(endpoint_name, pull_profile_name, app_name, version_name)
Link a Pull Profile to an Application Version

Link a pull profile to an app version. Without a link, the pull profile by itself will do nothing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_name** | **String** |  | [required] |
**pull_profile_name** | **String** |  | [required] |
**app_name** | **String** |  | [required] |
**version_name** | **String** |  | [required] |

### Return type

[**models::PullProfileAppVersionLinkResponse**](PullProfileAppVersionLinkResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pull_profile_list

> models::PullProfilesListResponse pull_profile_list(endpoint_name, page, limit)
List All Pull Profile of an Endpoint Storage

List all pull profiles of an endpoint storage.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_name** | **String** |  | [required] |
**page** | Option<**i32**> | Page number for pagination |  |[default to 1]
**limit** | Option<**i32**> | Limit of pull profiles for each page |  |[default to 10]

### Return type

[**models::PullProfilesListResponse**](PullProfilesListResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pull_profile_unlink_app_version

> pull_profile_unlink_app_version(endpoint_name, pull_profile_name, app_name, version_name)
Unlink a Pull Profile From an Application Version

Unlink a pull profile from an app version. It will not delete the pull profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_name** | **String** |  | [required] |
**pull_profile_name** | **String** |  | [required] |
**app_name** | **String** |  | [required] |
**version_name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pull_profile_update

> models::PulloProfilePatchResponse pull_profile_update(endpoint_name, pull_profile_name, payload)
Update a Pull Profile

Update a pull profile with new specifications.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_name** | **String** |  | [required] |
**pull_profile_name** | **String** |  | [required] |
**payload** | [**PullProfilePatchPayload**](PullProfilePatchPayload.md) |  | [required] |

### Return type

[**models::PulloProfilePatchResponse**](PulloProfilePatchResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

