# \DeploymentsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**deploy**](DeploymentsApi.md#deploy) | **POST** /v1/deploy | Create a Deployment
[**deployment_delete**](DeploymentsApi.md#deployment_delete) | **DELETE** /v1/stop/{request_id} | Delete a Deployment
[**deployment_get_logs**](DeploymentsApi.md#deployment_get_logs) | **GET** /v1/deployment/{request_id}/container-logs | Get Deployment Container Logs
[**deployment_status_get**](DeploymentsApi.md#deployment_status_get) | **GET** /v1/status/{request_id} | Get a Deployment Status and Information
[**deployment_update**](DeploymentsApi.md#deployment_update) | **PATCH** /v1/deployments/{request_id} | Updates properties of a deployment
[**deployments_available**](DeploymentsApi.md#deployments_available) | **POST** /v1/deployments:available | Deployments with Available Sockets
[**deployments_bulk_delete**](DeploymentsApi.md#deployments_bulk_delete) | **POST** /v1/deployments/bulk-stop | Delete Deployments in Bulk
[**deployments_get**](DeploymentsApi.md#deployments_get) | **GET** /v1/deployments | List All Deployments
[**self_deployment_delete**](DeploymentsApi.md#self_deployment_delete) | **DELETE** /v1/self/stop/{request_id}/{access_point_id} | Delete a Deployment from inside the container



## deploy

> models::Request deploy(payload)
Create a Deployment

Create a new deployment. Deployment is a server instance of your application version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | [**DeployModel**](DeployModel.md) |  | [required] |

### Return type

[**models::Request**](Request.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deployment_delete

> models::Delete deployment_delete(request_id, container_log_storage)
Delete a Deployment

Delete an instance of deployment. It will stop the running container and all its games.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** | Unique Identifier to keep track of your request across all Arbitrium ecosystem. It's included in the response of the app deploy, example:    93924761ccde | [required] |
**container_log_storage** | Option<**String**> | If you want to enable the container log storage for the deployment. You can put 'true' if you already have endpoint storage associated with your deployment's app version. You can put 'false' if it is enabled by default and you want to disable it for this specific request. Or you can put the name of your endpoint storage and if it is valid we will store the container logs. |  |

### Return type

[**models::Delete**](Delete.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deployment_get_logs

> models::ApiModelContainerlogs deployment_get_logs(request_id)
Get Deployment Container Logs

Retrieve the logs of your container. Logs are not available when your deployment is terminated. You can set up an endpoint storage to save your logs. <a target='_blank' href='https://docs.edgegap.com/docs/deployment/endpoint-storage'>Endpoint Storage Documentation</a>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** |  | [required] |

### Return type

[**models::ApiModelContainerlogs**](api-model-containerlogs.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deployment_status_get

> models::Status deployment_status_get(request_id)
Get a Deployment Status and Information

Retrieve the information for a deployment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** | Unique Identifier to keep track of your request across all Arbitrium ecosystem. It's included in the response of the app deploy, example:    93924761ccde | [required] |

### Return type

[**models::Status**](Status.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deployment_update

> models::DeploymentUpdateResponse deployment_update(request_id, payload)
Updates properties of a deployment

Updates properties of a deployment. Currently only the `is_joinable_by_session` property can be updated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** |  | [required] |
**payload** | [**DeploymentUpdatePayload**](DeploymentUpdatePayload.md) |  | [required] |

### Return type

[**models::DeploymentUpdateResponse**](DeploymentUpdateResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deployments_available

> models::DeploymentAvailableResponse deployments_available(payload)
Deployments with Available Sockets

Get the list of deployments that have available sockets sorted by proximity to the geographical data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | [**DeploymentAvailablePayload**](DeploymentAvailablePayload.md) |  | [required] |

### Return type

[**models::DeploymentAvailableResponse**](DeploymentAvailableResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deployments_bulk_delete

> models::DeploymentBulkStopResponse deployments_bulk_delete(payload)
Delete Deployments in Bulk

Make a bulk delete of deployments using filters. All the deployments matching the given filters will be permanently deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | [**DeploymentBulkStopPayload**](DeploymentBulkStopPayload.md) |  | [required] |

### Return type

[**models::DeploymentBulkStopResponse**](DeploymentBulkStopResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deployments_get

> models::Deployments deployments_get(query)
List All Deployments

List all deployments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | Option<**String**> | Query to filter the deployments. |  |

### Return type

[**models::Deployments**](Deployments.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## self_deployment_delete

> models::Delete self_deployment_delete(request_id, access_point_id, authorization, container_log_storage)
Delete a Deployment from inside the container

Delete a deployment from the inside of a container. You should use this URL inside your deployment. The URL is injected in your deployment and can be found via the environment variable ARBITRIUM_DELETE_URL.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** | Unique Identifier to keep track of your request across all Arbitrium ecosystem. It's included in the response of the app deploy, example:    93924761ccde | [required] |
**access_point_id** | **i32** | Access Point Number provided by our system | [required] |
**authorization** | **String** | Auto Generated token. This token is injected in your deployment and can be found via the environment variable named ARBITRIUM_DELETE_TOKEN | [required] |
**container_log_storage** | Option<**String**> | If you want to enable the container log storage for the deployment. You can put 'true' if you already have endpoint storage associated with your deployment's app version. You can put 'false' if it is enabled by default and you want to disable it for this specific request. Or you can put the name of your endpoint storage and if it is valid we will store the container logs. |  |

### Return type

[**models::Delete**](Delete.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

