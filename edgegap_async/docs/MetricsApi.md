# \MetricsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**deployment_metrics_get**](MetricsApi.md#deployment_metrics_get) | **GET** /v1/metrics/deployment/{request_id} | Get a Deployment Metrics



## deployment_metrics_get

> models::MetricsResponse deployment_metrics_get(request_id, start_time, end_time, steps, raw)
Get a Deployment Metrics

Get the metrics for a specific deployment based on the ```start_time```, ```end_time``` and ```steps```. ```raw``` parameter can be set to true to get the raw data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** |  | [required] |
**start_time** | Option<**String**> | Starting time for the query. Default to deployment start time. Should match %Y-%m-%d %H:%M:%S.%f    Example: 2021-07-10 00:00:00.00 |  |
**end_time** | Option<**String**> | End time for the metrics. Default to now.Must be greater than start_time. Should match %Y-%m-%d %H:%M:%S.%f    Example: 2021-07-19 00:00:00.00 |  |
**steps** | Option<**String**> | Steps between each metrics.    Example: 30s, 1m, 5m 10m, 1h  |  |
**raw** | Option<**String**> | If set to true, will return the raw data. Default to false. |  |

### Return type

[**models::MetricsResponse**](MetricsResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

