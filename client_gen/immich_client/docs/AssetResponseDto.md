# AssetResponseDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**checksum** | **String** | Base64 encoded SHA1 hash | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** | The UTC timestamp when the asset was originally uploaded to Immich. | 
**duplicate_id** | Option<**String**> | Duplicate group ID | [optional]
**duration** | Option<**i32**> | Video/gif duration in milliseconds (null for static images) | 
**exif_info** | Option<[**models::ExifResponseDto**](ExifResponseDto.md)> |  | [optional]
**file_created_at** | **chrono::DateTime<chrono::FixedOffset>** | The actual UTC timestamp when the file was created/captured, preserving timezone information. This is the authoritative timestamp for chronological sorting within timeline groups. Combined with timezone data, this can be used to determine the exact moment the photo was taken. | 
**file_modified_at** | **chrono::DateTime<chrono::FixedOffset>** | The UTC timestamp when the file was last modified on the filesystem. This reflects the last time the physical file was changed, which may be different from when the photo was originally taken. | 
**has_metadata** | **bool** | Whether asset has metadata | 
**height** | Option<**i32**> | Asset height | 
**id** | **String** | Asset ID | 
**is_archived** | **bool** | Is archived | 
**is_edited** | **bool** | Is edited | 
**is_favorite** | **bool** | Is favorite | 
**is_offline** | **bool** | Is offline | 
**is_trashed** | **bool** | Is trashed | 
**library_id** | Option<**uuid::Uuid**> | Library ID | [optional]
**live_photo_video_id** | Option<**String**> | Live photo video ID | [optional]
**local_date_time** | **chrono::DateTime<chrono::FixedOffset>** | The local date and time when the photo/video was taken, derived from EXIF metadata. This represents the photographer's local time regardless of timezone, stored as a timezone-agnostic timestamp. Used for timeline grouping by \"local\" days and months. | 
**original_file_name** | **String** | Original file name | 
**original_mime_type** | Option<**String**> | Original MIME type | [optional]
**original_path** | **String** | Original file path | 
**owner** | Option<[**models::UserResponseDto**](UserResponseDto.md)> |  | [optional]
**owner_id** | **String** | Owner user ID | 
**people** | Option<[**Vec<models::PersonResponseDto>**](PersonResponseDto.md)> |  | [optional]
**resized** | Option<**bool**> | Is resized | [optional]
**stack** | Option<[**models::AssetStackResponseDto**](AssetStackResponseDto.md)> |  | [optional]
**tags** | Option<[**Vec<models::TagResponseDto>**](TagResponseDto.md)> |  | [optional]
**thumbhash** | Option<**String**> | Thumbhash for thumbnail generation (base64) also used as the c query param for thumbnail cache busting. | 
**r#type** | [**models::AssetTypeEnum**](AssetTypeEnum.md) |  | 
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** | The UTC timestamp when the asset record was last updated in the database. This is automatically maintained by the database and reflects when any field in the asset was last modified. | 
**visibility** | [**models::AssetVisibility**](AssetVisibility.md) |  | 
**width** | Option<**i32**> | Asset width | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


