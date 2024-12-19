# \IpLookupApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**i_p**](IpLookupApi.md#i_p) | **GET** /v1/ip | Get Your Public IP
[**i_p_lookup**](IpLookupApi.md#i_p_lookup) | **GET** /v1/ip/{ip}/lookup | Get an IP's information
[**i_ps_lookup**](IpLookupApi.md#i_ps_lookup) | **POST** /v1/ips/lookup | Get IPs Information in Bulk



## i_p

> models::IpAddressResponse i_p()
Get Your Public IP

Retrieve your public IP address.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::IpAddressResponse**](IPAddressResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## i_p_lookup

> models::IpAddressLookupResponse i_p_lookup(ip)
Get an IP's information

Lookup an IP address and return the associated information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ip** | **String** |  | [required] |

### Return type

[**models::IpAddressLookupResponse**](IPAddressLookupResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## i_ps_lookup

> models::IpAddressesLookupResponse i_ps_lookup(payload)
Get IPs Information in Bulk

Lookup IP addresses and return the associated information. Maximum of 20 IPs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | [**IpAddressesLookupPayload**](IpAddressesLookupPayload.md) |  | [required] |

### Return type

[**models::IpAddressesLookupResponse**](IPAddressesLookupResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

