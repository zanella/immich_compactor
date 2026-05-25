# \StacksApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_stack**](StacksApi.md#create_stack) | **POST** /stacks | Create a stack
[**delete_stack**](StacksApi.md#delete_stack) | **DELETE** /stacks/{id} | Delete a stack
[**delete_stacks**](StacksApi.md#delete_stacks) | **DELETE** /stacks | Delete stacks
[**get_stack**](StacksApi.md#get_stack) | **GET** /stacks/{id} | Retrieve a stack
[**remove_asset_from_stack**](StacksApi.md#remove_asset_from_stack) | **DELETE** /stacks/{id}/assets/{assetId} | Remove an asset from a stack
[**search_stacks**](StacksApi.md#search_stacks) | **GET** /stacks | Retrieve stacks
[**update_stack**](StacksApi.md#update_stack) | **PUT** /stacks/{id} | Update a stack



## create_stack

> models::StackResponseDto create_stack(stack_create_dto)
Create a stack

Create a new stack by providing a name and a list of asset IDs to include in the stack. If any of the provided asset IDs are primary assets of an existing stack, the existing stack will be merged into the newly created stack.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stack_create_dto** | [**StackCreateDto**](StackCreateDto.md) |  | [required] |

### Return type

[**models::StackResponseDto**](StackResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_stack

> delete_stack(id)
Delete a stack

Delete a specific stack by its ID.

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


## delete_stacks

> delete_stacks(bulk_ids_dto)
Delete stacks

Delete multiple stacks by providing a list of stack IDs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_ids_dto** | [**BulkIdsDto**](BulkIdsDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stack

> models::StackResponseDto get_stack(id)
Retrieve a stack

Retrieve a specific stack by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::StackResponseDto**](StackResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_asset_from_stack

> remove_asset_from_stack(asset_id, id)
Remove an asset from a stack

Remove a specific asset from a stack by providing the stack ID and asset ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_id** | **uuid::Uuid** |  | [required] |
**id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_stacks

> Vec<models::StackResponseDto> search_stacks(primary_asset_id)
Retrieve stacks

Retrieve a list of stacks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**primary_asset_id** | Option<**uuid::Uuid**> | Filter by primary asset ID |  |

### Return type

[**Vec<models::StackResponseDto>**](StackResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_stack

> models::StackResponseDto update_stack(id, stack_update_dto)
Update a stack

Update an existing stack by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**stack_update_dto** | [**StackUpdateDto**](StackUpdateDto.md) |  | [required] |

### Return type

[**models::StackResponseDto**](StackResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

