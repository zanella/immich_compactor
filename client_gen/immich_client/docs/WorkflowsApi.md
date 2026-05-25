# \WorkflowsApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_workflow**](WorkflowsApi.md#create_workflow) | **POST** /workflows | Create a workflow
[**delete_workflow**](WorkflowsApi.md#delete_workflow) | **DELETE** /workflows/{id} | Delete a workflow
[**get_workflow**](WorkflowsApi.md#get_workflow) | **GET** /workflows/{id} | Retrieve a workflow
[**get_workflow_for_share**](WorkflowsApi.md#get_workflow_for_share) | **GET** /workflows/{id}/share | Retrieve a workflow
[**get_workflow_triggers**](WorkflowsApi.md#get_workflow_triggers) | **GET** /workflows/triggers | List all workflow triggers
[**search_workflows**](WorkflowsApi.md#search_workflows) | **GET** /workflows | List all workflows
[**update_workflow**](WorkflowsApi.md#update_workflow) | **PUT** /workflows/{id} | Update a workflow



## create_workflow

> models::WorkflowResponseDto create_workflow(workflow_create_dto)
Create a workflow

Create a new workflow, the workflow can also be created with empty filters and actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workflow_create_dto** | [**WorkflowCreateDto**](WorkflowCreateDto.md) |  | [required] |

### Return type

[**models::WorkflowResponseDto**](WorkflowResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_workflow

> delete_workflow(id)
Delete a workflow

Delete a workflow by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workflow

> models::WorkflowResponseDto get_workflow(id)
Retrieve a workflow

Retrieve information about a specific workflow by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::WorkflowResponseDto**](WorkflowResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workflow_for_share

> models::WorkflowShareResponseDto get_workflow_for_share(id)
Retrieve a workflow

Retrieve a workflow details without ids, default values, etc.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::WorkflowShareResponseDto**](WorkflowShareResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workflow_triggers

> Vec<models::WorkflowTriggerResponseDto> get_workflow_triggers()
List all workflow triggers

Retrieve a list of all available workflow triggers.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::WorkflowTriggerResponseDto>**](WorkflowTriggerResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_workflows

> Vec<models::WorkflowResponseDto> search_workflows(description, enabled, id, name, trigger)
List all workflows

Retrieve a list of workflows available to the authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**description** | Option<**String**> | Workflow description |  |
**enabled** | Option<**bool**> | Workflow enabled |  |
**id** | Option<**uuid::Uuid**> | Workflow ID |  |
**name** | Option<**String**> | Workflow name |  |
**trigger** | Option<[**WorkflowTrigger**](WorkflowTrigger.md)> | Workflow trigger type |  |

### Return type

[**Vec<models::WorkflowResponseDto>**](WorkflowResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_workflow

> models::WorkflowResponseDto update_workflow(id, workflow_update_dto)
Update a workflow

Update the information of a specific workflow by its ID. This endpoint can be used to update the workflow name, description, trigger type, filters and actions order, etc.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**workflow_update_dto** | [**WorkflowUpdateDto**](WorkflowUpdateDto.md) |  | [required] |

### Return type

[**models::WorkflowResponseDto**](WorkflowResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

