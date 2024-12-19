# AppVersionPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The Version Name | 
**is_active** | Option<**bool**> | If the Version is active currently in the system | [optional]
**docker_repository** | **String** | The Repository where the image is (i.e. 'harbor.edgegap.com' or 'docker.io') | 
**docker_image** | **String** | The name of your image (i.e. 'edgegap/demo') | 
**docker_tag** | **String** | The tag of your image (i.e. '0.1.2') | 
**private_username** | Option<**String**> | The username to access the docker repository | [optional]
**private_token** | Option<**String**> | The Private Password or Token of the username (We recommend to use a token) | [optional]
**req_cpu** | **i32** | Units of vCPU needed (1024 = 1vcpu) | 
**req_memory** | **i32** | Units of memory in MB needed (1024 = 1GB) | 
**req_video** | Option<**i32**> | Units of GPU needed (1024 = 1 GPU) | [optional]
**max_duration** | Option<**i32**> | The Max duration of the game in minute. 0 means forever. | [optional]
**use_telemetry** | Option<**bool**> | Allow to inject ASA Variables | [optional]
**inject_context_env** | Option<**bool**> | DEPRECATED - The context environment variables are now injected into all deployments by default. No changes are required. (https://docs.edgegap.com/docs/deployment/injected-variables) | [optional]
**whitelisting_active** | Option<**bool**> | ACL Protection is active | [optional]
**force_cache** | Option<**bool**> | Allow faster deployment by caching your container image in every Edge site | [optional]
**cache_min_hour** | Option<**i32**> | Start of the preferred interval for caching your container | [optional]
**cache_max_hour** | Option<**i32**> | End of the preferred interval for caching your container | [optional]
**time_to_deploy** | Option<**i32**> | Estimated maximum time in seconds to deploy, after this time we will consider it not working and retry. | [optional]
**enable_all_locations** | Option<**bool**> | Enable every location available. By enabling this, your request will use every potential location, including those which may require a longer time to deploy. This means that your application could take significantly more time before being ready. We do not recommend this feature for live games. This functionality does not support ACL and Caching at the moment. | [optional]
**session_config** | Option<[**models::AppVersionCreateSessionConfig**](AppVersionCreateSessionConfig.md)> |  | [optional]
**ports** | Option<[**Vec<models::AppVersionPort>**](AppVersionPort.md)> |  | [optional]
**probe** | Option<[**models::AppVersionProbe**](AppVersionProbe.md)> |  | [optional]
**envs** | Option<[**Vec<models::AppVersionEnv>**](AppVersionEnv.md)> |  | [optional]
**verify_image** | Option<**bool**> | By enabling the verify_image option, your image infos (docker_repository, docker_image, docker_tag) will be tested. | [optional][default to false]
**termination_grace_period_seconds** | Option<**i32**> | Termination grace period in seconds after the SIGTERM signal has been sent | [optional]
**endpoint_storage** | Option<**String**> | The name of the endpoint storage to link | [optional]
**build_type** | Option<**String**> | Available Build Types: Production or Development | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


