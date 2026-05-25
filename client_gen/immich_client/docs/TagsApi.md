# \TagsApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bulk_tag_assets**](TagsApi.md#bulk_tag_assets) | **PUT** /tags/assets | Tag assets
[**create_tag**](TagsApi.md#create_tag) | **POST** /tags | Create a tag
[**delete_tag**](TagsApi.md#delete_tag) | **DELETE** /tags/{id} | Delete a tag
[**get_all_tags**](TagsApi.md#get_all_tags) | **GET** /tags | Retrieve tags
[**get_tag_by_id**](TagsApi.md#get_tag_by_id) | **GET** /tags/{id} | Retrieve a tag
[**tag_assets**](TagsApi.md#tag_assets) | **PUT** /tags/{id}/assets | Tag assets
[**untag_assets**](TagsApi.md#untag_assets) | **DELETE** /tags/{id}/assets | Untag assets
[**update_tag**](TagsApi.md#update_tag) | **PUT** /tags/{id} | Update a tag
[**upsert_tags**](TagsApi.md#upsert_tags) | **PUT** /tags | Upsert tags



## bulk_tag_assets

> models::TagBulkAssetsResponseDto bulk_tag_assets(tag_bulk_assets_dto)
Tag assets

Add multiple tags to multiple assets in a single request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_bulk_assets_dto** | [**TagBulkAssetsDto**](TagBulkAssetsDto.md) |  | [required] |

### Return type

[**models::TagBulkAssetsResponseDto**](TagBulkAssetsResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_tag

> models::TagResponseDto create_tag(tag_create_dto)
Create a tag

Create a new tag by providing a name and optional color.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_create_dto** | [**TagCreateDto**](TagCreateDto.md) |  | [required] |

### Return type

[**models::TagResponseDto**](TagResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_tag

> delete_tag(id)
Delete a tag

Delete a specific tag by its ID.

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


## get_all_tags

> Vec<models::TagResponseDto> get_all_tags()
Retrieve tags

Retrieve a list of all tags.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::TagResponseDto>**](TagResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tag_by_id

> models::TagResponseDto get_tag_by_id(id)
Retrieve a tag

Retrieve a specific tag by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::TagResponseDto**](TagResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tag_assets

> Vec<models::BulkIdResponseDto> tag_assets(id, bulk_ids_dto)
Tag assets

Add a tag to all the specified assets.

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


## untag_assets

> Vec<models::BulkIdResponseDto> untag_assets(id, bulk_ids_dto)
Untag assets

Remove a tag from all the specified assets.

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


## update_tag

> models::TagResponseDto update_tag(id, tag_update_dto)
Update a tag

Update an existing tag identified by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**tag_update_dto** | [**TagUpdateDto**](TagUpdateDto.md) |  | [required] |

### Return type

[**models::TagResponseDto**](TagResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upsert_tags

> Vec<models::TagResponseDto> upsert_tags(tag_upsert_dto)
Upsert tags

Create or update multiple tags in a single request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_upsert_dto** | [**TagUpsertDto**](TagUpsertDto.md) |  | [required] |

### Return type

[**Vec<models::TagResponseDto>**](TagResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

