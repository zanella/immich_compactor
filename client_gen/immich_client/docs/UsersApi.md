# \UsersApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_profile_image**](UsersApi.md#create_profile_image) | **POST** /users/profile-image | Create user profile image
[**delete_profile_image**](UsersApi.md#delete_profile_image) | **DELETE** /users/profile-image | Delete user profile image
[**delete_user_license**](UsersApi.md#delete_user_license) | **DELETE** /users/me/license | Delete user product key
[**delete_user_onboarding**](UsersApi.md#delete_user_onboarding) | **DELETE** /users/me/onboarding | Delete user onboarding
[**get_my_preferences**](UsersApi.md#get_my_preferences) | **GET** /users/me/preferences | Get my preferences
[**get_my_user**](UsersApi.md#get_my_user) | **GET** /users/me | Get current user
[**get_profile_image**](UsersApi.md#get_profile_image) | **GET** /users/{id}/profile-image | Retrieve user profile image
[**get_user**](UsersApi.md#get_user) | **GET** /users/{id} | Retrieve a user
[**get_user_license**](UsersApi.md#get_user_license) | **GET** /users/me/license | Retrieve user product key
[**get_user_onboarding**](UsersApi.md#get_user_onboarding) | **GET** /users/me/onboarding | Retrieve user onboarding
[**search_users**](UsersApi.md#search_users) | **GET** /users | Get all users
[**set_user_license**](UsersApi.md#set_user_license) | **PUT** /users/me/license | Set user product key
[**set_user_onboarding**](UsersApi.md#set_user_onboarding) | **PUT** /users/me/onboarding | Update user onboarding
[**update_my_preferences**](UsersApi.md#update_my_preferences) | **PUT** /users/me/preferences | Update my preferences
[**update_my_user**](UsersApi.md#update_my_user) | **PUT** /users/me | Update current user



## create_profile_image

> models::CreateProfileImageResponseDto create_profile_image(file)
Create user profile image

Upload and set a new profile image for the current user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file** | **std::path::PathBuf** | Profile image file | [required] |

### Return type

[**models::CreateProfileImageResponseDto**](CreateProfileImageResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_profile_image

> delete_profile_image()
Delete user profile image

Delete the profile image of the current user.

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


## delete_user_license

> delete_user_license()
Delete user product key

Delete the registered product key for the current user.

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


## delete_user_onboarding

> delete_user_onboarding()
Delete user onboarding

Delete the onboarding status of the current user.

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


## get_my_preferences

> models::UserPreferencesResponseDto get_my_preferences()
Get my preferences

Retrieve the preferences for the current user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UserPreferencesResponseDto**](UserPreferencesResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_user

> models::UserAdminResponseDto get_my_user()
Get current user

Retrieve information about the user making the API request.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UserAdminResponseDto**](UserAdminResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_profile_image

> std::path::PathBuf get_profile_image(id)
Retrieve user profile image

Retrieve the profile image file for a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user

> models::UserResponseDto get_user(id)
Retrieve a user

Retrieve a specific user by their ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::UserResponseDto**](UserResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_license

> models::UserLicense get_user_license()
Retrieve user product key

Retrieve information about whether the current user has a registered product key.

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


## get_user_onboarding

> models::OnboardingResponseDto get_user_onboarding()
Retrieve user onboarding

Retrieve the onboarding status of the current user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::OnboardingResponseDto**](OnboardingResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_users

> Vec<models::UserResponseDto> search_users()
Get all users

Retrieve a list of all users on the server.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::UserResponseDto>**](UserResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_user_license

> models::UserLicense set_user_license(license_key_dto)
Set user product key

Register a product key for the current user.

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


## set_user_onboarding

> models::OnboardingResponseDto set_user_onboarding(onboarding_dto)
Update user onboarding

Update the onboarding status of the current user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**onboarding_dto** | [**OnboardingDto**](OnboardingDto.md) |  | [required] |

### Return type

[**models::OnboardingResponseDto**](OnboardingResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_my_preferences

> models::UserPreferencesResponseDto update_my_preferences(user_preferences_update_dto)
Update my preferences

Update the preferences of the current user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_preferences_update_dto** | [**UserPreferencesUpdateDto**](UserPreferencesUpdateDto.md) |  | [required] |

### Return type

[**models::UserPreferencesResponseDto**](UserPreferencesResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_my_user

> models::UserAdminResponseDto update_my_user(user_update_me_dto)
Update current user

Update the current user making the API request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_update_me_dto** | [**UserUpdateMeDto**](UserUpdateMeDto.md) |  | [required] |

### Return type

[**models::UserAdminResponseDto**](UserAdminResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

