# Rust API client for edgegap_async

<h1 style=\"margin-top: 2rem\">Introduction</h1>
<p>The Edgegap API lets you manage all your resources from standard HTTP requests. We promote using them to automate all your processes that are using Edgegap.</p>
<p>If you have any questions, don't hesitate to contact us via email, or you can also jump on our <a href=\"https://discord.com/invite/GYaHcKR9a5\" target=\"_blank\">Discord</a>. We will be happy to help. Feel free to make features request; we also love those.</p>
<div class=\"theme-doc-markdown markdown\"><h2 class=\"anchor anchorWithStickyNavbar_node_modules-@docusaurus-theme-classic-lib-theme-Heading-styles-module\" id=\"pagination---response\">Pagination - Response<a class=\"hash-link\" href=\"#pagination---response\" title=\"Direct link to heading\">​</a></h2><p>The GET response body can be returned with pagination to avoid requesting all the data at once.</p><p>Pagination object will always be under the  <em>paginator</em> key.</p><div class=\"language-json codeBlockContainer_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-Container-styles-module theme-code-block\" style=\"--prism-color:#bfc7d5; --prism-background-color:#292d3e;\"><div class=\"codeBlockContent_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-Content-styles-module\"><pre tabindex=\"0\" class=\"prism-code language-json codeBlock_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-Content-styles-module thin-scrollbar\"><code class=\"codeBlockLines_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-Content-styles-module\"><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">{</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">  </span><span class=\"token property\">\"number\"</span><span class=\"token operator\" style=\"color: rgb(137, 221, 255);\">:</span><span class=\"token plain\"> </span><span class=\"token string\" style=\"color: rgb(195, 232, 141);\">\"The Current page, default=1\"</span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">,</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">  </span><span class=\"token property\">\"next_page_number\"</span><span class=\"token operator\" style=\"color: rgb(137, 221, 255);\">:</span><span class=\"token plain\"> </span><span class=\"token string\" style=\"color: rgb(195, 232, 141);\">\"The Next page number or null\"</span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">,</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">  </span><span class=\"token property\">\"previous_page_number\"</span><span class=\"token operator\" style=\"color: rgb(137, 221, 255);\">:</span><span class=\"token plain\"> </span><span class=\"token string\" style=\"color: rgb(195, 232, 141);\">\"The Previous page number or null\"</span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">,</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">  </span><span class=\"token property\">\"paginator\"</span><span class=\"token operator\" style=\"color: rgb(137, 221, 255);\">:</span><span class=\"token plain\"> </span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">{</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">    </span><span class=\"token property\">\"num_pages\"</span><span class=\"token operator\" style=\"color: rgb(137, 221, 255);\">:</span><span class=\"token plain\"> </span><span class=\"token string\" style=\"color: rgb(195, 232, 141);\">\"The total numbers of pages\"</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">  </span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">}</span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">,</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">  </span><span class=\"token property\">\"has_next\"</span><span class=\"token operator\" style=\"color: rgb(137, 221, 255);\">:</span><span class=\"token plain\"> </span><span class=\"token string\" style=\"color: rgb(195, 232, 141);\">\"Boolean if there is a next page\"</span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">,</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">  </span><span class=\"token property\">\"has_previous\"</span><span class=\"token operator\" style=\"color: rgb(137, 221, 255);\">:</span><span class=\"token plain\"> </span><span class=\"token string\" style=\"color: rgb(195, 232, 141);\">\"Boolean if there is a previous page\"</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\"></span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">}</span><br></span></code></pre><div class=\"buttonGroup_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-Content-styles-module\"><button type=\"button\" aria-label=\"Copy code to clipboard\" title=\"Copy\" class=\"clean-btn\"><span class=\"copyButtonIcons_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-CopyButton-styles-module\" aria-hidden=\"true\"><svg class=\"copyButtonIcon_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-CopyButton-styles-module\" viewBox=\"0 0 24 24\"><path d=\"M19,21H8V7H19M19,5H8A2,2 0 0,0 6,7V21A2,2 0 0,0 8,23H19A2,2 0 0,0 21,21V7A2,2 0 0,0 19,5M16,1H4A2,2 0 0,0 2,3V17H4V3H16V1Z\"></path></svg><svg class=\"copyButtonSuccessIcon_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-CopyButton-styles-module\" viewBox=\"0 0 24 24\"><path d=\"M21,7L9,19L3.5,13.5L4.91,12.09L9,16.17L19.59,5.59L21,7Z\"></path></svg></span></button></div></div></div><p>Full Body Example:</p><div class=\"language-json codeBlockContainer_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-Container-styles-module theme-code-block\" style=\"--prism-color:#bfc7d5; --prism-background-color:#292d3e;\"><div class=\"codeBlockContent_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-Content-styles-module\"><pre tabindex=\"0\" class=\"prism-code language-json codeBlock_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-Content-styles-module thin-scrollbar\"><code class=\"codeBlockLines_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-Content-styles-module\"><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">{</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">  </span><span class=\"token property\">\"count\"</span><span class=\"token operator\" style=\"color: rgb(137, 221, 255);\">:</span><span class=\"token plain\"> </span><span class=\"token number\" style=\"color: rgb(247, 140, 108);\">100</span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">,</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">  </span><span class=\"token property\">\"data\"</span><span class=\"token operator\" style=\"color: rgb(137, 221, 255);\">:</span><span class=\"token plain\"> </span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">[</span><span class=\"token string\" style=\"color: rgb(195, 232, 141);\">\"value-0\"</span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">,</span><span class=\"token plain\"> </span><span class=\"token string\" style=\"color: rgb(195, 232, 141);\">\"[...]\"</span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">,</span><span class=\"token plain\"> </span><span class=\"token string\" style=\"color: rgb(195, 232, 141);\">\"value-9\"</span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">]</span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">,</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">  </span><span class=\"token property\">\"success\"</span><span class=\"token operator\" style=\"color: rgb(137, 221, 255);\">:</span><span class=\"token plain\"> </span><span class=\"token boolean\" style=\"color: rgb(255, 88, 116);\">true</span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">,</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">  </span><span class=\"token property\">\"pagination\"</span><span class=\"token operator\" style=\"color: rgb(137, 221, 255);\">:</span><span class=\"token plain\"> </span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">{</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">    </span><span class=\"token property\">\"number\"</span><span class=\"token operator\" style=\"color: rgb(137, 221, 255);\">:</span><span class=\"token plain\"> </span><span class=\"token number\" style=\"color: rgb(247, 140, 108);\">1</span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">,</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">    </span><span class=\"token property\">\"next_page_number\"</span><span class=\"token operator\" style=\"color: rgb(137, 221, 255);\">:</span><span class=\"token plain\"> </span><span class=\"token number\" style=\"color: rgb(247, 140, 108);\">2</span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">,</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">    </span><span class=\"token property\">\"previous_page_number\"</span><span class=\"token operator\" style=\"color: rgb(137, 221, 255);\">:</span><span class=\"token plain\"> </span><span class=\"token null keyword\" style=\"font-style: italic;\">null</span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">,</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">    </span><span class=\"token property\">\"paginator\"</span><span class=\"token operator\" style=\"color: rgb(137, 221, 255);\">:</span><span class=\"token plain\"> </span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">{</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">      </span><span class=\"token property\">\"num_pages\"</span><span class=\"token operator\" style=\"color: rgb(137, 221, 255);\">:</span><span class=\"token plain\"> </span><span class=\"token number\" style=\"color: rgb(247, 140, 108);\">10</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">    </span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">}</span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">,</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">    </span><span class=\"token property\">\"has_next\"</span><span class=\"token operator\" style=\"color: rgb(137, 221, 255);\">:</span><span class=\"token plain\"> </span><span class=\"token boolean\" style=\"color: rgb(255, 88, 116);\">true</span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">,</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">    </span><span class=\"token property\">\"has_previous\"</span><span class=\"token operator\" style=\"color: rgb(137, 221, 255);\">:</span><span class=\"token plain\"> </span><span class=\"token boolean\" style=\"color: rgb(255, 88, 116);\">false</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">  </span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">}</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\"></span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">}</span><br></span></code></pre><div class=\"buttonGroup_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-Content-styles-module\"><button type=\"button\" aria-label=\"Copy code to clipboard\" title=\"Copy\" class=\"clean-btn\"><span class=\"copyButtonIcons_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-CopyButton-styles-module\" aria-hidden=\"true\"><svg class=\"copyButtonIcon_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-CopyButton-styles-module\" viewBox=\"0 0 24 24\"><path d=\"M19,21H8V7H19M19,5H8A2,2 0 0,0 6,7V21A2,2 0 0,0 8,23H19A2,2 0 0,0 21,21V7A2,2 0 0,0 19,5M16,1H4A2,2 0 0,0 2,3V17H4V3H16V1Z\"></path></svg><svg class=\"copyButtonSuccessIcon_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-CopyButton-styles-module\" viewBox=\"0 0 24 24\"><path d=\"M21,7L9,19L3.5,13.5L4.91,12.09L9,16.17L19.59,5.59L21,7Z\"></path></svg></span></button></div></div></div><h2 class=\"anchor anchorWithStickyNavbar_node_modules-@docusaurus-theme-classic-lib-theme-Heading-styles-module\" id=\"pagination---parameters\">Pagination - Parameters<a class=\"hash-link\" href=\"#pagination---parameters\" title=\"Direct link to heading\">​</a></h2><p>You can add those values to manipulate the pagination object in the URL Parameters.</p><ul class=\"\"><li>page</li><li>limit</li></ul><p>Example:</p><div class=\"language-text codeBlockContainer_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-Container-styles-module theme-code-block\" style=\"--prism-color:#bfc7d5; --prism-background-color:#292d3e;\"><div class=\"codeBlockContent_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-Content-styles-module\"><pre tabindex=\"0\" class=\"prism-code language-text codeBlock_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-Content-styles-module thin-scrollbar\"><code class=\"codeBlockLines_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-Content-styles-module\"><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\"># To get the second page</span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">GET - https://api.edgegap.com/v1/apps?page=2</span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\" style=\"display: inline-block;\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\"># To change the count of element in one page (20/page)</span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">GET - https://api.edgegap.com/v1/apps?limit=20</span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\" style=\"display: inline-block;\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\"># You can mix those (20/page, second page)</span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">GET - https://api.edgegap.com/v1/apps?page=2&amp;limit=20</span><br></span></code></pre><div class=\"buttonGroup_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-Content-styles-module\"><button type=\"button\" aria-label=\"Copy code to clipboard\" title=\"Copy\" class=\"clean-btn\"><span class=\"copyButtonIcons_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-CopyButton-styles-module\" aria-hidden=\"true\"><svg class=\"copyButtonIcon_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-CopyButton-styles-module\" viewBox=\"0 0 24 24\"><path d=\"M19,21H8V7H19M19,5H8A2,2 0 0,0 6,7V21A2,2 0 0,0 8,23H19A2,2 0 0,0 21,21V7A2,2 0 0,0 19,5M16,1H4A2,2 0 0,0 2,3V17H4V3H16V1Z\"></path></svg><svg class=\"copyButtonSuccessIcon_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-CopyButton-styles-module\" viewBox=\"0 0 24 24\"><path d=\"M21,7L9,19L3.5,13.5L4.91,12.09L9,16.17L19.59,5.59L21,7Z\"></path></svg></span></button></div></div></div></div>

For more information, please visit [https://edgegap.com](https://edgegap.com)

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.0.0
- Package version: 0.1.0
- Generator version: 7.11.0-SNAPSHOT
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `edgegap_async` and add the following to `Cargo.toml` under `[dependencies]`:

```
edgegap_async = { path = "./edgegap_async" }
```

## Documentation for API Endpoints

All URIs are relative to *http://localhost*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*ApplicationsApi* | [**app_version_delete**](docs/ApplicationsApi.md#app_version_delete) | **DELETE** /v1/app/{app_name}/version/{version_name} | Delete an Application Version
*ApplicationsApi* | [**app_version_get**](docs/ApplicationsApi.md#app_version_get) | **GET** /v1/app/{app_name}/version/{version_name} | Get an Application Version
*ApplicationsApi* | [**app_version_post**](docs/ApplicationsApi.md#app_version_post) | **POST** /v1/app/{app_name}/version | Create a New Application Version
*ApplicationsApi* | [**app_version_whitelist_entry_delete**](docs/ApplicationsApi.md#app_version_whitelist_entry_delete) | **DELETE** /v1/app/{app_name}/version/{version_name}/whitelist/{entry_id} | Delete an ACL Entry
*ApplicationsApi* | [**app_version_whitelist_entry_get**](docs/ApplicationsApi.md#app_version_whitelist_entry_get) | **GET** /v1/app/{app_name}/version/{version_name}/whitelist/{entry_id} | Get an ACL Entry
*ApplicationsApi* | [**app_version_whitelist_get**](docs/ApplicationsApi.md#app_version_whitelist_get) | **GET** /v1/app/{app_name}/version/{version_name}/whitelist | List All ACL Entries for an Application Version
*ApplicationsApi* | [**app_version_whitelist_post**](docs/ApplicationsApi.md#app_version_whitelist_post) | **POST** /v1/app/{app_name}/version/{version_name}/whitelist | Create an ACL Entry
*ApplicationsApi* | [**app_versions_get**](docs/ApplicationsApi.md#app_versions_get) | **GET** /v1/app/{app_name}/versions | List All Versions for an Application
*ApplicationsApi* | [**app_versions_patch**](docs/ApplicationsApi.md#app_versions_patch) | **PATCH** /v1/app/{app_name}/version/{version_name} | Update an Application Version
*ApplicationsApi* | [**application_delete**](docs/ApplicationsApi.md#application_delete) | **DELETE** /v1/app/{app_name} | Delete an Application
*ApplicationsApi* | [**application_get**](docs/ApplicationsApi.md#application_get) | **GET** /v1/app/{app_name} | Get an Application
*ApplicationsApi* | [**application_patch**](docs/ApplicationsApi.md#application_patch) | **PATCH** /v1/app/{app_name} | Update an Application
*ApplicationsApi* | [**application_post**](docs/ApplicationsApi.md#application_post) | **POST** /v1/app | Create a New Application
*ApplicationsApi* | [**applications_get**](docs/ApplicationsApi.md#applications_get) | **GET** /v1/apps | List All Applications
*ContainerRegistryApi* | [**image_tag_delete**](docs/ContainerRegistryApi.md#image_tag_delete) | **DELETE** /v1/container-registry/images/{image_name}/tags/{tag_name} | Delete Tag For a Registry Image
*ContainerRegistryApi* | [**registry_image_tag_list**](docs/ContainerRegistryApi.md#registry_image_tag_list) | **GET** /v1/container-registry/images/{image_name}/tags | List All Tags for a Registry Image
*ContextApi* | [**context_create_deployment_tag**](docs/ContextApi.md#context_create_deployment_tag) | **POST** /v1/context/{request_id}/{security_number}/tags | Create a Tag for a Running Deployment
*ContextApi* | [**context_delete_deployment_tag**](docs/ContextApi.md#context_delete_deployment_tag) | **DELETE** /v1/context/{request_id}/{security_number}/tags/{tag_name} | Delete a Tag from a Running Deployment
*ContextApi* | [**context_get**](docs/ContextApi.md#context_get) | **GET** /v1/context/{request_id}/{security_number} | Get the Context of a Deployment
*DeploymentTagsApi* | [**deployments_tags_create**](docs/DeploymentTagsApi.md#deployments_tags_create) | **POST** /v1/deployments/{request_id}/tags | Create Tag for a Deployment
*DeploymentTagsApi* | [**deployments_tags_delete**](docs/DeploymentTagsApi.md#deployments_tags_delete) | **DELETE** /v1/deployments/{request_id}/tags/{tag_name} | Delete Tag for a Deployment
*DeploymentTagsApi* | [**deployments_tags_list**](docs/DeploymentTagsApi.md#deployments_tags_list) | **GET** /v1/deployments/{request_id}/tags | List tags for a Deployment
*DeploymentTagsApi* | [**deployments_tags_read**](docs/DeploymentTagsApi.md#deployments_tags_read) | **GET** /v1/deployments/{request_id}/tags/{tag_name} | Get tag for a Deployment
*DeploymentTagsApi* | [**deployments_tags_update**](docs/DeploymentTagsApi.md#deployments_tags_update) | **PATCH** /v1/deployments/{request_id}/tags/{tag_name} | Update Tag for a Deployment
*DeploymentsApi* | [**deploy**](docs/DeploymentsApi.md#deploy) | **POST** /v1/deploy | Create a Deployment
*DeploymentsApi* | [**deployment_delete**](docs/DeploymentsApi.md#deployment_delete) | **DELETE** /v1/stop/{request_id} | Delete a Deployment
*DeploymentsApi* | [**deployment_get_logs**](docs/DeploymentsApi.md#deployment_get_logs) | **GET** /v1/deployment/{request_id}/container-logs | Get Deployment Container Logs
*DeploymentsApi* | [**deployment_status_get**](docs/DeploymentsApi.md#deployment_status_get) | **GET** /v1/status/{request_id} | Get a Deployment Status and Information
*DeploymentsApi* | [**deployment_update**](docs/DeploymentsApi.md#deployment_update) | **PATCH** /v1/deployments/{request_id} | Updates properties of a deployment
*DeploymentsApi* | [**deployments_available**](docs/DeploymentsApi.md#deployments_available) | **POST** /v1/deployments:available | Deployments with Available Sockets
*DeploymentsApi* | [**deployments_bulk_delete**](docs/DeploymentsApi.md#deployments_bulk_delete) | **POST** /v1/deployments/bulk-stop | Delete Deployments in Bulk
*DeploymentsApi* | [**deployments_get**](docs/DeploymentsApi.md#deployments_get) | **GET** /v1/deployments | List All Deployments
*DeploymentsApi* | [**self_deployment_delete**](docs/DeploymentsApi.md#self_deployment_delete) | **DELETE** /v1/self/stop/{request_id}/{access_point_id} | Delete a Deployment from inside the container
*EndpointStorageApi* | [**endpoint_create**](docs/EndpointStorageApi.md#endpoint_create) | **POST** /v1/storage/endpoint | Create a New Endpoint Storage
*EndpointStorageApi* | [**endpoint_delete**](docs/EndpointStorageApi.md#endpoint_delete) | **DELETE** /v1/storage/endpoint/{endpoint_name} | Delete an Endpoint Storage
*EndpointStorageApi* | [**endpoint_get**](docs/EndpointStorageApi.md#endpoint_get) | **GET** /v1/storage/endpoint/{endpoint_name} | Get an Endpoint Storage
*EndpointStorageApi* | [**endpoint_update**](docs/EndpointStorageApi.md#endpoint_update) | **PATCH** /v1/storage/endpoint/{endpoint_name} | Update an Endpoint Storage
*EndpointStorageApi* | [**endpoints_list**](docs/EndpointStorageApi.md#endpoints_list) | **GET** /v1/storage/endpoints | List All Endpoint Storage
*EndpointStorageApi* | [**pull_profile_create**](docs/EndpointStorageApi.md#pull_profile_create) | **POST** /v1/storage/endpoint/{endpoint_name}/pull-profile | Create a New Pull Profile
*EndpointStorageApi* | [**pull_profile_delete**](docs/EndpointStorageApi.md#pull_profile_delete) | **DELETE** /v1/storage/endpoint/{endpoint_name}/pull-profile/{pull_profile_name} | Delete a Pull Profile
*EndpointStorageApi* | [**pull_profile_get**](docs/EndpointStorageApi.md#pull_profile_get) | **GET** /v1/storage/endpoint/{endpoint_name}/pull-profile/{pull_profile_name} | Get a Pull Profile
*EndpointStorageApi* | [**pull_profile_link_app_version**](docs/EndpointStorageApi.md#pull_profile_link_app_version) | **PUT** /v1/storage/endpoint/{endpoint_name}/pull-profile/{pull_profile_name}/app/{app_name}/version/{version_name} | Link a Pull Profile to an Application Version
*EndpointStorageApi* | [**pull_profile_list**](docs/EndpointStorageApi.md#pull_profile_list) | **GET** /v1/storage/endpoint/{endpoint_name}/pull-profiles | List All Pull Profile of an Endpoint Storage
*EndpointStorageApi* | [**pull_profile_unlink_app_version**](docs/EndpointStorageApi.md#pull_profile_unlink_app_version) | **DELETE** /v1/storage/endpoint/{endpoint_name}/pull-profile/{pull_profile_name}/app/{app_name}/version/{version_name} | Unlink a Pull Profile From an Application Version
*EndpointStorageApi* | [**pull_profile_update**](docs/EndpointStorageApi.md#pull_profile_update) | **PATCH** /v1/storage/endpoint/{endpoint_name}/pull-profile/{pull_profile_name} | Update a Pull Profile
*FleetsApi* | [**fleet_create**](docs/FleetsApi.md#fleet_create) | **POST** /v1/fleet | Create a Fleet
*FleetsApi* | [**fleet_delete**](docs/FleetsApi.md#fleet_delete) | **DELETE** /v1/fleet/{fleet_name} | Delete a Fleet
*FleetsApi* | [**fleet_get**](docs/FleetsApi.md#fleet_get) | **GET** /v1/fleet/{fleet_name} | Get a Fleet
*FleetsApi* | [**fleet_link_app_version**](docs/FleetsApi.md#fleet_link_app_version) | **PUT** /v1/fleet/{fleet_name}/app/{app_name}/version/{version_name} | Link an Application Version to a Fleet
*FleetsApi* | [**fleet_policies_create**](docs/FleetsApi.md#fleet_policies_create) | **POST** /v1/fleet/{fleet_name}/policies | Create a Fleet Policy
*FleetsApi* | [**fleet_policies_delete**](docs/FleetsApi.md#fleet_policies_delete) | **DELETE** /v1/fleet/{fleet_name}/policies/{policy_name} | Delete a Policy
*FleetsApi* | [**fleet_policies_get**](docs/FleetsApi.md#fleet_policies_get) | **GET** /v1/fleet/{fleet_name}/policies/{policy_name} | Get a Policy
*FleetsApi* | [**fleet_policies_list**](docs/FleetsApi.md#fleet_policies_list) | **GET** /v1/fleet/{fleet_name}/policies | List All Policies of a Fleet
*FleetsApi* | [**fleet_policies_update**](docs/FleetsApi.md#fleet_policies_update) | **PATCH** /v1/fleet/{fleet_name}/policies/{policy_name} | Update a Policy
*FleetsApi* | [**fleet_unlink_app_version**](docs/FleetsApi.md#fleet_unlink_app_version) | **DELETE** /v1/fleet/{fleet_name}/app/{app_name}/version/{version_name} | Unlink an Application Version From a Fleet
*FleetsApi* | [**fleet_update**](docs/FleetsApi.md#fleet_update) | **PATCH** /v1/fleet/{fleet_name} | Update a Fleet
*FleetsApi* | [**fleets**](docs/FleetsApi.md#fleets) | **GET** /v1/fleets | List All Fleets
*IpLookupApi* | [**i_p**](docs/IpLookupApi.md#i_p) | **GET** /v1/ip | Get Your Public IP
*IpLookupApi* | [**i_p_lookup**](docs/IpLookupApi.md#i_p_lookup) | **GET** /v1/ip/{ip}/lookup | Get an IP's information
*IpLookupApi* | [**i_ps_lookup**](docs/IpLookupApi.md#i_ps_lookup) | **POST** /v1/ips/lookup | Get IPs Information in Bulk
*LobbiesApi* | [**lobby_create**](docs/LobbiesApi.md#lobby_create) | **POST** /v1/lobbies | Create a Lobby
*LobbiesApi* | [**lobby_delete**](docs/LobbiesApi.md#lobby_delete) | **DELETE** /v1/lobbies/{lobby_name} | Delete a Lobby
*LobbiesApi* | [**lobby_deploy**](docs/LobbiesApi.md#lobby_deploy) | **POST** /v1/lobbies:deploy | Deploy a Lobby
*LobbiesApi* | [**lobby_get**](docs/LobbiesApi.md#lobby_get) | **GET** /v1/lobbies/{lobby_name} | Get a Lobby
*LobbiesApi* | [**lobby_list**](docs/LobbiesApi.md#lobby_list) | **GET** /v1/lobbies | List All Lobbies
*LobbiesApi* | [**lobby_terminate**](docs/LobbiesApi.md#lobby_terminate) | **POST** /v1/lobbies:terminate | Terminate a Lobby
*LocationsApi* | [**location_beacon_list**](docs/LocationsApi.md#location_beacon_list) | **GET** /v1/locations/beacons | List All Location Beacons
*LocationsApi* | [**locations_get**](docs/LocationsApi.md#locations_get) | **GET** /v1/locations | List All Locations
*MatchmakerApi* | [**delete_matchmaker**](docs/MatchmakerApi.md#delete_matchmaker) | **DELETE** /v1/aom/matchmaker/{matchmaker_name} | Delete a Matchmaker
*MatchmakerApi* | [**delete_matchmaker_component**](docs/MatchmakerApi.md#delete_matchmaker_component) | **DELETE** /v1/aom/component/{component_name} | Delete a Matchmaker Component
*MatchmakerApi* | [**delete_matchmaker_component_env**](docs/MatchmakerApi.md#delete_matchmaker_component_env) | **DELETE** /v1/aom/component/{component_name}/env/{env_key} | Delete a Matchmaker Component ENV
*MatchmakerApi* | [**delete_matchmaker_managed_release**](docs/MatchmakerApi.md#delete_matchmaker_managed_release) | **DELETE** /v1/aom/matchmaker/{matchmaker_name}/release/managed/{release_version} | Delete a Matchmaker Managed Release
*MatchmakerApi* | [**delete_matchmaker_release**](docs/MatchmakerApi.md#delete_matchmaker_release) | **DELETE** /v1/aom/matchmaker/{matchmaker_name}/release/{release_version} | Delete a Matchmaker Release
*MatchmakerApi* | [**delete_matchmaker_release_config**](docs/MatchmakerApi.md#delete_matchmaker_release_config) | **DELETE** /v1/aom/release/config/{config_name} | Delete a Matchmaker Release Config
*MatchmakerApi* | [**get_component_list**](docs/MatchmakerApi.md#get_component_list) | **GET** /v1/aom/components | List All Matchmaker Components
*MatchmakerApi* | [**get_envs_list**](docs/MatchmakerApi.md#get_envs_list) | **GET** /v1/aom/component/{component_name}/envs | List All Matchmaker Component ENVs
*MatchmakerApi* | [**get_matchmaker**](docs/MatchmakerApi.md#get_matchmaker) | **GET** /v1/aom/matchmaker/{matchmaker_name} | Get a Matchmaker
*MatchmakerApi* | [**get_matchmaker_component**](docs/MatchmakerApi.md#get_matchmaker_component) | **GET** /v1/aom/component/{component_name} | Get a Matchmaker Component
*MatchmakerApi* | [**get_matchmaker_component_env**](docs/MatchmakerApi.md#get_matchmaker_component_env) | **GET** /v1/aom/component/{component_name}/env/{env_key} | Get a Matchmaker Component ENV
*MatchmakerApi* | [**get_matchmaker_list**](docs/MatchmakerApi.md#get_matchmaker_list) | **GET** /v1/aom/matchmakers | List All Matchmakers
*MatchmakerApi* | [**get_matchmaker_managed_release**](docs/MatchmakerApi.md#get_matchmaker_managed_release) | **GET** /v1/aom/matchmaker/{matchmaker_name}/release/managed/{release_version} | Get a Matchmaker Managed Release
*MatchmakerApi* | [**get_matchmaker_release**](docs/MatchmakerApi.md#get_matchmaker_release) | **GET** /v1/aom/matchmaker/{matchmaker_name}/release/{release_version} | Get a Matchmaker Release
*MatchmakerApi* | [**get_matchmaker_release_config**](docs/MatchmakerApi.md#get_matchmaker_release_config) | **GET** /v1/aom/release/config/{config_name} | Get a Matchmaker Release Config
*MatchmakerApi* | [**get_release_configs_list**](docs/MatchmakerApi.md#get_release_configs_list) | **GET** /v1/aom/release/configs | List All Matchmaker Release Configs
*MatchmakerApi* | [**get_release_list**](docs/MatchmakerApi.md#get_release_list) | **GET** /v1/aom/matchmaker/{matchmaker_name}/releases | List All Matchmaker Releases
*MatchmakerApi* | [**patch_matchmaker**](docs/MatchmakerApi.md#patch_matchmaker) | **PATCH** /v1/aom/matchmaker/{matchmaker_name} | Update a Matchmaker
*MatchmakerApi* | [**patch_matchmaker_component**](docs/MatchmakerApi.md#patch_matchmaker_component) | **PATCH** /v1/aom/component/{component_name} | Update a Matchmaker Component
*MatchmakerApi* | [**patch_matchmaker_component_env**](docs/MatchmakerApi.md#patch_matchmaker_component_env) | **PATCH** /v1/aom/component/{component_name}/env/{env_key} | Update a Matchmaker Component ENV
*MatchmakerApi* | [**patch_matchmaker_managed_release**](docs/MatchmakerApi.md#patch_matchmaker_managed_release) | **PATCH** /v1/aom/matchmaker/{matchmaker_name}/release/managed/{release_version} | Update a Matchmaker Managed Release
*MatchmakerApi* | [**patch_matchmaker_release**](docs/MatchmakerApi.md#patch_matchmaker_release) | **PATCH** /v1/aom/matchmaker/{matchmaker_name}/release/{release_version} | Update a Matchmaker Release
*MatchmakerApi* | [**patch_matchmaker_release_config**](docs/MatchmakerApi.md#patch_matchmaker_release_config) | **PATCH** /v1/aom/release/config/{config_name} | Update a Matchmaker Release Config
*MatchmakerApi* | [**post_matchmaker**](docs/MatchmakerApi.md#post_matchmaker) | **POST** /v1/aom/matchmaker | Create a Matchmaker
*MatchmakerApi* | [**post_matchmaker_component**](docs/MatchmakerApi.md#post_matchmaker_component) | **POST** /v1/aom/component | Create a Matchmaker Component
*MatchmakerApi* | [**post_matchmaker_component_env**](docs/MatchmakerApi.md#post_matchmaker_component_env) | **POST** /v1/aom/component/{component_name}/env | Create a Matchmaker Component ENV
*MatchmakerApi* | [**post_matchmaker_managed_release**](docs/MatchmakerApi.md#post_matchmaker_managed_release) | **POST** /v1/aom/matchmaker/{matchmaker_name}/release/managed | Create a Matchmaker Managed Release
*MatchmakerApi* | [**post_matchmaker_release**](docs/MatchmakerApi.md#post_matchmaker_release) | **POST** /v1/aom/matchmaker/{matchmaker_name}/release | Create a Matchmaker Release
*MatchmakerApi* | [**post_matchmaker_release_config**](docs/MatchmakerApi.md#post_matchmaker_release_config) | **POST** /v1/aom/release/config | Create a Matchmaker Release Config
*MetricsApi* | [**deployment_metrics_get**](docs/MetricsApi.md#deployment_metrics_get) | **GET** /v1/metrics/deployment/{request_id} | Get a Deployment Metrics
*MonitoringApi* | [**monitoring**](docs/MonitoringApi.md#monitoring) | **GET** /monitor | 
*RelaysApi* | [**relay_session_create**](docs/RelaysApi.md#relay_session_create) | **POST** /v1/relays/sessions | Create a Relay Session
*RelaysApi* | [**relay_session_delete**](docs/RelaysApi.md#relay_session_delete) | **DELETE** /v1/relays/sessions/{session_id} | Delete a Relay Session
*RelaysApi* | [**relay_session_get**](docs/RelaysApi.md#relay_session_get) | **GET** /v1/relays/sessions/{session_id} | Get a Relay Session
*RelaysApi* | [**relay_session_list**](docs/RelaysApi.md#relay_session_list) | **GET** /v1/relays/sessions | List all Relay Sessions
*RelaysApi* | [**relay_user_authorize**](docs/RelaysApi.md#relay_user_authorize) | **POST** /v1/relays/sessions:authorize-user | Authorize a user on a Relay Session
*RelaysApi* | [**relay_user_revoke**](docs/RelaysApi.md#relay_user_revoke) | **POST** /v1/relays/sessions:revoke-user | Remove a user on a Relay Session
*SessionsApi* | [**delete_users_session**](docs/SessionsApi.md#delete_users_session) | **DELETE** /v1/session/{session_id}/users | Delete Users From a Session
*SessionsApi* | [**get_session**](docs/SessionsApi.md#get_session) | **GET** /v1/session/{session_id} | Get a Session
*SessionsApi* | [**get_users_session**](docs/SessionsApi.md#get_users_session) | **GET** /v1/session/{session_id}/users | List Users of a Session
*SessionsApi* | [**list_sessions**](docs/SessionsApi.md#list_sessions) | **GET** /v1/sessions | List All Sessions
*SessionsApi* | [**put_users_session**](docs/SessionsApi.md#put_users_session) | **PUT** /v1/session/{session_id}/users | Put Users in a Session
*SessionsApi* | [**session_delete**](docs/SessionsApi.md#session_delete) | **DELETE** /v1/session/{session_id} | Delete a Session
*SessionsApi* | [**session_post**](docs/SessionsApi.md#session_post) | **POST** /v1/session | Create a Session
*SessionsApi* | [**sessions_bulk_stop**](docs/SessionsApi.md#sessions_bulk_stop) | **POST** /v1/sessions/bulk-stop | Delete Sessions in Bulk
*TelemetryApi* | [**active_deployment_telemetry_get**](docs/TelemetryApi.md#active_deployment_telemetry_get) | **GET** /v1/telemetry/active-deployments/{retrieval_key} | Get the Result of an Active Deployment Telemetry Request
*TelemetryApi* | [**active_deployment_telemetry_post**](docs/TelemetryApi.md#active_deployment_telemetry_post) | **POST** /v1/telemetry/active-deployments | Create a New Active Deployment Telemetry Request


## Documentation For Models

 - [ActiveDeploymentTelemetryGetResult](docs/ActiveDeploymentTelemetryGetResult.md)
 - [ActiveDeploymentTelemetryRequest](docs/ActiveDeploymentTelemetryRequest.md)
 - [ActiveDeploymentTelemetryResponse](docs/ActiveDeploymentTelemetryResponse.md)
 - [ActiveDeploymentTelemetryScore](docs/ActiveDeploymentTelemetryScore.md)
 - [ApiModelContainercrashdata](docs/ApiModelContainercrashdata.md)
 - [ApiModelContainerlogs](docs/ApiModelContainerlogs.md)
 - [ApiModelDeploymentfilter](docs/ApiModelDeploymentfilter.md)
 - [ApiModelLocation](docs/ApiModelLocation.md)
 - [ApiModelLocationbeacon](docs/ApiModelLocationbeacon.md)
 - [ApiModelRegistryartifacttagdeleteresponse](docs/ApiModelRegistryartifacttagdeleteresponse.md)
 - [ApiModelTagdeleteartifact](docs/ApiModelTagdeleteartifact.md)
 - [AppVersionCreateResponse](docs/AppVersionCreateResponse.md)
 - [AppVersionCreateSessionConfig](docs/AppVersionCreateSessionConfig.md)
 - [AppVersionDelete](docs/AppVersionDelete.md)
 - [AppVersionEnv](docs/AppVersionEnv.md)
 - [AppVersionList](docs/AppVersionList.md)
 - [AppVersionPayload](docs/AppVersionPayload.md)
 - [AppVersionPort](docs/AppVersionPort.md)
 - [AppVersionProbe](docs/AppVersionProbe.md)
 - [AppVersionUpdatePayload](docs/AppVersionUpdatePayload.md)
 - [AppVersionUpdateResponse](docs/AppVersionUpdateResponse.md)
 - [AppVersionUpdateSessionConfig](docs/AppVersionUpdateSessionConfig.md)
 - [AppVersionWhitelistEntry](docs/AppVersionWhitelistEntry.md)
 - [AppVersionWhitelistEntryPayload](docs/AppVersionWhitelistEntryPayload.md)
 - [AppVersionWhitelistEntrySuccess](docs/AppVersionWhitelistEntrySuccess.md)
 - [AppVersionWhitelistResponse](docs/AppVersionWhitelistResponse.md)
 - [Application](docs/Application.md)
 - [ApplicationPatch](docs/ApplicationPatch.md)
 - [ApplicationPost](docs/ApplicationPost.md)
 - [Applications](docs/Applications.md)
 - [ArtifactPayload](docs/ArtifactPayload.md)
 - [BaseModel](docs/BaseModel.md)
 - [ClientRelayPort](docs/ClientRelayPort.md)
 - [ComponentCredentials](docs/ComponentCredentials.md)
 - [ContainerLogStorageModel](docs/ContainerLogStorageModel.md)
 - [ContextCreateDeploymentTagRequest](docs/ContextCreateDeploymentTagRequest.md)
 - [ContextDeploymentTagResponse](docs/ContextDeploymentTagResponse.md)
 - [Delete](docs/Delete.md)
 - [DeleteRequestReceived](docs/DeleteRequestReceived.md)
 - [DeployEnvModel](docs/DeployEnvModel.md)
 - [DeployModel](docs/DeployModel.md)
 - [Deployment](docs/Deployment.md)
 - [DeploymentAvailable](docs/DeploymentAvailable.md)
 - [DeploymentAvailablePayload](docs/DeploymentAvailablePayload.md)
 - [DeploymentAvailableResponse](docs/DeploymentAvailableResponse.md)
 - [DeploymentBulkStopFiltersPayload](docs/DeploymentBulkStopFiltersPayload.md)
 - [DeploymentBulkStopPayload](docs/DeploymentBulkStopPayload.md)
 - [DeploymentBulkStopResponse](docs/DeploymentBulkStopResponse.md)
 - [DeploymentListData](docs/DeploymentListData.md)
 - [DeploymentLocation](docs/DeploymentLocation.md)
 - [DeploymentSessionContext](docs/DeploymentSessionContext.md)
 - [DeploymentStopResponse](docs/DeploymentStopResponse.md)
 - [DeploymentTagListResponse](docs/DeploymentTagListResponse.md)
 - [DeploymentTagPayload](docs/DeploymentTagPayload.md)
 - [DeploymentTagResponse](docs/DeploymentTagResponse.md)
 - [DeploymentUpdatePayload](docs/DeploymentUpdatePayload.md)
 - [DeploymentUpdateResponse](docs/DeploymentUpdateResponse.md)
 - [Deployments](docs/Deployments.md)
 - [EndpointStorageDeleteResponse](docs/EndpointStorageDeleteResponse.md)
 - [EndpointStorageGetResponse](docs/EndpointStorageGetResponse.md)
 - [EndpointStorageListResponse](docs/EndpointStorageListResponse.md)
 - [EndpointStoragePatchPayload](docs/EndpointStoragePatchPayload.md)
 - [EndpointStoragePatchResponse](docs/EndpointStoragePatchResponse.md)
 - [EndpointStoragePostPayload](docs/EndpointStoragePostPayload.md)
 - [EndpointStoragePostResponse](docs/EndpointStoragePostResponse.md)
 - [Error](docs/Error.md)
 - [FleetDeleteResponse](docs/FleetDeleteResponse.md)
 - [FleetGetResponse](docs/FleetGetResponse.md)
 - [FleetList](docs/FleetList.md)
 - [FleetPatchPayload](docs/FleetPatchPayload.md)
 - [FleetPatchResponse](docs/FleetPatchResponse.md)
 - [FleetPoliciesGetResponse](docs/FleetPoliciesGetResponse.md)
 - [FleetPoliciesPatchPayload](docs/FleetPoliciesPatchPayload.md)
 - [FleetPoliciesPostPayload](docs/FleetPoliciesPostPayload.md)
 - [FleetPoliciesPostResponse](docs/FleetPoliciesPostResponse.md)
 - [FleetPostPayload](docs/FleetPostPayload.md)
 - [FleetPostResponse](docs/FleetPostResponse.md)
 - [GeoIpListModel](docs/GeoIpListModel.md)
 - [HorizontalScalerAppVersionLink](docs/HorizontalScalerAppVersionLink.md)
 - [HorizontalScalerConstraintList](docs/HorizontalScalerConstraintList.md)
 - [ImageTagList](docs/ImageTagList.md)
 - [ImageTagPayload](docs/ImageTagPayload.md)
 - [IpAddressLookupLocation](docs/IpAddressLookupLocation.md)
 - [IpAddressLookupLocationContinent](docs/IpAddressLookupLocationContinent.md)
 - [IpAddressLookupLocationCountry](docs/IpAddressLookupLocationCountry.md)
 - [IpAddressLookupResponse](docs/IpAddressLookupResponse.md)
 - [IpAddressResponse](docs/IpAddressResponse.md)
 - [IpAddressesLookupPayload](docs/IpAddressesLookupPayload.md)
 - [IpAddressesLookupResponse](docs/IpAddressesLookupResponse.md)
 - [LobbyCreatePayload](docs/LobbyCreatePayload.md)
 - [LobbyDeployPayload](docs/LobbyDeployPayload.md)
 - [LobbyReadResponse](docs/LobbyReadResponse.md)
 - [LobbyTerminatePayload](docs/LobbyTerminatePayload.md)
 - [Location](docs/Location.md)
 - [LocationBeaconList](docs/LocationBeaconList.md)
 - [LocationModel](docs/LocationModel.md)
 - [Locations](docs/Locations.md)
 - [MappedPortResponse](docs/MappedPortResponse.md)
 - [MatchmakerComponentCreate](docs/MatchmakerComponentCreate.md)
 - [MatchmakerComponentEnvListResponse](docs/MatchmakerComponentEnvListResponse.md)
 - [MatchmakerComponentEnvsCreate](docs/MatchmakerComponentEnvsCreate.md)
 - [MatchmakerComponentEnvsResponse](docs/MatchmakerComponentEnvsResponse.md)
 - [MatchmakerComponentEnvsUpdate](docs/MatchmakerComponentEnvsUpdate.md)
 - [MatchmakerComponentListResponse](docs/MatchmakerComponentListResponse.md)
 - [MatchmakerComponentResponse](docs/MatchmakerComponentResponse.md)
 - [MatchmakerComponentUpdate](docs/MatchmakerComponentUpdate.md)
 - [MatchmakerCreate](docs/MatchmakerCreate.md)
 - [MatchmakerListResponse](docs/MatchmakerListResponse.md)
 - [MatchmakerManagedReleaseCreate](docs/MatchmakerManagedReleaseCreate.md)
 - [MatchmakerManagedReleaseResponse](docs/MatchmakerManagedReleaseResponse.md)
 - [MatchmakerManagedReleaseUpdate](docs/MatchmakerManagedReleaseUpdate.md)
 - [MatchmakerReleaseConfigCreate](docs/MatchmakerReleaseConfigCreate.md)
 - [MatchmakerReleaseConfigResponse](docs/MatchmakerReleaseConfigResponse.md)
 - [MatchmakerReleaseConfigUpdate](docs/MatchmakerReleaseConfigUpdate.md)
 - [MatchmakerReleaseCreate](docs/MatchmakerReleaseCreate.md)
 - [MatchmakerReleaseCreateBase](docs/MatchmakerReleaseCreateBase.md)
 - [MatchmakerReleaseResponse](docs/MatchmakerReleaseResponse.md)
 - [MatchmakerReleaseResponseBase](docs/MatchmakerReleaseResponseBase.md)
 - [MatchmakerReleaseUpdate](docs/MatchmakerReleaseUpdate.md)
 - [MatchmakerReleaseUpdateBase](docs/MatchmakerReleaseUpdateBase.md)
 - [MatchmakerResponse](docs/MatchmakerResponse.md)
 - [MatchmakerUpdate](docs/MatchmakerUpdate.md)
 - [MetricsModel](docs/MetricsModel.md)
 - [MetricsResponse](docs/MetricsResponse.md)
 - [Monitor](docs/Monitor.md)
 - [NetworkMetricsModel](docs/NetworkMetricsModel.md)
 - [Pagination](docs/Pagination.md)
 - [Paginator](docs/Paginator.md)
 - [PatchSessionModel](docs/PatchSessionModel.md)
 - [PortMapping](docs/PortMapping.md)
 - [PullProfileAppVersionLinkResponse](docs/PullProfileAppVersionLinkResponse.md)
 - [PullProfileGetResponse](docs/PullProfileGetResponse.md)
 - [PullProfilePatchPayload](docs/PullProfilePatchPayload.md)
 - [PullProfilePostPayload](docs/PullProfilePostPayload.md)
 - [PullProfilePostResponse](docs/PullProfilePostResponse.md)
 - [PullProfilesListResponse](docs/PullProfilesListResponse.md)
 - [PulloProfilePatchResponse](docs/PulloProfilePatchResponse.md)
 - [RelayFilterModel](docs/RelayFilterModel.md)
 - [RelayResponse](docs/RelayResponse.md)
 - [RelaySessionBaseResponse](docs/RelaySessionBaseResponse.md)
 - [RelaySessionCreatePayload](docs/RelaySessionCreatePayload.md)
 - [RelaySessionListResponse](docs/RelaySessionListResponse.md)
 - [RelaySessionUser](docs/RelaySessionUser.md)
 - [RelaySessionUserBaseResponse](docs/RelaySessionUserBaseResponse.md)
 - [RelaySessionUserResponse](docs/RelaySessionUserResponse.md)
 - [RelayUserAuthorizePayload](docs/RelayUserAuthorizePayload.md)
 - [RelayUserRevokePayload](docs/RelayUserRevokePayload.md)
 - [Request](docs/Request.md)
 - [SelectorEnvModel](docs/SelectorEnvModel.md)
 - [SelectorModel](docs/SelectorModel.md)
 - [ServerRelayPort](docs/ServerRelayPort.md)
 - [SessionBulkStopFiltersPayload](docs/SessionBulkStopFiltersPayload.md)
 - [SessionBulkStopPayload](docs/SessionBulkStopPayload.md)
 - [SessionBulkStopResponse](docs/SessionBulkStopResponse.md)
 - [SessionContext](docs/SessionContext.md)
 - [SessionDelete](docs/SessionDelete.md)
 - [SessionFilterModel](docs/SessionFilterModel.md)
 - [SessionGet](docs/SessionGet.md)
 - [SessionModel](docs/SessionModel.md)
 - [SessionRequest](docs/SessionRequest.md)
 - [SessionStopResponse](docs/SessionStopResponse.md)
 - [SessionUser](docs/SessionUser.md)
 - [SessionUserContext](docs/SessionUserContext.md)
 - [Sessions](docs/Sessions.md)
 - [Status](docs/Status.md)
 - [Tag](docs/Tag.md)
 - [TotalMetricsModel](docs/TotalMetricsModel.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

info@edgegap.com

