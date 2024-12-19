/*
 * Edgegap V1
 *
 * <h1 style=\"margin-top: 2rem\">Introduction</h1> <p>The Edgegap API lets you manage all your resources from standard HTTP requests. We promote using them to automate all your processes that are using Edgegap.</p> <p>If you have any questions, don't hesitate to contact us via email, or you can also jump on our <a href=\"https://discord.com/invite/GYaHcKR9a5\" target=\"_blank\">Discord</a>. We will be happy to help. Feel free to make features request; we also love those.</p> <div class=\"theme-doc-markdown markdown\"><h2 class=\"anchor anchorWithStickyNavbar_node_modules-@docusaurus-theme-classic-lib-theme-Heading-styles-module\" id=\"pagination---response\">Pagination - Response<a class=\"hash-link\" href=\"#pagination---response\" title=\"Direct link to heading\">​</a></h2><p>The GET response body can be returned with pagination to avoid requesting all the data at once.</p><p>Pagination object will always be under the  <em>paginator</em> key.</p><div class=\"language-json codeBlockContainer_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-Container-styles-module theme-code-block\" style=\"--prism-color:#bfc7d5; --prism-background-color:#292d3e;\"><div class=\"codeBlockContent_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-Content-styles-module\"><pre tabindex=\"0\" class=\"prism-code language-json codeBlock_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-Content-styles-module thin-scrollbar\"><code class=\"codeBlockLines_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-Content-styles-module\"><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">{</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">  </span><span class=\"token property\">\"number\"</span><span class=\"token operator\" style=\"color: rgb(137, 221, 255);\">:</span><span class=\"token plain\"> </span><span class=\"token string\" style=\"color: rgb(195, 232, 141);\">\"The Current page, default=1\"</span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">,</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">  </span><span class=\"token property\">\"next_page_number\"</span><span class=\"token operator\" style=\"color: rgb(137, 221, 255);\">:</span><span class=\"token plain\"> </span><span class=\"token string\" style=\"color: rgb(195, 232, 141);\">\"The Next page number or null\"</span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">,</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">  </span><span class=\"token property\">\"previous_page_number\"</span><span class=\"token operator\" style=\"color: rgb(137, 221, 255);\">:</span><span class=\"token plain\"> </span><span class=\"token string\" style=\"color: rgb(195, 232, 141);\">\"The Previous page number or null\"</span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">,</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">  </span><span class=\"token property\">\"paginator\"</span><span class=\"token operator\" style=\"color: rgb(137, 221, 255);\">:</span><span class=\"token plain\"> </span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">{</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">    </span><span class=\"token property\">\"num_pages\"</span><span class=\"token operator\" style=\"color: rgb(137, 221, 255);\">:</span><span class=\"token plain\"> </span><span class=\"token string\" style=\"color: rgb(195, 232, 141);\">\"The total numbers of pages\"</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">  </span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">}</span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">,</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">  </span><span class=\"token property\">\"has_next\"</span><span class=\"token operator\" style=\"color: rgb(137, 221, 255);\">:</span><span class=\"token plain\"> </span><span class=\"token string\" style=\"color: rgb(195, 232, 141);\">\"Boolean if there is a next page\"</span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">,</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">  </span><span class=\"token property\">\"has_previous\"</span><span class=\"token operator\" style=\"color: rgb(137, 221, 255);\">:</span><span class=\"token plain\"> </span><span class=\"token string\" style=\"color: rgb(195, 232, 141);\">\"Boolean if there is a previous page\"</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\"></span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">}</span><br></span></code></pre><div class=\"buttonGroup_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-Content-styles-module\"><button type=\"button\" aria-label=\"Copy code to clipboard\" title=\"Copy\" class=\"clean-btn\"><span class=\"copyButtonIcons_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-CopyButton-styles-module\" aria-hidden=\"true\"><svg class=\"copyButtonIcon_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-CopyButton-styles-module\" viewBox=\"0 0 24 24\"><path d=\"M19,21H8V7H19M19,5H8A2,2 0 0,0 6,7V21A2,2 0 0,0 8,23H19A2,2 0 0,0 21,21V7A2,2 0 0,0 19,5M16,1H4A2,2 0 0,0 2,3V17H4V3H16V1Z\"></path></svg><svg class=\"copyButtonSuccessIcon_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-CopyButton-styles-module\" viewBox=\"0 0 24 24\"><path d=\"M21,7L9,19L3.5,13.5L4.91,12.09L9,16.17L19.59,5.59L21,7Z\"></path></svg></span></button></div></div></div><p>Full Body Example:</p><div class=\"language-json codeBlockContainer_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-Container-styles-module theme-code-block\" style=\"--prism-color:#bfc7d5; --prism-background-color:#292d3e;\"><div class=\"codeBlockContent_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-Content-styles-module\"><pre tabindex=\"0\" class=\"prism-code language-json codeBlock_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-Content-styles-module thin-scrollbar\"><code class=\"codeBlockLines_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-Content-styles-module\"><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">{</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">  </span><span class=\"token property\">\"count\"</span><span class=\"token operator\" style=\"color: rgb(137, 221, 255);\">:</span><span class=\"token plain\"> </span><span class=\"token number\" style=\"color: rgb(247, 140, 108);\">100</span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">,</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">  </span><span class=\"token property\">\"data\"</span><span class=\"token operator\" style=\"color: rgb(137, 221, 255);\">:</span><span class=\"token plain\"> </span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">[</span><span class=\"token string\" style=\"color: rgb(195, 232, 141);\">\"value-0\"</span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">,</span><span class=\"token plain\"> </span><span class=\"token string\" style=\"color: rgb(195, 232, 141);\">\"[...]\"</span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">,</span><span class=\"token plain\"> </span><span class=\"token string\" style=\"color: rgb(195, 232, 141);\">\"value-9\"</span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">]</span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">,</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">  </span><span class=\"token property\">\"success\"</span><span class=\"token operator\" style=\"color: rgb(137, 221, 255);\">:</span><span class=\"token plain\"> </span><span class=\"token boolean\" style=\"color: rgb(255, 88, 116);\">true</span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">,</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">  </span><span class=\"token property\">\"pagination\"</span><span class=\"token operator\" style=\"color: rgb(137, 221, 255);\">:</span><span class=\"token plain\"> </span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">{</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">    </span><span class=\"token property\">\"number\"</span><span class=\"token operator\" style=\"color: rgb(137, 221, 255);\">:</span><span class=\"token plain\"> </span><span class=\"token number\" style=\"color: rgb(247, 140, 108);\">1</span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">,</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">    </span><span class=\"token property\">\"next_page_number\"</span><span class=\"token operator\" style=\"color: rgb(137, 221, 255);\">:</span><span class=\"token plain\"> </span><span class=\"token number\" style=\"color: rgb(247, 140, 108);\">2</span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">,</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">    </span><span class=\"token property\">\"previous_page_number\"</span><span class=\"token operator\" style=\"color: rgb(137, 221, 255);\">:</span><span class=\"token plain\"> </span><span class=\"token null keyword\" style=\"font-style: italic;\">null</span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">,</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">    </span><span class=\"token property\">\"paginator\"</span><span class=\"token operator\" style=\"color: rgb(137, 221, 255);\">:</span><span class=\"token plain\"> </span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">{</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">      </span><span class=\"token property\">\"num_pages\"</span><span class=\"token operator\" style=\"color: rgb(137, 221, 255);\">:</span><span class=\"token plain\"> </span><span class=\"token number\" style=\"color: rgb(247, 140, 108);\">10</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">    </span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">}</span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">,</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">    </span><span class=\"token property\">\"has_next\"</span><span class=\"token operator\" style=\"color: rgb(137, 221, 255);\">:</span><span class=\"token plain\"> </span><span class=\"token boolean\" style=\"color: rgb(255, 88, 116);\">true</span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">,</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">    </span><span class=\"token property\">\"has_previous\"</span><span class=\"token operator\" style=\"color: rgb(137, 221, 255);\">:</span><span class=\"token plain\"> </span><span class=\"token boolean\" style=\"color: rgb(255, 88, 116);\">false</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">  </span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">}</span><span class=\"token plain\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\"></span><span class=\"token punctuation\" style=\"color: rgb(199, 146, 234);\">}</span><br></span></code></pre><div class=\"buttonGroup_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-Content-styles-module\"><button type=\"button\" aria-label=\"Copy code to clipboard\" title=\"Copy\" class=\"clean-btn\"><span class=\"copyButtonIcons_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-CopyButton-styles-module\" aria-hidden=\"true\"><svg class=\"copyButtonIcon_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-CopyButton-styles-module\" viewBox=\"0 0 24 24\"><path d=\"M19,21H8V7H19M19,5H8A2,2 0 0,0 6,7V21A2,2 0 0,0 8,23H19A2,2 0 0,0 21,21V7A2,2 0 0,0 19,5M16,1H4A2,2 0 0,0 2,3V17H4V3H16V1Z\"></path></svg><svg class=\"copyButtonSuccessIcon_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-CopyButton-styles-module\" viewBox=\"0 0 24 24\"><path d=\"M21,7L9,19L3.5,13.5L4.91,12.09L9,16.17L19.59,5.59L21,7Z\"></path></svg></span></button></div></div></div><h2 class=\"anchor anchorWithStickyNavbar_node_modules-@docusaurus-theme-classic-lib-theme-Heading-styles-module\" id=\"pagination---parameters\">Pagination - Parameters<a class=\"hash-link\" href=\"#pagination---parameters\" title=\"Direct link to heading\">​</a></h2><p>You can add those values to manipulate the pagination object in the URL Parameters.</p><ul class=\"\"><li>page</li><li>limit</li></ul><p>Example:</p><div class=\"language-text codeBlockContainer_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-Container-styles-module theme-code-block\" style=\"--prism-color:#bfc7d5; --prism-background-color:#292d3e;\"><div class=\"codeBlockContent_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-Content-styles-module\"><pre tabindex=\"0\" class=\"prism-code language-text codeBlock_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-Content-styles-module thin-scrollbar\"><code class=\"codeBlockLines_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-Content-styles-module\"><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\"># To get the second page</span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">GET - https://api.edgegap.com/v1/apps?page=2</span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\" style=\"display: inline-block;\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\"># To change the count of element in one page (20/page)</span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">GET - https://api.edgegap.com/v1/apps?limit=20</span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\" style=\"display: inline-block;\"></span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\"># You can mix those (20/page, second page)</span><br></span><span class=\"token-line\" style=\"color: rgb(191, 199, 213);\"><span class=\"token plain\">GET - https://api.edgegap.com/v1/apps?page=2&amp;limit=20</span><br></span></code></pre><div class=\"buttonGroup_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-Content-styles-module\"><button type=\"button\" aria-label=\"Copy code to clipboard\" title=\"Copy\" class=\"clean-btn\"><span class=\"copyButtonIcons_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-CopyButton-styles-module\" aria-hidden=\"true\"><svg class=\"copyButtonIcon_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-CopyButton-styles-module\" viewBox=\"0 0 24 24\"><path d=\"M19,21H8V7H19M19,5H8A2,2 0 0,0 6,7V21A2,2 0 0,0 8,23H19A2,2 0 0,0 21,21V7A2,2 0 0,0 19,5M16,1H4A2,2 0 0,0 2,3V17H4V3H16V1Z\"></path></svg><svg class=\"copyButtonSuccessIcon_node_modules-@docusaurus-theme-classic-lib-theme-CodeBlock-CopyButton-styles-module\" viewBox=\"0 0 24 24\"><path d=\"M21,7L9,19L3.5,13.5L4.91,12.09L9,16.17L19.59,5.59L21,7Z\"></path></svg></span></button></div></div></div></div>
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: info@edgegap.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeployModel {
    /// The name of the App you want to deploy
    #[serde(rename = "app_name")]
    pub app_name: String,
    /// The name of the App Version you want to deploy, if not present, the last version created is picked
    #[serde(rename = "version_name", skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
    /// If the Application is public or private. If not specified, we will look for a private Application
    #[serde(rename = "is_public_app", skip_serializing_if = "Option::is_none")]
    pub is_public_app: Option<bool>,
    /// The List of IP of your user
    #[serde(rename = "ip_list", skip_serializing_if = "Option::is_none")]
    pub ip_list: Option<Vec<String>>,
    /// The list of IP of your user with their location (latitude, longitude)
    #[serde(rename = "geo_ip_list", skip_serializing_if = "Option::is_none")]
    pub geo_ip_list: Option<Vec<models::GeoIpListModel>>,
    #[serde(rename = "telemetry_profile_uuid_list", skip_serializing_if = "Option::is_none")]
    pub telemetry_profile_uuid_list: Option<Vec<String>>,
    /// A list of deployment variables
    #[serde(rename = "env_vars", skip_serializing_if = "Option::is_none")]
    pub env_vars: Option<Vec<models::DeployEnvModel>>,
    /// If you want to skip the Telemetry and use a geolocations decision only
    #[serde(rename = "skip_telemetry", skip_serializing_if = "Option::is_none")]
    pub skip_telemetry: Option<bool>,
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<Box<models::LocationModel>>,
    /// DEPRECATED - See `filters` instead.
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// DEPRECATED - See `filters` instead.
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// DEPRECATED - See `filters` instead.
    #[serde(rename = "continent", skip_serializing_if = "Option::is_none")]
    pub continent: Option<String>,
    /// DEPRECATED - See `filters` instead.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// DEPRECATED - See `filters` instead.
    #[serde(rename = "administrative_division", skip_serializing_if = "Option::is_none")]
    pub administrative_division: Option<String>,
    /// A web URL. This url will be called with method POST. The deployment status will be send in JSON format
    #[serde(rename = "webhook_url", skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
    /// The list of tags for your deployment
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "container_log_storage", skip_serializing_if = "Option::is_none")]
    pub container_log_storage: Option<Box<models::ContainerLogStorageModel>>,
    /// Filters to use while choosing the deployment location.
    #[serde(rename = "filters", skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<models::ApiModelDeploymentfilter>>,
    /// Algorithm used to select the edge location
    #[serde(rename = "ap_sort_strategy", skip_serializing_if = "Option::is_none")]
    pub ap_sort_strategy: Option<ApSortStrategy>,
}

impl DeployModel {
    pub fn new(app_name: String) -> DeployModel {
        DeployModel {
            app_name,
            version_name: None,
            is_public_app: None,
            ip_list: None,
            geo_ip_list: None,
            telemetry_profile_uuid_list: None,
            env_vars: None,
            skip_telemetry: None,
            location: None,
            city: None,
            country: None,
            continent: None,
            region: None,
            administrative_division: None,
            webhook_url: None,
            tags: None,
            container_log_storage: None,
            filters: None,
            ap_sort_strategy: None,
        }
    }
}
/// Algorithm used to select the edge location
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ApSortStrategy {
    #[serde(rename = "basic")]
    Basic,
    #[serde(rename = "weighted")]
    Weighted,
}

impl Default for ApSortStrategy {
    fn default() -> ApSortStrategy {
        Self::Basic
    }
}

