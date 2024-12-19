# PortMapping

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**external** | Option<**i32**> | The Port to Connect from Internet | [optional]
**internal** | Option<**i32**> | The internal Port of the Container | [optional]
**protocol** | Option<**String**> | The Protocol (i.e. 'TCP') | [optional]
**name** | Option<**String**> | The Name of the port if given, default to internal port in string | [optional]
**tls_upgrade** | Option<**bool**> | If the port require TLS Upgrade | [optional]
**link** | Option<**String**> | link of the port with scheme depending of the protocol | [optional]
**proxy** | Option<**i32**> | Internal Proxy Mapping | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


