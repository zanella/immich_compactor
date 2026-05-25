# \DatabaseBackupsAdminApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_database_backup**](DatabaseBackupsAdminApi.md#delete_database_backup) | **DELETE** /admin/database-backups | Delete database backup
[**download_database_backup**](DatabaseBackupsAdminApi.md#download_database_backup) | **GET** /admin/database-backups/{filename} | Download database backup
[**list_database_backups**](DatabaseBackupsAdminApi.md#list_database_backups) | **GET** /admin/database-backups | List database backups
[**start_database_restore_flow**](DatabaseBackupsAdminApi.md#start_database_restore_flow) | **POST** /admin/database-backups/start-restore | Start database backup restore flow
[**upload_database_backup**](DatabaseBackupsAdminApi.md#upload_database_backup) | **POST** /admin/database-backups/upload | Upload database backup



## delete_database_backup

> delete_database_backup(database_backup_delete_dto)
Delete database backup

Delete a backup by its filename

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database_backup_delete_dto** | [**DatabaseBackupDeleteDto**](DatabaseBackupDeleteDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_database_backup

> std::path::PathBuf download_database_backup(filename)
Download database backup

Downloads the database backup file

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filename** | **String** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_database_backups

> models::DatabaseBackupListResponseDto list_database_backups()
List database backups

Get the list of the successful and failed backups

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::DatabaseBackupListResponseDto**](DatabaseBackupListResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_database_restore_flow

> start_database_restore_flow()
Start database backup restore flow

Put Immich into maintenance mode to restore a backup (Immich must not be configured)

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_database_backup

> upload_database_backup(file)
Upload database backup

Uploads .sql/.sql.gz file to restore backup from

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file** | Option<**std::path::PathBuf**> | Database backup file |  |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

