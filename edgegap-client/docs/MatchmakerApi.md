# \MatchmakerApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_matchmaker**](MatchmakerApi.md#delete_matchmaker) | **DELETE** /v1/aom/matchmaker/{matchmaker_name} | Delete a Matchmaker
[**delete_matchmaker_component**](MatchmakerApi.md#delete_matchmaker_component) | **DELETE** /v1/aom/component/{component_name} | Delete a Matchmaker Component
[**delete_matchmaker_component_env**](MatchmakerApi.md#delete_matchmaker_component_env) | **DELETE** /v1/aom/component/{component_name}/env/{env_key} | Delete a Matchmaker Component ENV
[**delete_matchmaker_managed_release**](MatchmakerApi.md#delete_matchmaker_managed_release) | **DELETE** /v1/aom/matchmaker/{matchmaker_name}/release/managed/{release_version} | Delete a Matchmaker Managed Release
[**delete_matchmaker_release**](MatchmakerApi.md#delete_matchmaker_release) | **DELETE** /v1/aom/matchmaker/{matchmaker_name}/release/{release_version} | Delete a Matchmaker Release
[**delete_matchmaker_release_config**](MatchmakerApi.md#delete_matchmaker_release_config) | **DELETE** /v1/aom/release/config/{config_name} | Delete a Matchmaker Release Config
[**get_component_list**](MatchmakerApi.md#get_component_list) | **GET** /v1/aom/components | List All Matchmaker Components
[**get_envs_list**](MatchmakerApi.md#get_envs_list) | **GET** /v1/aom/component/{component_name}/envs | List All Matchmaker Component ENVs
[**get_matchmaker**](MatchmakerApi.md#get_matchmaker) | **GET** /v1/aom/matchmaker/{matchmaker_name} | Get a Matchmaker
[**get_matchmaker_component**](MatchmakerApi.md#get_matchmaker_component) | **GET** /v1/aom/component/{component_name} | Get a Matchmaker Component
[**get_matchmaker_component_env**](MatchmakerApi.md#get_matchmaker_component_env) | **GET** /v1/aom/component/{component_name}/env/{env_key} | Get a Matchmaker Component ENV
[**get_matchmaker_list**](MatchmakerApi.md#get_matchmaker_list) | **GET** /v1/aom/matchmakers | List All Matchmakers
[**get_matchmaker_managed_release**](MatchmakerApi.md#get_matchmaker_managed_release) | **GET** /v1/aom/matchmaker/{matchmaker_name}/release/managed/{release_version} | Get a Matchmaker Managed Release
[**get_matchmaker_release**](MatchmakerApi.md#get_matchmaker_release) | **GET** /v1/aom/matchmaker/{matchmaker_name}/release/{release_version} | Get a Matchmaker Release
[**get_matchmaker_release_config**](MatchmakerApi.md#get_matchmaker_release_config) | **GET** /v1/aom/release/config/{config_name} | Get a Matchmaker Release Config
[**get_release_configs_list**](MatchmakerApi.md#get_release_configs_list) | **GET** /v1/aom/release/configs | List All Matchmaker Release Configs
[**get_release_list**](MatchmakerApi.md#get_release_list) | **GET** /v1/aom/matchmaker/{matchmaker_name}/releases | List All Matchmaker Releases
[**patch_matchmaker**](MatchmakerApi.md#patch_matchmaker) | **PATCH** /v1/aom/matchmaker/{matchmaker_name} | Update a Matchmaker
[**patch_matchmaker_component**](MatchmakerApi.md#patch_matchmaker_component) | **PATCH** /v1/aom/component/{component_name} | Update a Matchmaker Component
[**patch_matchmaker_component_env**](MatchmakerApi.md#patch_matchmaker_component_env) | **PATCH** /v1/aom/component/{component_name}/env/{env_key} | Update a Matchmaker Component ENV
[**patch_matchmaker_managed_release**](MatchmakerApi.md#patch_matchmaker_managed_release) | **PATCH** /v1/aom/matchmaker/{matchmaker_name}/release/managed/{release_version} | Update a Matchmaker Managed Release
[**patch_matchmaker_release**](MatchmakerApi.md#patch_matchmaker_release) | **PATCH** /v1/aom/matchmaker/{matchmaker_name}/release/{release_version} | Update a Matchmaker Release
[**patch_matchmaker_release_config**](MatchmakerApi.md#patch_matchmaker_release_config) | **PATCH** /v1/aom/release/config/{config_name} | Update a Matchmaker Release Config
[**post_matchmaker**](MatchmakerApi.md#post_matchmaker) | **POST** /v1/aom/matchmaker | Create a Matchmaker
[**post_matchmaker_component**](MatchmakerApi.md#post_matchmaker_component) | **POST** /v1/aom/component | Create a Matchmaker Component
[**post_matchmaker_component_env**](MatchmakerApi.md#post_matchmaker_component_env) | **POST** /v1/aom/component/{component_name}/env | Create a Matchmaker Component ENV
[**post_matchmaker_managed_release**](MatchmakerApi.md#post_matchmaker_managed_release) | **POST** /v1/aom/matchmaker/{matchmaker_name}/release/managed | Create a Matchmaker Managed Release
[**post_matchmaker_release**](MatchmakerApi.md#post_matchmaker_release) | **POST** /v1/aom/matchmaker/{matchmaker_name}/release | Create a Matchmaker Release
[**post_matchmaker_release_config**](MatchmakerApi.md#post_matchmaker_release_config) | **POST** /v1/aom/release/config | Create a Matchmaker Release Config



## delete_matchmaker

> delete_matchmaker(matchmaker_name)
Delete a Matchmaker

Delete a matchmaker.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**matchmaker_name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_matchmaker_component

> delete_matchmaker_component(component_name)
Delete a Matchmaker Component

Delete a matchmaker component. It will not delete the matchmaker.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**component_name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_matchmaker_component_env

> delete_matchmaker_component_env(component_name, env_key)
Delete a Matchmaker Component ENV

Delete a matchmaker component ENV. It will not delete the component or the matchmaker.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**component_name** | **String** |  | [required] |
**env_key** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_matchmaker_managed_release

> delete_matchmaker_managed_release(matchmaker_name, release_version)
Delete a Matchmaker Managed Release

Delete a matchmaker managed release. It will not delete the matchmaker.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**matchmaker_name** | **String** |  | [required] |
**release_version** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_matchmaker_release

> delete_matchmaker_release(matchmaker_name, release_version)
Delete a Matchmaker Release

Delete a matchmaker release.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**matchmaker_name** | **String** |  | [required] |
**release_version** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_matchmaker_release_config

> delete_matchmaker_release_config(config_name)
Delete a Matchmaker Release Config

Delete a matchmaker release config.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_component_list

> models::MatchmakerComponentListResponse get_component_list()
List All Matchmaker Components

List all components for a specific matchmaker.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::MatchmakerComponentListResponse**](MatchmakerComponentListResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_envs_list

> models::MatchmakerComponentEnvListResponse get_envs_list(component_name)
List All Matchmaker Component ENVs

List all ENVs for a specific matchmaker component.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**component_name** | **String** |  | [required] |

### Return type

[**models::MatchmakerComponentEnvListResponse**](MatchmakerComponentEnvListResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_matchmaker

> models::MatchmakerResponse get_matchmaker(matchmaker_name)
Get a Matchmaker

Retrieve a matchmaker.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**matchmaker_name** | **String** |  | [required] |

### Return type

[**models::MatchmakerResponse**](MatchmakerResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_matchmaker_component

> models::MatchmakerComponentResponse get_matchmaker_component(component_name)
Get a Matchmaker Component

Retrieve a matchmaker component.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**component_name** | **String** |  | [required] |

### Return type

[**models::MatchmakerComponentResponse**](MatchmakerComponentResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_matchmaker_component_env

> models::MatchmakerComponentEnvsResponse get_matchmaker_component_env(component_name, env_key)
Get a Matchmaker Component ENV

Retrieve a matchmaker component ENV.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**component_name** | **String** |  | [required] |
**env_key** | **String** |  | [required] |

### Return type

[**models::MatchmakerComponentEnvsResponse**](MatchmakerComponentEnvsResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_matchmaker_list

> models::MatchmakerListResponse get_matchmaker_list()
List All Matchmakers

List all matchmakers.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::MatchmakerListResponse**](MatchmakerListResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_matchmaker_managed_release

> models::MatchmakerManagedReleaseResponse get_matchmaker_managed_release(matchmaker_name, release_version)
Get a Matchmaker Managed Release

Retrieve a matchmaker managed release.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**matchmaker_name** | **String** |  | [required] |
**release_version** | **String** |  | [required] |

### Return type

[**models::MatchmakerManagedReleaseResponse**](MatchmakerManagedReleaseResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_matchmaker_release

> models::MatchmakerReleaseResponse get_matchmaker_release(matchmaker_name, release_version)
Get a Matchmaker Release

Retrieve a matchmaker release.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**matchmaker_name** | **String** |  | [required] |
**release_version** | **String** |  | [required] |

### Return type

[**models::MatchmakerReleaseResponse**](MatchmakerReleaseResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_matchmaker_release_config

> models::MatchmakerReleaseConfigResponse get_matchmaker_release_config(config_name)
Get a Matchmaker Release Config

Get a matchmaker release config.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_name** | **String** |  | [required] |

### Return type

[**models::MatchmakerReleaseConfigResponse**](MatchmakerReleaseConfigResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_release_configs_list

> models::MatchmakerComponentEnvListResponse get_release_configs_list()
List All Matchmaker Release Configs

List all configs for a specific matchmaker release.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::MatchmakerComponentEnvListResponse**](MatchmakerComponentEnvListResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_release_list

> models::MatchmakerComponentEnvListResponse get_release_list(matchmaker_name)
List All Matchmaker Releases

List all releases of a specific matchmaker.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**matchmaker_name** | **String** |  | [required] |

### Return type

[**models::MatchmakerComponentEnvListResponse**](MatchmakerComponentEnvListResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_matchmaker

> models::MatchmakerResponse patch_matchmaker(matchmaker_name, payload)
Update a Matchmaker

Update a matchmaker with new specifications.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**matchmaker_name** | **String** |  | [required] |
**payload** | [**MatchmakerUpdate**](MatchmakerUpdate.md) |  | [required] |

### Return type

[**models::MatchmakerResponse**](MatchmakerResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_matchmaker_component

> models::MatchmakerComponentResponse patch_matchmaker_component(component_name, payload)
Update a Matchmaker Component

Update a matchmaker component with new specifications.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**component_name** | **String** |  | [required] |
**payload** | [**MatchmakerComponentUpdate**](MatchmakerComponentUpdate.md) |  | [required] |

### Return type

[**models::MatchmakerComponentResponse**](MatchmakerComponentResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_matchmaker_component_env

> models::MatchmakerComponentEnvsResponse patch_matchmaker_component_env(component_name, env_key, payload)
Update a Matchmaker Component ENV

Update a matchmaker component ENV.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**component_name** | **String** |  | [required] |
**env_key** | **String** |  | [required] |
**payload** | [**MatchmakerComponentEnvsUpdate**](MatchmakerComponentEnvsUpdate.md) |  | [required] |

### Return type

[**models::MatchmakerComponentEnvsResponse**](MatchmakerComponentEnvsResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_matchmaker_managed_release

> models::MatchmakerManagedReleaseResponse patch_matchmaker_managed_release(matchmaker_name, release_version, payload)
Update a Matchmaker Managed Release

Update a matchmaker managed release.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**matchmaker_name** | **String** |  | [required] |
**release_version** | **String** |  | [required] |
**payload** | [**MatchmakerManagedReleaseUpdate**](MatchmakerManagedReleaseUpdate.md) |  | [required] |

### Return type

[**models::MatchmakerManagedReleaseResponse**](MatchmakerManagedReleaseResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_matchmaker_release

> models::MatchmakerReleaseResponse patch_matchmaker_release(matchmaker_name, release_version, payload)
Update a Matchmaker Release

Update a matchmaker release.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**matchmaker_name** | **String** |  | [required] |
**release_version** | **String** |  | [required] |
**payload** | [**MatchmakerReleaseUpdate**](MatchmakerReleaseUpdate.md) |  | [required] |

### Return type

[**models::MatchmakerReleaseResponse**](MatchmakerReleaseResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_matchmaker_release_config

> models::MatchmakerReleaseConfigResponse patch_matchmaker_release_config(config_name, payload)
Update a Matchmaker Release Config

Update a matchmaker release config.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_name** | **String** |  | [required] |
**payload** | [**MatchmakerReleaseConfigUpdate**](MatchmakerReleaseConfigUpdate.md) |  | [required] |

### Return type

[**models::MatchmakerReleaseConfigResponse**](MatchmakerReleaseConfigResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_matchmaker

> models::MatchmakerResponse post_matchmaker(payload)
Create a Matchmaker

Create a new matchmaker. A matchmaker is a top-level object; you must create child resources to work properly.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | [**MatchmakerCreate**](MatchmakerCreate.md) |  | [required] |

### Return type

[**models::MatchmakerResponse**](MatchmakerResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_matchmaker_component

> models::MatchmakerComponentResponse post_matchmaker_component(payload)
Create a Matchmaker Component

Create a new matchmaker component.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | [**MatchmakerComponentCreate**](MatchmakerComponentCreate.md) |  | [required] |

### Return type

[**models::MatchmakerComponentResponse**](MatchmakerComponentResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_matchmaker_component_env

> models::MatchmakerComponentEnvsResponse post_matchmaker_component_env(component_name, payload)
Create a Matchmaker Component ENV

Create a new matchmaker component ENV.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**component_name** | **String** |  | [required] |
**payload** | [**MatchmakerComponentEnvsCreate**](MatchmakerComponentEnvsCreate.md) |  | [required] |

### Return type

[**models::MatchmakerComponentEnvsResponse**](MatchmakerComponentEnvsResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_matchmaker_managed_release

> models::MatchmakerManagedReleaseResponse post_matchmaker_managed_release(matchmaker_name, payload)
Create a Matchmaker Managed Release

Create a matchmaker managed release.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**matchmaker_name** | **String** |  | [required] |
**payload** | [**MatchmakerManagedReleaseCreate**](MatchmakerManagedReleaseCreate.md) |  | [required] |

### Return type

[**models::MatchmakerManagedReleaseResponse**](MatchmakerManagedReleaseResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_matchmaker_release

> models::MatchmakerReleaseResponse post_matchmaker_release(matchmaker_name, payload)
Create a Matchmaker Release

Create a matchmaker release.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**matchmaker_name** | **String** |  | [required] |
**payload** | [**MatchmakerReleaseCreate**](MatchmakerReleaseCreate.md) |  | [required] |

### Return type

[**models::MatchmakerReleaseResponse**](MatchmakerReleaseResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_matchmaker_release_config

> models::MatchmakerReleaseConfigResponse post_matchmaker_release_config(payload)
Create a Matchmaker Release Config

Create a matchmaker release config.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | [**MatchmakerReleaseConfigCreate**](MatchmakerReleaseConfigCreate.md) |  | [required] |

### Return type

[**models::MatchmakerReleaseConfigResponse**](MatchmakerReleaseConfigResponse.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

