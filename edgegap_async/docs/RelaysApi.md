# \RelaysApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**relay_session_create**](RelaysApi.md#relay_session_create) | **POST** /v1/relays/sessions | Create a Relay Session
[**relay_session_delete**](RelaysApi.md#relay_session_delete) | **DELETE** /v1/relays/sessions/{session_id} | Delete a Relay Session
[**relay_session_get**](RelaysApi.md#relay_session_get) | **GET** /v1/relays/sessions/{session_id} | Get a Relay Session
[**relay_session_list**](RelaysApi.md#relay_session_list) | **GET** /v1/relays/sessions | List all Relay Sessions
[**relay_user_authorize**](RelaysApi.md#relay_user_authorize) | **POST** /v1/relays/sessions:authorize-user | Authorize a user on a Relay Session
[**relay_user_revoke**](RelaysApi.md#relay_user_revoke) | **POST** /v1/relays/sessions:revoke-user | Remove a user on a Relay Session



## relay_session_create

> models::RelaySessionBaseResponse relay_session_create(payload)
Create a Relay Session

Create a relay session with users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | [**RelaySessionCreatePayload**](RelaySessionCreatePayload.md) |  | [required] |

### Return type

[**models::RelaySessionBaseResponse**](RelaySessionBaseResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## relay_session_delete

> relay_session_delete(session_id)
Delete a Relay Session

Delete a relay session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## relay_session_get

> models::RelaySessionBaseResponse relay_session_get(session_id)
Get a Relay Session

Retrieve the information for a relay session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** |  | [required] |

### Return type

[**models::RelaySessionBaseResponse**](RelaySessionBaseResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## relay_session_list

> models::RelaySessionListResponse relay_session_list()
List all Relay Sessions

List all the active relay sessions.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RelaySessionListResponse**](RelaySessionListResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## relay_user_authorize

> models::RelaySessionUserBaseResponse relay_user_authorize(payload)
Authorize a user on a Relay Session

Authorize a user on a Relay Session

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | [**RelayUserAuthorizePayload**](RelayUserAuthorizePayload.md) |  | [required] |

### Return type

[**models::RelaySessionUserBaseResponse**](RelaySessionUserBaseResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## relay_user_revoke

> models::RelaySessionBaseResponse relay_user_revoke(payload)
Remove a user on a Relay Session

Authorize a user on a Relay Session

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | [**RelayUserRevokePayload**](RelayUserRevokePayload.md) |  | [required] |

### Return type

[**models::RelaySessionBaseResponse**](RelaySessionBaseResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

