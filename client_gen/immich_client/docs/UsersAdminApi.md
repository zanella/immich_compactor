# \UsersAdminApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_user_admin**](UsersAdminApi.md#create_user_admin) | **POST** /admin/users | Create a user
[**delete_user_admin**](UsersAdminApi.md#delete_user_admin) | **DELETE** /admin/users/{id} | Delete a user
[**get_user_admin**](UsersAdminApi.md#get_user_admin) | **GET** /admin/users/{id} | Retrieve a user
[**get_user_preferences_admin**](UsersAdminApi.md#get_user_preferences_admin) | **GET** /admin/users/{id}/preferences | Retrieve user preferences
[**get_user_sessions_admin**](UsersAdminApi.md#get_user_sessions_admin) | **GET** /admin/users/{id}/sessions | Retrieve user sessions
[**get_user_statistics_admin**](UsersAdminApi.md#get_user_statistics_admin) | **GET** /admin/users/{id}/statistics | Retrieve user statistics
[**restore_user_admin**](UsersAdminApi.md#restore_user_admin) | **POST** /admin/users/{id}/restore | Restore a deleted user
[**search_users_admin**](UsersAdminApi.md#search_users_admin) | **GET** /admin/users | Search users
[**update_user_admin**](UsersAdminApi.md#update_user_admin) | **PUT** /admin/users/{id} | Update a user
[**update_user_preferences_admin**](UsersAdminApi.md#update_user_preferences_admin) | **PUT** /admin/users/{id}/preferences | Update user preferences



## create_user_admin

> models::UserAdminResponseDto create_user_admin(user_admin_create_dto)
Create a user

Create a new user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_admin_create_dto** | [**UserAdminCreateDto**](UserAdminCreateDto.md) |  | [required] |

### Return type

[**models::UserAdminResponseDto**](UserAdminResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_admin

> models::UserAdminResponseDto delete_user_admin(id, user_admin_delete_dto)
Delete a user

Delete a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**user_admin_delete_dto** | [**UserAdminDeleteDto**](UserAdminDeleteDto.md) |  | [required] |

### Return type

[**models::UserAdminResponseDto**](UserAdminResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_admin

> models::UserAdminResponseDto get_user_admin(id)
Retrieve a user

Retrieve  a specific user by their ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::UserAdminResponseDto**](UserAdminResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_preferences_admin

> models::UserPreferencesResponseDto get_user_preferences_admin(id)
Retrieve user preferences

Retrieve the preferences of a specific user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::UserPreferencesResponseDto**](UserPreferencesResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_sessions_admin

> Vec<models::SessionResponseDto> get_user_sessions_admin(id)
Retrieve user sessions

Retrieve all sessions for a specific user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**Vec<models::SessionResponseDto>**](SessionResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_statistics_admin

> models::AssetStatsResponseDto get_user_statistics_admin(id, is_favorite, is_trashed, visibility)
Retrieve user statistics

Retrieve asset statistics for a specific user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
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


## restore_user_admin

> models::UserAdminResponseDto restore_user_admin(id)
Restore a deleted user

Restore a previously deleted user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::UserAdminResponseDto**](UserAdminResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_users_admin

> Vec<models::UserAdminResponseDto> search_users_admin(id, with_deleted)
Search users

Search for users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**uuid::Uuid**> | User ID filter |  |
**with_deleted** | Option<**bool**> | Include deleted users |  |

### Return type

[**Vec<models::UserAdminResponseDto>**](UserAdminResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_admin

> models::UserAdminResponseDto update_user_admin(id, user_admin_update_dto)
Update a user

Update an existing user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**user_admin_update_dto** | [**UserAdminUpdateDto**](UserAdminUpdateDto.md) |  | [required] |

### Return type

[**models::UserAdminResponseDto**](UserAdminResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_preferences_admin

> models::UserPreferencesResponseDto update_user_preferences_admin(id, user_preferences_update_dto)
Update user preferences

Update the preferences of a specific user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**user_preferences_update_dto** | [**UserPreferencesUpdateDto**](UserPreferencesUpdateDto.md) |  | [required] |

### Return type

[**models::UserPreferencesResponseDto**](UserPreferencesResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

