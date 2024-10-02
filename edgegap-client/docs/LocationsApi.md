# \LocationsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**location_beacon_list**](LocationsApi.md#location_beacon_list) | **GET** /v1/locations/beacons | List All Location Beacons
[**locations_get**](LocationsApi.md#locations_get) | **GET** /v1/locations | List All Locations



## location_beacon_list

> models::LocationBeaconList location_beacon_list()
List All Location Beacons

List all the active location beacons. They can be used to ping them for your matchmaking system. You cannot deploy on beacons.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::LocationBeaconList**](LocationBeaconList.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## locations_get

> models::Locations locations_get(app, version, r#type, tags)
List All Locations

List all the locations available to deploy on. You can specify an application and a version to filter out the locations that don’t have enough resources to deploy this application version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app** | Option<**String**> | The App Name you want to filter with capacity |  |
**version** | Option<**String**> | The Version Name you want to filter with capacity |  |
**r#type** | Option<**String**> | The type of the location |  |
**tags** | Option<**String**> | Gets locations with tags. Set to: \"true\" to have the tags |  |

### Return type

[**models::Locations**](Locations.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

