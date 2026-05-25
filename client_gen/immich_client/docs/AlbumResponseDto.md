# AlbumResponseDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**album_name** | **String** | Album name | 
**album_thumbnail_asset_id** | Option<**String**> | Thumbnail asset ID | 
**album_users** | [**Vec<models::AlbumUserResponseDto>**](AlbumUserResponseDto.md) | First entry is always the album owner. Second entry is the auth user, if it differs from the owner. The rest are ordered alphabetically. | 
**asset_count** | **i32** | Number of assets | 
**contributor_counts** | Option<[**Vec<models::ContributorCountResponseDto>**](ContributorCountResponseDto.md)> |  | [optional]
**created_at** | **chrono::DateTime<chrono::FixedOffset>** | Creation date | 
**description** | **String** | Album description | 
**end_date** | Option<**chrono::DateTime<chrono::FixedOffset>**> | End date (latest asset) | [optional]
**has_shared_link** | **bool** | Has shared link | 
**id** | **String** | Album ID | 
**is_activity_enabled** | **bool** | Activity feed enabled | 
**last_modified_asset_timestamp** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Last modified asset timestamp | [optional]
**order** | Option<[**models::AssetOrder**](AssetOrder.md)> |  | [optional]
**shared** | **bool** | Is shared album | 
**start_date** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Start date (earliest asset) | [optional]
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** | Last update date | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


