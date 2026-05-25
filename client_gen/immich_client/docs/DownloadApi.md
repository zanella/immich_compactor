# \DownloadApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**download_archive**](DownloadApi.md#download_archive) | **POST** /download/archive | Download asset archive
[**get_download_info**](DownloadApi.md#get_download_info) | **POST** /download/info | Retrieve download information



## download_archive

> std::path::PathBuf download_archive(download_archive_dto, key, slug)
Download asset archive

Download a ZIP archive containing the specified assets. The assets must have been previously requested via the \"getDownloadInfo\" endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**download_archive_dto** | [**DownloadArchiveDto**](DownloadArchiveDto.md) |  | [required] |
**key** | Option<**String**> |  |  |
**slug** | Option<**String**> |  |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_download_info

> models::DownloadResponseDto get_download_info(download_info_dto, key, slug)
Retrieve download information

Retrieve information about how to request a download for the specified assets or album. The response includes groups of assets that can be downloaded together.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**download_info_dto** | [**DownloadInfoDto**](DownloadInfoDto.md) |  | [required] |
**key** | Option<**String**> |  |  |
**slug** | Option<**String**> |  |  |

### Return type

[**models::DownloadResponseDto**](DownloadResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

