# \CustomSessionsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_custom_session**](CustomSessionsApi.md#delete_custom_session) | **DELETE** /v1/deployment/{request_id}/custom/session/{custom_id} | Delete a Custom Session
[**delete_custom_sessions**](CustomSessionsApi.md#delete_custom_sessions) | **DELETE** /v1/deployment/{request_id}/custom/sessions | Delete Custom Sessions in Bulk
[**get_custom_session**](CustomSessionsApi.md#get_custom_session) | **GET** /v1/deployment/{request_id}/custom/session/{custom_id} | Get a Custom Session
[**get_custom_sessions**](CustomSessionsApi.md#get_custom_sessions) | **GET** /v1/deployment/{request_id}/custom/sessions | List All Custom Session of a Deployment
[**patch_custom_session**](CustomSessionsApi.md#patch_custom_session) | **PATCH** /v1/deployment/{request_id}/custom/session/{custom_id} | Update a Custom Session
[**post_custom_session**](CustomSessionsApi.md#post_custom_session) | **POST** /v1/deployment/{request_id}/custom/session/{custom_id} | Create a New Custom Session
[**post_custom_sessions**](CustomSessionsApi.md#post_custom_sessions) | **POST** /v1/deployment/{request_id}/custom/sessions | Create Custom Sessions in Bulk



## delete_custom_session

> models::SessionDelete delete_custom_session(custom_id, request_id)
Delete a Custom Session

Delete a custom session. Once deleted, a custom session is no more accessible and does not have a history. The deployment associated will not be deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**custom_id** | **String** | Custom ID Managed by you | [required] |
**request_id** | **String** | Deployment Request ID | [required] |

### Return type

[**models::SessionDelete**](SessionDelete.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_custom_sessions

> models::BulkSessionDelete delete_custom_sessions(request_id, payload)
Delete Custom Sessions in Bulk

Delete multiple custom sessions from a specific deployment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** | Deployment Request ID | [required] |
**payload** | [**CustomSessionDeleteModel**](CustomSessionDeleteModel.md) |  | [required] |

### Return type

[**models::BulkSessionDelete**](BulkSessionDelete.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_custom_session

> models::SessionGet get_custom_session(custom_id, request_id)
Get a Custom Session

Retrieve the information of a custom session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**custom_id** | **String** | Custom ID Managed by you | [required] |
**request_id** | **String** | Deployment Request ID | [required] |

### Return type

[**models::SessionGet**](SessionGet.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_custom_sessions

> models::Sessions get_custom_sessions(request_id)
List All Custom Session of a Deployment

List all custom sessions of a deployment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** | Deployment Request ID | [required] |

### Return type

[**models::Sessions**](Sessions.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_custom_session

> models::SessionGet patch_custom_session(custom_id, request_id, payload)
Update a Custom Session

Update a custom session with new specifications.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**custom_id** | **String** | Custom ID Managed by you | [required] |
**request_id** | **String** | Deployment Request ID | [required] |
**payload** | [**CustomSessionModel**](CustomSessionModel.md) |  | [required] |

### Return type

[**models::SessionGet**](SessionGet.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_custom_session

> models::SessionRequest post_custom_session(custom_id, request_id, payload)
Create a New Custom Session

Create a custom session with users. You must specify a custom ID and a deployment request ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**custom_id** | **String** | Custom ID Managed by you | [required] |
**request_id** | **String** | Deployment Request ID | [required] |
**payload** | [**CustomSessionModel**](CustomSessionModel.md) |  | [required] |

### Return type

[**models::SessionRequest**](SessionRequest.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_custom_sessions

> models::BulkSessionPost post_custom_sessions(request_id, payload)
Create Custom Sessions in Bulk

Create multiple custom sessions in a deployment. You must specify a custom ID for each.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** | Deployment Request ID | [required] |
**payload** | [**CustomBulkSessionsModel**](CustomBulkSessionsModel.md) |  | [required] |

### Return type

[**models::BulkSessionPost**](BulkSessionPost.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

