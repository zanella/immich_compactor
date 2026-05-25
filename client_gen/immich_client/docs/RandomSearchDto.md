# RandomSearchDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**album_ids** | Option<**Vec<uuid::Uuid>**> | Filter by album IDs | [optional]
**city** | Option<**String**> | Filter by city name | [optional]
**country** | Option<**String**> | Filter by country name | [optional]
**created_after** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Filter by creation date (after) | [optional]
**created_before** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Filter by creation date (before) | [optional]
**is_encoded** | Option<**bool**> | Filter by encoded status | [optional]
**is_favorite** | Option<**bool**> | Filter by favorite status | [optional]
**is_motion** | Option<**bool**> | Filter by motion photo status | [optional]
**is_not_in_album** | Option<**bool**> | Filter assets not in any album | [optional]
**is_offline** | Option<**bool**> | Filter by offline status | [optional]
**lens_model** | Option<**String**> | Filter by lens model | [optional]
**library_id** | Option<**uuid::Uuid**> | Library ID to filter by | [optional]
**make** | Option<**String**> | Filter by camera make | [optional]
**model** | Option<**String**> | Filter by camera model | [optional]
**ocr** | Option<**String**> | Filter by OCR text content | [optional]
**person_ids** | Option<**Vec<uuid::Uuid>**> | Filter by person IDs | [optional]
**rating** | Option<**i32**> | Filter by rating [1-5], or null for unrated | [optional]
**size** | Option<**i32**> | Number of results to return | [optional]
**state** | Option<**String**> | Filter by state/province name | [optional]
**tag_ids** | Option<**Vec<uuid::Uuid>**> | Filter by tag IDs | [optional]
**taken_after** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Filter by taken date (after) | [optional]
**taken_before** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Filter by taken date (before) | [optional]
**trashed_after** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Filter by trash date (after) | [optional]
**trashed_before** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Filter by trash date (before) | [optional]
**r#type** | Option<[**models::AssetTypeEnum**](AssetTypeEnum.md)> |  | [optional]
**updated_after** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Filter by update date (after) | [optional]
**updated_before** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Filter by update date (before) | [optional]
**visibility** | Option<[**models::AssetVisibility**](AssetVisibility.md)> |  | [optional]
**with_deleted** | Option<**bool**> | Include deleted assets | [optional]
**with_exif** | Option<**bool**> | Include EXIF data in response | [optional]
**with_people** | Option<**bool**> | Include people data in response | [optional]
**with_stacked** | Option<**bool**> | Include stacked assets | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


