# ActivityResponseDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asset_id** | Option<**uuid::Uuid**> | Asset ID (if activity is for an asset) | 
**comment** | Option<**String**> | Comment text (for comment activities) | [optional]
**created_at** | **chrono::DateTime<chrono::FixedOffset>** | Creation date | 
**id** | **uuid::Uuid** | Activity ID | 
**r#type** | [**models::ReactionType**](ReactionType.md) |  | 
**user** | [**models::UserResponseDto**](UserResponseDto.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


