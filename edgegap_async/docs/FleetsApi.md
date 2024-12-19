# \FleetsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fleet_create**](FleetsApi.md#fleet_create) | **POST** /v1/fleet | Create a Fleet
[**fleet_delete**](FleetsApi.md#fleet_delete) | **DELETE** /v1/fleet/{fleet_name} | Delete a Fleet
[**fleet_get**](FleetsApi.md#fleet_get) | **GET** /v1/fleet/{fleet_name} | Get a Fleet
[**fleet_link_app_version**](FleetsApi.md#fleet_link_app_version) | **PUT** /v1/fleet/{fleet_name}/app/{app_name}/version/{version_name} | Link an Application Version to a Fleet
[**fleet_policies_create**](FleetsApi.md#fleet_policies_create) | **POST** /v1/fleet/{fleet_name}/policies | Create a Fleet Policy
[**fleet_policies_delete**](FleetsApi.md#fleet_policies_delete) | **DELETE** /v1/fleet/{fleet_name}/policies/{policy_name} | Delete a Policy
[**fleet_policies_get**](FleetsApi.md#fleet_policies_get) | **GET** /v1/fleet/{fleet_name}/policies/{policy_name} | Get a Policy
[**fleet_policies_list**](FleetsApi.md#fleet_policies_list) | **GET** /v1/fleet/{fleet_name}/policies | List All Policies of a Fleet
[**fleet_policies_update**](FleetsApi.md#fleet_policies_update) | **PATCH** /v1/fleet/{fleet_name}/policies/{policy_name} | Update a Policy
[**fleet_unlink_app_version**](FleetsApi.md#fleet_unlink_app_version) | **DELETE** /v1/fleet/{fleet_name}/app/{app_name}/version/{version_name} | Unlink an Application Version From a Fleet
[**fleet_update**](FleetsApi.md#fleet_update) | **PATCH** /v1/fleet/{fleet_name} | Update a Fleet
[**fleets**](FleetsApi.md#fleets) | **GET** /v1/fleets | List All Fleets



## fleet_create

> models::FleetPostResponse fleet_create(payload)
Create a Fleet

Create a fleet. A fleet is a top-level object; you must create child resources to work properly.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | [**FleetPostPayload**](FleetPostPayload.md) |  | [required] |

### Return type

[**models::FleetPostResponse**](FleetPostResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fleet_delete

> models::FleetDeleteResponse fleet_delete(fleet_name)
Delete a Fleet

Delete a fleet, its policies and links between the application versions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fleet_name** | **String** | The name of the fleet | [required] |

### Return type

[**models::FleetDeleteResponse**](FleetDeleteResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fleet_get

> models::FleetGetResponse fleet_get(fleet_name)
Get a Fleet

Retrieve a fleet with its details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fleet_name** | **String** | The name of the fleet | [required] |

### Return type

[**models::FleetGetResponse**](FleetGetResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fleet_link_app_version

> models::HorizontalScalerAppVersionLink fleet_link_app_version(fleet_name, app_name, version_name)
Link an Application Version to a Fleet

Link an application version to a fleet. By linking this version, the fleet will automatically create deployments of this version according to the fleet policies.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fleet_name** | **String** | The name of the fleet | [required] |
**app_name** | **String** | The name of the application to link | [required] |
**version_name** | **String** | The name of the application version to link | [required] |

### Return type

[**models::HorizontalScalerAppVersionLink**](HorizontalScalerAppVersionLink.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fleet_policies_create

> models::FleetPoliciesPostResponse fleet_policies_create(fleet_name, payload)
Create a Fleet Policy

Create a fleet policy. Policies are conditions that the fleet must respect.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fleet_name** | **String** | The name of the fleet | [required] |
**payload** | [**FleetPoliciesPostPayload**](FleetPoliciesPostPayload.md) |  | [required] |

### Return type

[**models::FleetPoliciesPostResponse**](FleetPoliciesPostResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fleet_policies_delete

> fleet_policies_delete(fleet_name, policy_name)
Delete a Policy

Delete a policy. It will not delete the fleet.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fleet_name** | **String** | The name of the fleet | [required] |
**policy_name** | **String** | The name of the policy to delete | [required] |

### Return type

 (empty response body)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fleet_policies_get

> models::FleetPoliciesGetResponse fleet_policies_get(fleet_name, policy_name)
Get a Policy

Retrieve a specific policy of a fleet.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fleet_name** | **String** | The name of the fleet | [required] |
**policy_name** | **String** | The name of the policy | [required] |

### Return type

[**models::FleetPoliciesGetResponse**](FleetPoliciesGetResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fleet_policies_list

> models::HorizontalScalerConstraintList fleet_policies_list(fleet_name, page, limit, x_fields)
List All Policies of a Fleet

List all the policies of a fleet.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fleet_name** | **String** | The name of the fleet | [required] |
**page** | Option<**i32**> | Page number for pagination |  |[default to 1]
**limit** | Option<**i32**> | Limit of Fleet for each page |  |[default to 10]
**x_fields** | Option<**String**> | An optional fields mask |  |

### Return type

[**models::HorizontalScalerConstraintList**](HorizontalScalerConstraintList.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fleet_policies_update

> models::FleetPoliciesPatchPayload fleet_policies_update(fleet_name, policy_name, payload)
Update a Policy

Update a policy with new specifications.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fleet_name** | **String** | The name of the fleet | [required] |
**policy_name** | **String** | The name of the policy | [required] |
**payload** | [**FleetPoliciesPatchPayload**](FleetPoliciesPatchPayload.md) |  | [required] |

### Return type

[**models::FleetPoliciesPatchPayload**](FleetPoliciesPatchPayload.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fleet_unlink_app_version

> fleet_unlink_app_version(fleet_name, app_name, version_name)
Unlink an Application Version From a Fleet

Unlink an application version from a fleet. It will not delete the application version or the fleet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fleet_name** | **String** | The name of the fleet | [required] |
**app_name** | **String** | The name of the application to link | [required] |
**version_name** | **String** | The name of the application version to link | [required] |

### Return type

 (empty response body)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fleet_update

> models::FleetPatchResponse fleet_update(fleet_name, payload)
Update a Fleet

Update a fleet with new specifications.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fleet_name** | **String** | The name of the fleet | [required] |
**payload** | [**FleetPatchPayload**](FleetPatchPayload.md) |  | [required] |

### Return type

[**models::FleetPatchResponse**](FleetPatchResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fleets

> models::FleetList fleets(page, limit)
List All Fleets

List all the fleets you own.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Page number for pagination |  |[default to 1]
**limit** | Option<**i32**> | Limit of Fleet for each page |  |[default to 10]

### Return type

[**models::FleetList**](FleetList.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

