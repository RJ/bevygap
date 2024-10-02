# \ApplicationsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**app_version_delete**](ApplicationsApi.md#app_version_delete) | **DELETE** /v1/app/{app_name}/version/{version_name} | Delete an Application Version
[**app_version_get**](ApplicationsApi.md#app_version_get) | **GET** /v1/app/{app_name}/version/{version_name} | Get an Application Version
[**app_version_post**](ApplicationsApi.md#app_version_post) | **POST** /v1/app/{app_name}/version | Create a New Application Version
[**app_version_whitelist_entry_delete**](ApplicationsApi.md#app_version_whitelist_entry_delete) | **DELETE** /v1/app/{app_name}/version/{version_name}/whitelist/{entry_id} | Delete an ACL Entry
[**app_version_whitelist_entry_get**](ApplicationsApi.md#app_version_whitelist_entry_get) | **GET** /v1/app/{app_name}/version/{version_name}/whitelist/{entry_id} | Get an ACL Entry
[**app_version_whitelist_get**](ApplicationsApi.md#app_version_whitelist_get) | **GET** /v1/app/{app_name}/version/{version_name}/whitelist | List All ACL Entries for an Application Version
[**app_version_whitelist_post**](ApplicationsApi.md#app_version_whitelist_post) | **POST** /v1/app/{app_name}/version/{version_name}/whitelist | Create an ACL Entry
[**app_versions_get**](ApplicationsApi.md#app_versions_get) | **GET** /v1/app/{app_name}/versions | List All Versions for an Application
[**app_versions_patch**](ApplicationsApi.md#app_versions_patch) | **PATCH** /v1/app/{app_name}/version/{version_name} | Update an Application Version
[**application_delete**](ApplicationsApi.md#application_delete) | **DELETE** /v1/app/{app_name} | Delete an Application
[**application_get**](ApplicationsApi.md#application_get) | **GET** /v1/app/{app_name} | Get an Application
[**application_patch**](ApplicationsApi.md#application_patch) | **PATCH** /v1/app/{app_name} | Update an Application
[**application_post**](ApplicationsApi.md#application_post) | **POST** /v1/app | Create a New Application
[**applications_get**](ApplicationsApi.md#applications_get) | **GET** /v1/apps | List All Applications



## app_version_delete

> models::AppVersionDelete app_version_delete(app_name, version_name)
Delete an Application Version

Delete a specific version of an application.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | The name of the application | [required] |
**version_name** | **String** | The name of the application version | [required] |

### Return type

[**models::AppVersionDelete**](AppVersionDelete.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_version_get

> models::AppVersionPayload app_version_get(app_name, version_name)
Get an Application Version

Retrieve the specifications of an application version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | The name of the application | [required] |
**version_name** | **String** | The name of the application version | [required] |

### Return type

[**models::AppVersionPayload**](AppVersionPayload.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_version_post

> models::AppVersionCreateResponse app_version_post(app_name, payload)
Create a New Application Version

Create an application version associated with an application. The version contains all the specifications to create a deployment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | The name of the application associated | [required] |
**payload** | [**AppVersionPayload**](AppVersionPayload.md) |  | [required] |

### Return type

[**models::AppVersionCreateResponse**](AppVersionCreateResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_version_whitelist_entry_delete

> models::AppVersionWhitelistEntrySuccess app_version_whitelist_entry_delete(app_name, version_name, entry_id)
Delete an ACL Entry

Delete an access control list entry for a specific application version

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | The name of the application | [required] |
**version_name** | **String** | The name of the application version | [required] |
**entry_id** | **String** | The unique ID of the entry | [required] |

### Return type

[**models::AppVersionWhitelistEntrySuccess**](AppVersionWhitelistEntrySuccess.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_version_whitelist_entry_get

> models::AppVersionWhitelistEntry app_version_whitelist_entry_get(app_name, version_name, entry_id)
Get an ACL Entry

Retrieve a specific access control list entry for an application version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | The name of the application | [required] |
**version_name** | **String** | The name of the application version | [required] |
**entry_id** | **String** | The unique ID of the entry | [required] |

### Return type

[**models::AppVersionWhitelistEntry**](AppVersionWhitelistEntry.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_version_whitelist_get

> models::AppVersionWhitelistResponse app_version_whitelist_get(app_name, version_name)
List All ACL Entries for an Application Version

List all the access control list entries for a specific application version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | The name of the application | [required] |
**version_name** | **String** | The name of the application version | [required] |

### Return type

[**models::AppVersionWhitelistResponse**](AppVersionWhitelistResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_version_whitelist_post

> models::AppVersionWhitelistEntrySuccess app_version_whitelist_post(app_name, version_name, payload)
Create an ACL Entry

Create an access control list entry for an app version. This will allow the specified CIDR to connect to the deployment. The option ```whitelisting_active``` must be activated in the application version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | The name of the application | [required] |
**version_name** | **String** | The name of the application version | [required] |
**payload** | [**AppVersionWhitelistEntryPayload**](AppVersionWhitelistEntryPayload.md) |  | [required] |

### Return type

[**models::AppVersionWhitelistEntrySuccess**](AppVersionWhitelistEntrySuccess.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_versions_get

> models::AppVersionList app_versions_get(app_name)
List All Versions for an Application

List all versions of a specific application.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | The name of the application | [required] |

### Return type

[**models::AppVersionList**](AppVersionList.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## app_versions_patch

> models::AppVersionUpdateResponse app_versions_patch(app_name, version_name, payload)
Update an Application Version

Update an application version with new specifications.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** | The name of the application | [required] |
**version_name** | **String** | The name of the application version | [required] |
**payload** | [**AppVersionUpdatePayload**](AppVersionUpdatePayload.md) |  | [required] |

### Return type

[**models::AppVersionUpdateResponse**](AppVersionUpdateResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## application_delete

> application_delete(app_name)
Delete an Application

Delete an application and all its current versions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## application_get

> models::Application application_get(app_name)
Get an Application

Retrieve an application and its information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** |  | [required] |

### Return type

[**models::Application**](Application.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## application_patch

> models::Application application_patch(app_name, payload)
Update an Application

Update an application with new information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_name** | **String** |  | [required] |
**payload** | [**ApplicationPatch**](ApplicationPatch.md) |  | [required] |

### Return type

[**models::Application**](Application.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## application_post

> models::Application application_post(payload)
Create a New Application

Create an application that will regroup application versions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | [**ApplicationPost**](ApplicationPost.md) |  | [required] |

### Return type

[**models::Application**](Application.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_get

> models::Applications applications_get()
List All Applications

List all the applications that you own.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Applications**](Applications.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

