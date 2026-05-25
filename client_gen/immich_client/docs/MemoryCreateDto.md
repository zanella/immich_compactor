# MemoryCreateDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asset_ids** | Option<**Vec<uuid::Uuid>**> | Asset IDs to associate with memory | [optional]
**data** | [**models::OnThisDayDto**](OnThisDayDto.md) |  | 
**hide_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Date when memory should be hidden | [optional]
**is_saved** | Option<**bool**> | Is memory saved | [optional]
**memory_at** | **chrono::DateTime<chrono::FixedOffset>** | Memory date | 
**seen_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Date when memory was seen | [optional]
**show_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Date when memory should be shown | [optional]
**r#type** | [**models::MemoryType**](MemoryType.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


