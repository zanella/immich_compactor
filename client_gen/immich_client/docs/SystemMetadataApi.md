# \SystemMetadataApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_admin_onboarding**](SystemMetadataApi.md#get_admin_onboarding) | **GET** /system-metadata/admin-onboarding | Retrieve admin onboarding
[**get_reverse_geocoding_state**](SystemMetadataApi.md#get_reverse_geocoding_state) | **GET** /system-metadata/reverse-geocoding-state | Retrieve reverse geocoding state
[**get_version_check_state**](SystemMetadataApi.md#get_version_check_state) | **GET** /system-metadata/version-check-state | Retrieve version check state
[**update_admin_onboarding**](SystemMetadataApi.md#update_admin_onboarding) | **POST** /system-metadata/admin-onboarding | Update admin onboarding



## get_admin_onboarding

> models::AdminOnboardingUpdateDto get_admin_onboarding()
Retrieve admin onboarding

Retrieve the current admin onboarding status.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AdminOnboardingUpdateDto**](AdminOnboardingUpdateDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_reverse_geocoding_state

> models::ReverseGeocodingStateResponseDto get_reverse_geocoding_state()
Retrieve reverse geocoding state

Retrieve the current state of the reverse geocoding import.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ReverseGeocodingStateResponseDto**](ReverseGeocodingStateResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_version_check_state

> models::VersionCheckStateResponseDto get_version_check_state()
Retrieve version check state

Retrieve the current state of the version check process.

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


## update_admin_onboarding

> update_admin_onboarding(admin_onboarding_update_dto)
Update admin onboarding

Update the admin onboarding status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**admin_onboarding_update_dto** | [**AdminOnboardingUpdateDto**](AdminOnboardingUpdateDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

