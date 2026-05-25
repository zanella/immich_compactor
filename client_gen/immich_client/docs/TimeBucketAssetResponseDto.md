# TimeBucketAssetResponseDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**city** | **Vec<String>** | Array of city names extracted from EXIF GPS data | 
**country** | **Vec<String>** | Array of country names extracted from EXIF GPS data | 
**created_at** | **Vec<String>** | Array of UTC timestamps when each asset was originally uploaded to Immich | 
**duration** | **Vec<i32>** | Array of video/gif durations in milliseconds (null for static images) | 
**file_created_at** | **Vec<String>** | Array of file creation timestamps in UTC | 
**id** | **Vec<String>** | Array of asset IDs in the time bucket | 
**is_favorite** | **Vec<bool>** | Array indicating whether each asset is favorited | 
**is_image** | **Vec<bool>** | Array indicating whether each asset is an image (false for videos) | 
**is_trashed** | **Vec<bool>** | Array indicating whether each asset is in the trash | 
**latitude** | Option<**Vec<f64>**> | Array of latitude coordinates extracted from EXIF GPS data | [optional]
**live_photo_video_id** | **Vec<String>** | Array of live photo video asset IDs (null for non-live photos) | 
**local_offset_hours** | **Vec<f64>** | Array of UTC offset hours at the time each photo was taken. Positive values are east of UTC, negative values are west of UTC. Values may be fractional (e.g., 5.5 for +05:30, -9.75 for -09:45). Applying this offset to 'fileCreatedAt' will give you the time the photo was taken from the photographer's perspective. | 
**longitude** | Option<**Vec<f64>**> | Array of longitude coordinates extracted from EXIF GPS data | [optional]
**owner_id** | **Vec<String>** | Array of owner IDs for each asset | 
**projection_type** | **Vec<String>** | Array of projection types for 360° content (e.g., \"EQUIRECTANGULAR\", \"CUBEFACE\", \"CYLINDRICAL\") | 
**ratio** | **Vec<f64>** | Array of aspect ratios (width/height) for each asset | 
**stack** | Option<[**Vec<Vec<String>>**](Vec.md)> | Array of stack information as [stackId, assetCount] tuples (null for non-stacked assets) | [optional]
**thumbhash** | **Vec<String>** | Array of BlurHash strings for generating asset previews (base64 encoded) | 
**visibility** | [**Vec<models::AssetVisibility>**](AssetVisibility.md) | Array of visibility statuses for each asset (e.g., ARCHIVE, TIMELINE, HIDDEN, LOCKED) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


