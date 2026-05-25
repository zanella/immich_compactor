# \ViewsApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_assets_by_original_path**](ViewsApi.md#get_assets_by_original_path) | **GET** /view/folder | Retrieve assets by original path
[**get_unique_original_paths**](ViewsApi.md#get_unique_original_paths) | **GET** /view/folder/unique-paths | Retrieve unique paths



## get_assets_by_original_path

> Vec<models::AssetResponseDto> get_assets_by_original_path(path)
Retrieve assets by original path

Retrieve assets that are children of a specific folder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** |  | [required] |

### Return type

[**Vec<models::AssetResponseDto>**](AssetResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_unique_original_paths

> Vec<String> get_unique_original_paths()
Retrieve unique paths

Retrieve a list of unique folder paths from asset original paths.

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

