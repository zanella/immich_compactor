# \ActivitiesApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_activity**](ActivitiesApi.md#create_activity) | **POST** /activities | Create an activity
[**delete_activity**](ActivitiesApi.md#delete_activity) | **DELETE** /activities/{id} | Delete an activity
[**get_activities**](ActivitiesApi.md#get_activities) | **GET** /activities | List all activities
[**get_activity_statistics**](ActivitiesApi.md#get_activity_statistics) | **GET** /activities/statistics | Retrieve activity statistics



## create_activity

> models::ActivityResponseDto create_activity(activity_create_dto)
Create an activity

Create a like or a comment for an album, or an asset in an album.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**activity_create_dto** | [**ActivityCreateDto**](ActivityCreateDto.md) |  | [required] |

### Return type

[**models::ActivityResponseDto**](ActivityResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_activity

> delete_activity(id)
Delete an activity

Removes a like or comment from a given album or asset in an album.

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


## get_activities

> Vec<models::ActivityResponseDto> get_activities(album_id, asset_id, level, r#type, user_id)
List all activities

Returns a list of activities for the selected asset or album. The activities are returned in sorted order, with the oldest activities appearing first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**album_id** | **uuid::Uuid** | Album ID | [required] |
**asset_id** | Option<**uuid::Uuid**> | Asset ID (if activity is for an asset) |  |
**level** | Option<[**ReactionLevel**](ReactionLevel.md)> |  |  |
**r#type** | Option<[**ReactionType**](ReactionType.md)> |  |  |
**user_id** | Option<**uuid::Uuid**> | Filter by user ID |  |

### Return type

[**Vec<models::ActivityResponseDto>**](ActivityResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_activity_statistics

> models::ActivityStatisticsResponseDto get_activity_statistics(album_id, asset_id)
Retrieve activity statistics

Returns the number of likes and comments for a given album or asset in an album.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**album_id** | **uuid::Uuid** | Album ID | [required] |
**asset_id** | Option<**uuid::Uuid**> | Asset ID (if activity is for an asset) |  |

### Return type

[**models::ActivityStatisticsResponseDto**](ActivityStatisticsResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

