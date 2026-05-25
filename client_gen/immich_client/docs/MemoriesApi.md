# \MemoriesApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_memory_assets**](MemoriesApi.md#add_memory_assets) | **PUT** /memories/{id}/assets | Add assets to a memory
[**create_memory**](MemoriesApi.md#create_memory) | **POST** /memories | Create a memory
[**delete_memory**](MemoriesApi.md#delete_memory) | **DELETE** /memories/{id} | Delete a memory
[**get_memory**](MemoriesApi.md#get_memory) | **GET** /memories/{id} | Retrieve a memory
[**memories_statistics**](MemoriesApi.md#memories_statistics) | **GET** /memories/statistics | Retrieve memories statistics
[**remove_memory_assets**](MemoriesApi.md#remove_memory_assets) | **DELETE** /memories/{id}/assets | Remove assets from a memory
[**search_memories**](MemoriesApi.md#search_memories) | **GET** /memories | Retrieve memories
[**update_memory**](MemoriesApi.md#update_memory) | **PUT** /memories/{id} | Update a memory



## add_memory_assets

> Vec<models::BulkIdResponseDto> add_memory_assets(id, bulk_ids_dto)
Add assets to a memory

Add a list of asset IDs to a specific memory.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**bulk_ids_dto** | [**BulkIdsDto**](BulkIdsDto.md) |  | [required] |

### Return type

[**Vec<models::BulkIdResponseDto>**](BulkIdResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_memory

> models::MemoryResponseDto create_memory(memory_create_dto)
Create a memory

Create a new memory by providing a name, description, and a list of asset IDs to include in the memory.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**memory_create_dto** | [**MemoryCreateDto**](MemoryCreateDto.md) |  | [required] |

### Return type

[**models::MemoryResponseDto**](MemoryResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_memory

> delete_memory(id)
Delete a memory

Delete a specific memory by its ID.

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


## get_memory

> models::MemoryResponseDto get_memory(id)
Retrieve a memory

Retrieve a specific memory by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::MemoryResponseDto**](MemoryResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## memories_statistics

> models::MemoryStatisticsResponseDto memories_statistics(r#for, is_saved, is_trashed, order, size, r#type)
Retrieve memories statistics

Retrieve statistics about memories, such as total count and other relevant metrics.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#for** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Filter by date |  |
**is_saved** | Option<**bool**> | Filter by saved status |  |
**is_trashed** | Option<**bool**> | Include trashed memories |  |
**order** | Option<[**MemorySearchOrder**](MemorySearchOrder.md)> |  |  |
**size** | Option<**i32**> | Number of memories to return |  |
**r#type** | Option<[**MemoryType**](MemoryType.md)> |  |  |

### Return type

[**models::MemoryStatisticsResponseDto**](MemoryStatisticsResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_memory_assets

> Vec<models::BulkIdResponseDto> remove_memory_assets(id, bulk_ids_dto)
Remove assets from a memory

Remove a list of asset IDs from a specific memory.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**bulk_ids_dto** | [**BulkIdsDto**](BulkIdsDto.md) |  | [required] |

### Return type

[**Vec<models::BulkIdResponseDto>**](BulkIdResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_memories

> Vec<models::MemoryResponseDto> search_memories(r#for, is_saved, is_trashed, order, size, r#type)
Retrieve memories

Retrieve a list of memories. Memories are sorted descending by creation date by default, although they can also be sorted in ascending order, or randomly.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#for** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Filter by date |  |
**is_saved** | Option<**bool**> | Filter by saved status |  |
**is_trashed** | Option<**bool**> | Include trashed memories |  |
**order** | Option<[**MemorySearchOrder**](MemorySearchOrder.md)> |  |  |
**size** | Option<**i32**> | Number of memories to return |  |
**r#type** | Option<[**MemoryType**](MemoryType.md)> |  |  |

### Return type

[**Vec<models::MemoryResponseDto>**](MemoryResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_memory

> models::MemoryResponseDto update_memory(id, memory_update_dto)
Update a memory

Update an existing memory by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**memory_update_dto** | [**MemoryUpdateDto**](MemoryUpdateDto.md) |  | [required] |

### Return type

[**models::MemoryResponseDto**](MemoryResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

