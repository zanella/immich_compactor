# \ServerApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_server_license**](ServerApi.md#delete_server_license) | **DELETE** /server/license | Delete server product key
[**get_about_info**](ServerApi.md#get_about_info) | **GET** /server/about | Get server information
[**get_apk_links**](ServerApi.md#get_apk_links) | **GET** /server/apk-links | Get APK links
[**get_server_config**](ServerApi.md#get_server_config) | **GET** /server/config | Get config
[**get_server_features**](ServerApi.md#get_server_features) | **GET** /server/features | Get features
[**get_server_license**](ServerApi.md#get_server_license) | **GET** /server/license | Get product key
[**get_server_statistics**](ServerApi.md#get_server_statistics) | **GET** /server/statistics | Get statistics
[**get_server_version**](ServerApi.md#get_server_version) | **GET** /server/version | Get server version
[**get_storage**](ServerApi.md#get_storage) | **GET** /server/storage | Get storage
[**get_supported_media_types**](ServerApi.md#get_supported_media_types) | **GET** /server/media-types | Get supported media types
[**get_version_check**](ServerApi.md#get_version_check) | **GET** /server/version-check | Get version check status
[**get_version_history**](ServerApi.md#get_version_history) | **GET** /server/version-history | Get version history
[**ping_server**](ServerApi.md#ping_server) | **GET** /server/ping | Ping
[**set_server_license**](ServerApi.md#set_server_license) | **PUT** /server/license | Set server product key



## delete_server_license

> delete_server_license()
Delete server product key

Delete the currently set server product key.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_about_info

> models::ServerAboutResponseDto get_about_info()
Get server information

Retrieve a list of information about the server.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ServerAboutResponseDto**](ServerAboutResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_apk_links

> models::ServerApkLinksDto get_apk_links()
Get APK links

Retrieve links to the APKs for the current server version.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ServerApkLinksDto**](ServerApkLinksDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_server_config

> models::ServerConfigDto get_server_config()
Get config

Retrieve the current server configuration.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ServerConfigDto**](ServerConfigDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_server_features

> models::ServerFeaturesDto get_server_features()
Get features

Retrieve available features supported by this server.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ServerFeaturesDto**](ServerFeaturesDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_server_license

> models::UserLicense get_server_license()
Get product key

Retrieve information about whether the server currently has a product key registered.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UserLicense**](UserLicense.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_server_statistics

> models::ServerStatsResponseDto get_server_statistics()
Get statistics

Retrieve statistics about the entire Immich instance such as asset counts.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ServerStatsResponseDto**](ServerStatsResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_server_version

> models::ServerVersionResponseDto get_server_version()
Get server version

Retrieve the current server version in semantic versioning (semver) format.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ServerVersionResponseDto**](ServerVersionResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_storage

> models::ServerStorageResponseDto get_storage()
Get storage

Retrieve the current storage utilization information of the server.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ServerStorageResponseDto**](ServerStorageResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_supported_media_types

> models::ServerMediaTypesResponseDto get_supported_media_types()
Get supported media types

Retrieve all media types supported by the server.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ServerMediaTypesResponseDto**](ServerMediaTypesResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_version_check

> models::VersionCheckStateResponseDto get_version_check()
Get version check status

Retrieve information about the last time the version check ran.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::VersionCheckStateResponseDto**](VersionCheckStateResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_version_history

> Vec<models::ServerVersionHistoryResponseDto> get_version_history()
Get version history

Retrieve a list of past versions the server has been on.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ServerVersionHistoryResponseDto>**](ServerVersionHistoryResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ping_server

> models::ServerPingResponse ping_server()
Ping

Pong

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ServerPingResponse**](ServerPingResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_server_license

> models::UserLicense set_server_license(license_key_dto)
Set server product key

Validate and set the server product key if successful.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**license_key_dto** | [**LicenseKeyDto**](LicenseKeyDto.md) |  | [required] |

### Return type

[**models::UserLicense**](UserLicense.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

