# MemoryResponseDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assets** | [**Vec<models::AssetResponseDto>**](AssetResponseDto.md) |  | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** | Creation date | 
**data** | [**models::OnThisDayDto**](OnThisDayDto.md) |  | 
**deleted_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Deletion date | [optional]
**hide_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Date when memory should be hidden | [optional]
**id** | **String** | Memory ID | 
**is_saved** | **bool** | Is memory saved | 
**memory_at** | **chrono::DateTime<chrono::FixedOffset>** | Memory date | 
**owner_id** | **String** | Owner user ID | 
**seen_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Date when memory was seen | [optional]
**show_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Date when memory should be shown | [optional]
**r#type** | [**models::MemoryType**](MemoryType.md) |  | 
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** | Last update date | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


