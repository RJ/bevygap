# AppVersionPort

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**port** | **i32** | The Port to Expose your service. Port 0 reserved for one-to-one port mapping. See our doc for more information. | 
**protocol** | **String** | Available protocols: TCP, UDP, TCP/UDP, HTTP, HTTPS, WS or WSS | 
**to_check** | Option<**bool**> | If the port must be verified by our port validations | [optional][default to true]
**tls_upgrade** | Option<**bool**> | Enabling with HTTP or WS will inject a sidecar proxy that upgrades the connection with TLS | [optional][default to false]
**name** | Option<**String**> | An optional name for the port for easier handling. Mandatory if using port 0 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


