# AssetBulkUpdateDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**date_time_original** | Option<**String**> | Original date and time | [optional]
**date_time_relative** | Option<**i32**> | Relative time offset in seconds | [optional]
**description** | Option<**String**> | Asset description | [optional]
**duplicate_id** | Option<**String**> | Duplicate ID | [optional]
**ids** | **Vec<uuid::Uuid>** | Asset IDs to update | 
**is_favorite** | Option<**bool**> | Mark as favorite | [optional]
**latitude** | Option<**f64**> | Latitude coordinate | [optional]
**longitude** | Option<**f64**> | Longitude coordinate | [optional]
**rating** | Option<**i32**> | Rating in range [1-5], or null for unrated | [optional]
**time_zone** | Option<**String**> | Time zone (IANA timezone) | [optional]
**visibility** | Option<[**models::AssetVisibility**](AssetVisibility.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


