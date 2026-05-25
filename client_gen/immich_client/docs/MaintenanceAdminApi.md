# \MaintenanceAdminApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**detect_prior_install**](MaintenanceAdminApi.md#detect_prior_install) | **GET** /admin/maintenance/detect-install | Detect existing install
[**get_maintenance_status**](MaintenanceAdminApi.md#get_maintenance_status) | **GET** /admin/maintenance/status | Get maintenance mode status
[**maintenance_login**](MaintenanceAdminApi.md#maintenance_login) | **POST** /admin/maintenance/login | Log into maintenance mode
[**set_maintenance_mode**](MaintenanceAdminApi.md#set_maintenance_mode) | **POST** /admin/maintenance | Set maintenance mode



## detect_prior_install

> models::MaintenanceDetectInstallResponseDto detect_prior_install()
Detect existing install

Collect integrity checks and other heuristics about local data.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::MaintenanceDetectInstallResponseDto**](MaintenanceDetectInstallResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_maintenance_status

> models::MaintenanceStatusResponseDto get_maintenance_status()
Get maintenance mode status

Fetch information about the currently running maintenance action.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::MaintenanceStatusResponseDto**](MaintenanceStatusResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## maintenance_login

> models::MaintenanceAuthDto maintenance_login(maintenance_login_dto)
Log into maintenance mode

Login with maintenance token or cookie to receive current information and perform further actions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**maintenance_login_dto** | [**MaintenanceLoginDto**](MaintenanceLoginDto.md) |  | [required] |

### Return type

[**models::MaintenanceAuthDto**](MaintenanceAuthDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_maintenance_mode

> set_maintenance_mode(set_maintenance_mode_dto)
Set maintenance mode

Put Immich into or take it out of maintenance mode

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**set_maintenance_mode_dto** | [**SetMaintenanceModeDto**](SetMaintenanceModeDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

