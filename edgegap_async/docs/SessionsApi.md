# \SessionsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_users_session**](SessionsApi.md#delete_users_session) | **DELETE** /v1/session/{session_id}/users | Delete Users From a Session
[**get_session**](SessionsApi.md#get_session) | **GET** /v1/session/{session_id} | Get a Session
[**get_users_session**](SessionsApi.md#get_users_session) | **GET** /v1/session/{session_id}/users | List Users of a Session
[**list_sessions**](SessionsApi.md#list_sessions) | **GET** /v1/sessions | List All Sessions
[**put_users_session**](SessionsApi.md#put_users_session) | **PUT** /v1/session/{session_id}/users | Put Users in a Session
[**session_delete**](SessionsApi.md#session_delete) | **DELETE** /v1/session/{session_id} | Delete a Session
[**session_post**](SessionsApi.md#session_post) | **POST** /v1/session | Create a Session
[**sessions_bulk_stop**](SessionsApi.md#sessions_bulk_stop) | **POST** /v1/sessions/bulk-stop | Delete Sessions in Bulk



## delete_users_session

> models::SessionUserContext delete_users_session(session_id, payload)
Delete Users From a Session

Remove specified users from a session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** |  | [required] |
**payload** | [**PatchSessionModel**](PatchSessionModel.md) |  | [required] |

### Return type

[**models::SessionUserContext**](SessionUserContext.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_session

> models::SessionGet get_session(session_id)
Get a Session

Retrieve the information for a session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** |  | [required] |

### Return type

[**models::SessionGet**](SessionGet.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_session

> models::SessionUserContext get_users_session(session_id)
List Users of a Session

List all the users of session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** |  | [required] |

### Return type

[**models::SessionUserContext**](SessionUserContext.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_sessions

> models::Sessions list_sessions()
List All Sessions

List all the active sessions.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Sessions**](Sessions.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_users_session

> models::SessionUserContext put_users_session(session_id, payload)
Put Users in a Session

Add specified users to a session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** |  | [required] |
**payload** | [**PatchSessionModel**](PatchSessionModel.md) |  | [required] |

### Return type

[**models::SessionUserContext**](SessionUserContext.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## session_delete

> models::SessionDelete session_delete(session_id)
Delete a Session

Delete a session. Once deleted, a session is no more accessible and does not have a history. The deployment associated will not be deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** |  | [required] |

### Return type

[**models::SessionDelete**](SessionDelete.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## session_post

> models::SessionRequest session_post(payload)
Create a Session

Create a session with users. Sessions are linked to a deployment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | [**SessionModel**](SessionModel.md) |  | [required] |

### Return type

[**models::SessionRequest**](SessionRequest.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sessions_bulk_stop

> models::SessionBulkStopResponse sessions_bulk_stop(payload)
Delete Sessions in Bulk

Make a bulk delete of sessions using filters. All the sessions matching the given filters will be permanently deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | [**SessionBulkStopPayload**](SessionBulkStopPayload.md) |  | [required] |

### Return type

[**models::SessionBulkStopResponse**](SessionBulkStopResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

