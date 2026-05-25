# \TrashApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**empty_trash**](TrashApi.md#empty_trash) | **POST** /trash/empty | Empty trash
[**restore_assets**](TrashApi.md#restore_assets) | **POST** /trash/restore/assets | Restore assets
[**restore_trash**](TrashApi.md#restore_trash) | **POST** /trash/restore | Restore trash



## empty_trash

> models::TrashResponseDto empty_trash()
Empty trash

Permanently delete all items in the trash.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::TrashResponseDto**](TrashResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## restore_assets

> models::TrashResponseDto restore_assets(bulk_ids_dto)
Restore assets

Restore specific assets from the trash.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_ids_dto** | [**BulkIdsDto**](BulkIdsDto.md) |  | [required] |

### Return type

[**models::TrashResponseDto**](TrashResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## restore_trash

> models::TrashResponseDto restore_trash()
Restore trash

Restore all items in the trash.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::TrashResponseDto**](TrashResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

