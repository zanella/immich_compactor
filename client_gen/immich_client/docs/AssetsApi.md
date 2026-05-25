# \AssetsApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check_bulk_upload**](AssetsApi.md#check_bulk_upload) | **POST** /assets/bulk-upload-check | Check bulk upload
[**copy_asset**](AssetsApi.md#copy_asset) | **PUT** /assets/copy | Copy asset
[**delete_asset_metadata**](AssetsApi.md#delete_asset_metadata) | **DELETE** /assets/{id}/metadata/{key} | Delete asset metadata by key
[**delete_assets**](AssetsApi.md#delete_assets) | **DELETE** /assets | Delete assets
[**delete_bulk_asset_metadata**](AssetsApi.md#delete_bulk_asset_metadata) | **DELETE** /assets/metadata | Delete asset metadata
[**download_asset**](AssetsApi.md#download_asset) | **GET** /assets/{id}/original | Download original asset
[**edit_asset**](AssetsApi.md#edit_asset) | **PUT** /assets/{id}/edits | Apply edits to an existing asset
[**get_asset_edits**](AssetsApi.md#get_asset_edits) | **GET** /assets/{id}/edits | Retrieve edits for an existing asset
[**get_asset_info**](AssetsApi.md#get_asset_info) | **GET** /assets/{id} | Retrieve an asset
[**get_asset_metadata**](AssetsApi.md#get_asset_metadata) | **GET** /assets/{id}/metadata | Get asset metadata
[**get_asset_metadata_by_key**](AssetsApi.md#get_asset_metadata_by_key) | **GET** /assets/{id}/metadata/{key} | Retrieve asset metadata by key
[**get_asset_ocr**](AssetsApi.md#get_asset_ocr) | **GET** /assets/{id}/ocr | Retrieve asset OCR data
[**get_asset_statistics**](AssetsApi.md#get_asset_statistics) | **GET** /assets/statistics | Get asset statistics
[**play_asset_video**](AssetsApi.md#play_asset_video) | **GET** /assets/{id}/video/playback | Play asset video
[**remove_asset_edits**](AssetsApi.md#remove_asset_edits) | **DELETE** /assets/{id}/edits | Remove edits from an existing asset
[**run_asset_jobs**](AssetsApi.md#run_asset_jobs) | **POST** /assets/jobs | Run an asset job
[**update_asset**](AssetsApi.md#update_asset) | **PUT** /assets/{id} | Update an asset
[**update_asset_metadata**](AssetsApi.md#update_asset_metadata) | **PUT** /assets/{id}/metadata | Update asset metadata
[**update_assets**](AssetsApi.md#update_assets) | **PUT** /assets | Update assets
[**update_bulk_asset_metadata**](AssetsApi.md#update_bulk_asset_metadata) | **PUT** /assets/metadata | Upsert asset metadata
[**upload_asset**](AssetsApi.md#upload_asset) | **POST** /assets | Upload asset
[**view_asset**](AssetsApi.md#view_asset) | **GET** /assets/{id}/thumbnail | View asset thumbnail



## check_bulk_upload

> models::AssetBulkUploadCheckResponseDto check_bulk_upload(asset_bulk_upload_check_dto)
Check bulk upload

Determine which assets have already been uploaded to the server based on their SHA1 checksums.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_bulk_upload_check_dto** | [**AssetBulkUploadCheckDto**](AssetBulkUploadCheckDto.md) |  | [required] |

### Return type

[**models::AssetBulkUploadCheckResponseDto**](AssetBulkUploadCheckResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## copy_asset

> copy_asset(asset_copy_dto)
Copy asset

Copy asset information like albums, tags, etc. from one asset to another.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_copy_dto** | [**AssetCopyDto**](AssetCopyDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_asset_metadata

> delete_asset_metadata(id, key)
Delete asset metadata by key

Delete a specific metadata key-value pair associated with the specified asset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Asset ID | [required] |
**key** | **String** | Metadata key | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_assets

> delete_assets(asset_bulk_delete_dto)
Delete assets

Deletes multiple assets at the same time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_bulk_delete_dto** | [**AssetBulkDeleteDto**](AssetBulkDeleteDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_bulk_asset_metadata

> delete_bulk_asset_metadata(asset_metadata_bulk_delete_dto)
Delete asset metadata

Delete metadata key-value pairs for multiple assets.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_metadata_bulk_delete_dto** | [**AssetMetadataBulkDeleteDto**](AssetMetadataBulkDeleteDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_asset

> std::path::PathBuf download_asset(id, edited, key, slug)
Download original asset

Downloads the original file of the specified asset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**edited** | Option<**bool**> | Return edited asset if available |  |[default to false]
**key** | Option<**String**> |  |  |
**slug** | Option<**String**> |  |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_asset

> models::AssetEditsResponseDto edit_asset(id, asset_edits_create_dto)
Apply edits to an existing asset

Apply a series of edit actions (crop, rotate, mirror) to the specified asset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**asset_edits_create_dto** | [**AssetEditsCreateDto**](AssetEditsCreateDto.md) |  | [required] |

### Return type

[**models::AssetEditsResponseDto**](AssetEditsResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_asset_edits

> models::AssetEditsResponseDto get_asset_edits(id)
Retrieve edits for an existing asset

Retrieve a series of edit actions (crop, rotate, mirror) associated with the specified asset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::AssetEditsResponseDto**](AssetEditsResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_asset_info

> models::AssetResponseDto get_asset_info(id, key, slug)
Retrieve an asset

Retrieve detailed information about a specific asset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**key** | Option<**String**> |  |  |
**slug** | Option<**String**> |  |  |

### Return type

[**models::AssetResponseDto**](AssetResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_asset_metadata

> Vec<models::AssetMetadataResponseDto> get_asset_metadata(id)
Get asset metadata

Retrieve all metadata key-value pairs associated with the specified asset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**Vec<models::AssetMetadataResponseDto>**](AssetMetadataResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_asset_metadata_by_key

> models::AssetMetadataResponseDto get_asset_metadata_by_key(id, key)
Retrieve asset metadata by key

Retrieve the value of a specific metadata key associated with the specified asset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Asset ID | [required] |
**key** | **String** | Metadata key | [required] |

### Return type

[**models::AssetMetadataResponseDto**](AssetMetadataResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_asset_ocr

> Vec<models::AssetOcrResponseDto> get_asset_ocr(id)
Retrieve asset OCR data

Retrieve all OCR (Optical Character Recognition) data associated with the specified asset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**Vec<models::AssetOcrResponseDto>**](AssetOcrResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_asset_statistics

> models::AssetStatsResponseDto get_asset_statistics(is_favorite, is_trashed, visibility)
Get asset statistics

Retrieve various statistics about the assets owned by the authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**is_favorite** | Option<**bool**> | Filter by favorite status |  |
**is_trashed** | Option<**bool**> | Filter by trash status |  |
**visibility** | Option<[**AssetVisibility**](AssetVisibility.md)> |  |  |

### Return type

[**models::AssetStatsResponseDto**](AssetStatsResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## play_asset_video

> std::path::PathBuf play_asset_video(id, key, slug)
Play asset video

Streams the video file for the specified asset. This endpoint also supports byte range requests.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**key** | Option<**String**> |  |  |
**slug** | Option<**String**> |  |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_asset_edits

> remove_asset_edits(id)
Remove edits from an existing asset

Removes all edit actions (crop, rotate, mirror) associated with the specified asset.

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


## run_asset_jobs

> run_asset_jobs(asset_jobs_dto)
Run an asset job

Run a specific job on a set of assets.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_jobs_dto** | [**AssetJobsDto**](AssetJobsDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_asset

> models::AssetResponseDto update_asset(id, update_asset_dto)
Update an asset

Update information of a specific asset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**update_asset_dto** | [**UpdateAssetDto**](UpdateAssetDto.md) |  | [required] |

### Return type

[**models::AssetResponseDto**](AssetResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_asset_metadata

> Vec<models::AssetMetadataResponseDto> update_asset_metadata(id, asset_metadata_upsert_dto)
Update asset metadata

Update or add metadata key-value pairs for the specified asset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**asset_metadata_upsert_dto** | [**AssetMetadataUpsertDto**](AssetMetadataUpsertDto.md) |  | [required] |

### Return type

[**Vec<models::AssetMetadataResponseDto>**](AssetMetadataResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_assets

> update_assets(asset_bulk_update_dto)
Update assets

Updates multiple assets at the same time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_bulk_update_dto** | [**AssetBulkUpdateDto**](AssetBulkUpdateDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_bulk_asset_metadata

> Vec<models::AssetMetadataBulkResponseDto> update_bulk_asset_metadata(asset_metadata_bulk_upsert_dto)
Upsert asset metadata

Upsert metadata key-value pairs for multiple assets.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_metadata_bulk_upsert_dto** | [**AssetMetadataBulkUpsertDto**](AssetMetadataBulkUpsertDto.md) |  | [required] |

### Return type

[**Vec<models::AssetMetadataBulkResponseDto>**](AssetMetadataBulkResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_asset

> models::AssetMediaResponseDto upload_asset(asset_data, file_created_at, file_modified_at, key, slug, x_immich_checksum, duration, filename, is_favorite, live_photo_video_id, metadata, sidecar_data, visibility)
Upload asset

Uploads a new asset to the server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_data** | **std::path::PathBuf** | Asset file data | [required] |
**file_created_at** | **chrono::DateTime<chrono::FixedOffset>** | File creation date | [required] |
**file_modified_at** | **chrono::DateTime<chrono::FixedOffset>** | File modification date | [required] |
**key** | Option<**String**> |  |  |
**slug** | Option<**String**> |  |  |
**x_immich_checksum** | Option<**String**> | sha1 checksum that can be used for duplicate detection before the file is uploaded |  |
**duration** | Option<**i32**> | Duration in milliseconds (for videos) |  |
**filename** | Option<**String**> | Filename |  |
**is_favorite** | Option<**bool**> | Mark as favorite |  |
**live_photo_video_id** | Option<**uuid::Uuid**> | Live photo video ID |  |
**metadata** | Option<[**Vec<models::AssetMetadataUpsertItemDto>**](Models__AssetMetadataUpsertItemDto.md)> | Asset metadata items |  |
**sidecar_data** | Option<**std::path::PathBuf**> | Sidecar file data |  |
**visibility** | Option<[**models::AssetVisibility**](AssetVisibility.md)> |  |  |

### Return type

[**models::AssetMediaResponseDto**](AssetMediaResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## view_asset

> std::path::PathBuf view_asset(id, edited, key, size, slug)
View asset thumbnail

Retrieve the thumbnail image for the specified asset. Viewing the fullsize thumbnail might redirect to downloadAsset, which requires a different permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**edited** | Option<**bool**> | Return edited asset if available |  |[default to false]
**key** | Option<**String**> |  |  |
**size** | Option<[**AssetMediaSize**](AssetMediaSize.md)> |  |  |
**slug** | Option<**String**> |  |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

