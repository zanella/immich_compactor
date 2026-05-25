# \PluginsApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_plugin**](PluginsApi.md#get_plugin) | **GET** /plugins/{id} | Retrieve a plugin
[**search_plugin_methods**](PluginsApi.md#search_plugin_methods) | **GET** /plugins/methods | Retrieve plugin methods
[**search_plugins**](PluginsApi.md#search_plugins) | **GET** /plugins | List all plugins



## get_plugin

> models::PluginResponseDto get_plugin(id)
Retrieve a plugin

Retrieve information about a specific plugin by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::PluginResponseDto**](PluginResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_plugin_methods

> Vec<models::PluginMethodResponseDto> search_plugin_methods(description, enabled, id, name, plugin_name, plugin_version, title, trigger, r#type)
Retrieve plugin methods

Retrieve a list of plugin methods

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**description** | Option<**String**> |  |  |
**enabled** | Option<**bool**> | Whether the plugin method is enabled |  |
**id** | Option<**uuid::Uuid**> | Plugin method ID |  |
**name** | Option<**String**> |  |  |
**plugin_name** | Option<**String**> | Plugin name |  |
**plugin_version** | Option<**String**> | Plugin version |  |
**title** | Option<**String**> |  |  |
**trigger** | Option<[**WorkflowTrigger**](WorkflowTrigger.md)> | Workflow trigger |  |
**r#type** | Option<[**WorkflowType**](WorkflowType.md)> | Workflow types |  |

### Return type

[**Vec<models::PluginMethodResponseDto>**](PluginMethodResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_plugins

> Vec<models::PluginResponseDto> search_plugins(description, enabled, id, name, title, version)
List all plugins

Retrieve a list of plugins available to the authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**description** | Option<**String**> |  |  |
**enabled** | Option<**bool**> | Whether the plugin is enabled |  |
**id** | Option<**uuid::Uuid**> | Plugin ID |  |
**name** | Option<**String**> |  |  |
**title** | Option<**String**> |  |  |
**version** | Option<**String**> |  |  |

### Return type

[**Vec<models::PluginResponseDto>**](PluginResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

