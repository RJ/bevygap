# \TelemetryApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**active_deployment_telemetry_get**](TelemetryApi.md#active_deployment_telemetry_get) | **GET** /v1/telemetry/active-deployments/{retrieval_key} | Get the Result of an Active Deployment Telemetry Request
[**active_deployment_telemetry_post**](TelemetryApi.md#active_deployment_telemetry_post) | **POST** /v1/telemetry/active-deployments | Create a New Active Deployment Telemetry Request



## active_deployment_telemetry_get

> models::ActiveDeploymentTelemetryGetResult active_deployment_telemetry_get(retrieval_key)
Get the Result of an Active Deployment Telemetry Request

Retrieve the results of a telemetry request on active deployment(s) for given IP(s). The score array is sorted from the best to the worse deployment. You can use this to add players on a running deployment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**retrieval_key** | **String** |  | [required] |

### Return type

[**models::ActiveDeploymentTelemetryGetResult**](ActiveDeploymentTelemetryGetResult.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## active_deployment_telemetry_post

> models::ActiveDeploymentTelemetryResponse active_deployment_telemetry_post(payload)
Create a New Active Deployment Telemetry Request

Create a telemetry request to get the best deployment(s) for given IP(s). You can use this to add players on a running deployment. If you set a webhook URL, the result will be sent to it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | [**ActiveDeploymentTelemetryRequest**](ActiveDeploymentTelemetryRequest.md) |  | [required] |

### Return type

[**models::ActiveDeploymentTelemetryResponse**](ActiveDeploymentTelemetryResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

