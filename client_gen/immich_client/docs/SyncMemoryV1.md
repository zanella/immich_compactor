# SyncMemoryV1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **chrono::DateTime<chrono::FixedOffset>** | Created at | 
**data** | **std::collections::HashMap<String, serde_json::Value>** | Data | 
**deleted_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Deleted at | 
**hide_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Hide at | 
**id** | **String** | Memory ID | 
**is_saved** | **bool** | Is saved | 
**memory_at** | **chrono::DateTime<chrono::FixedOffset>** | Memory at | 
**owner_id** | **String** | Owner ID | 
**seen_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Seen at | 
**show_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Show at | 
**r#type** | [**models::MemoryType**](MemoryType.md) |  | 
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** | Updated at | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


