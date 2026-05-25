# \DuplicatesApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_duplicate**](DuplicatesApi.md#delete_duplicate) | **DELETE** /duplicates/{id} | Dismiss a duplicate group
[**delete_duplicates**](DuplicatesApi.md#delete_duplicates) | **DELETE** /duplicates | Delete duplicates
[**get_asset_duplicates**](DuplicatesApi.md#get_asset_duplicates) | **GET** /duplicates | Retrieve duplicates
[**resolve_duplicates**](DuplicatesApi.md#resolve_duplicates) | **POST** /duplicates/resolve | Resolve duplicate groups



## delete_duplicate

> delete_duplicate(id)
Dismiss a duplicate group

Dismiss a duplicate group by its ID, unlinking all assets in the group without deleting them.

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


## delete_duplicates

> delete_duplicates(bulk_ids_dto)
Delete duplicates

Delete multiple duplicate assets specified by their IDs.

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


## get_asset_duplicates

> Vec<models::DuplicateResponseDto> get_asset_duplicates()
Retrieve duplicates

Retrieve a list of duplicate assets available to the authenticated user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::DuplicateResponseDto>**](DuplicateResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resolve_duplicates

> Vec<models::BulkIdResponseDto> resolve_duplicates(duplicate_resolve_dto)
Resolve duplicate groups

Resolve duplicate groups by synchronizing metadata across assets and deleting/trashing duplicates.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**duplicate_resolve_dto** | [**DuplicateResolveDto**](DuplicateResolveDto.md) |  | [required] |

### Return type

[**Vec<models::BulkIdResponseDto>**](BulkIdResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

