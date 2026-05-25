# ActivityCreateDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**album_id** | **uuid::Uuid** | Album ID | 
**asset_id** | Option<**uuid::Uuid**> | Asset ID (if activity is for an asset) | [optional]
**comment** | Option<**String**> | Comment text (required if type is comment) | [optional]
**r#type** | [**models::ReactionType**](ReactionType.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


