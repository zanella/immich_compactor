# SharedLinkCreateDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**album_id** | Option<**uuid::Uuid**> | Album ID (for album sharing) | [optional]
**allow_download** | Option<**bool**> | Allow downloads | [optional][default to true]
**allow_upload** | Option<**bool**> | Allow uploads | [optional]
**asset_ids** | Option<**Vec<uuid::Uuid>**> | Asset IDs (for individual assets) | [optional]
**description** | Option<**String**> | Link description | [optional]
**expires_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Expiration date | [optional]
**password** | Option<**String**> | Link password | [optional]
**show_metadata** | Option<**bool**> | Show metadata | [optional][default to true]
**slug** | Option<**String**> | Custom URL slug | [optional]
**r#type** | [**models::SharedLinkType**](SharedLinkType.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


