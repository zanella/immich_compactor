# \TimelineApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_time_bucket**](TimelineApi.md#get_time_bucket) | **GET** /timeline/bucket | Get time bucket
[**get_time_buckets**](TimelineApi.md#get_time_buckets) | **GET** /timeline/buckets | Get time buckets



## get_time_bucket

> models::TimeBucketAssetResponseDto get_time_bucket(time_bucket, album_id, bbox, is_favorite, is_trashed, key, order, order_by, person_id, slug, tag_id, user_id, visibility, with_coordinates, with_partners, with_stacked)
Get time bucket

Retrieve a string of all asset ids in a given time bucket.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**time_bucket** | **String** | Time bucket identifier in YYYY-MM-DD format | [required] |
**album_id** | Option<**uuid::Uuid**> | Filter assets belonging to a specific album |  |
**bbox** | Option<**String**> | Bounding box coordinates as west,south,east,north (WGS84) |  |
**is_favorite** | Option<**bool**> | Filter by favorite status (true for favorites only, false for non-favorites only) |  |
**is_trashed** | Option<**bool**> | Filter by trash status (true for trashed assets only, false for non-trashed only) |  |
**key** | Option<**String**> |  |  |
**order** | Option<[**AssetOrder**](AssetOrder.md)> | Sort order for assets within time buckets (ASC for oldest first, DESC for newest first) |  |
**order_by** | Option<[**AssetOrderBy**](AssetOrderBy.md)> | Date to group and order assets by (takenAt for date taken, createdAt for date added to Immich) |  |
**person_id** | Option<**uuid::Uuid**> | Filter assets containing a specific person (face recognition) |  |
**slug** | Option<**String**> |  |  |
**tag_id** | Option<**uuid::Uuid**> | Filter assets with a specific tag |  |
**user_id** | Option<**uuid::Uuid**> | Filter assets by specific user ID |  |
**visibility** | Option<[**AssetVisibility**](AssetVisibility.md)> | Filter by asset visibility status (ARCHIVE, TIMELINE, HIDDEN, LOCKED) |  |
**with_coordinates** | Option<**bool**> | Include location data in the response |  |
**with_partners** | Option<**bool**> | Include assets shared by partners |  |
**with_stacked** | Option<**bool**> | Include stacked assets in the response. When true, only primary assets from stacks are returned. |  |

### Return type

[**models::TimeBucketAssetResponseDto**](TimeBucketAssetResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_time_buckets

> Vec<models::TimeBucketsResponseDto> get_time_buckets(album_id, bbox, is_favorite, is_trashed, key, order, order_by, person_id, slug, tag_id, user_id, visibility, with_coordinates, with_partners, with_stacked)
Get time buckets

Retrieve a list of all minimal time buckets.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**album_id** | Option<**uuid::Uuid**> | Filter assets belonging to a specific album |  |
**bbox** | Option<**String**> | Bounding box coordinates as west,south,east,north (WGS84) |  |
**is_favorite** | Option<**bool**> | Filter by favorite status (true for favorites only, false for non-favorites only) |  |
**is_trashed** | Option<**bool**> | Filter by trash status (true for trashed assets only, false for non-trashed only) |  |
**key** | Option<**String**> |  |  |
**order** | Option<[**AssetOrder**](AssetOrder.md)> | Sort order for assets within time buckets (ASC for oldest first, DESC for newest first) |  |
**order_by** | Option<[**AssetOrderBy**](AssetOrderBy.md)> | Date to group and order assets by (takenAt for date taken, createdAt for date added to Immich) |  |
**person_id** | Option<**uuid::Uuid**> | Filter assets containing a specific person (face recognition) |  |
**slug** | Option<**String**> |  |  |
**tag_id** | Option<**uuid::Uuid**> | Filter assets with a specific tag |  |
**user_id** | Option<**uuid::Uuid**> | Filter assets by specific user ID |  |
**visibility** | Option<[**AssetVisibility**](AssetVisibility.md)> | Filter by asset visibility status (ARCHIVE, TIMELINE, HIDDEN, LOCKED) |  |
**with_coordinates** | Option<**bool**> | Include location data in the response |  |
**with_partners** | Option<**bool**> | Include assets shared by partners |  |
**with_stacked** | Option<**bool**> | Include stacked assets in the response. When true, only primary assets from stacks are returned. |  |

### Return type

[**Vec<models::TimeBucketsResponseDto>**](TimeBucketsResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

