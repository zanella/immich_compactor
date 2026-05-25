# SharedLinkResponseDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**album** | Option<[**models::AlbumResponseDto**](AlbumResponseDto.md)> |  | [optional]
**allow_download** | **bool** | Allow downloads | 
**allow_upload** | **bool** | Allow uploads | 
**assets** | [**Vec<models::AssetResponseDto>**](AssetResponseDto.md) |  | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** | Creation date | 
**description** | Option<**String**> | Link description | 
**expires_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Expiration date | 
**id** | **String** | Shared link ID | 
**key** | **String** | Encryption key (base64url) | 
**password** | Option<**String**> | Has password | 
**show_metadata** | **bool** | Show metadata | 
**slug** | Option<**String**> | Custom URL slug | 
**r#type** | [**models::SharedLinkType**](SharedLinkType.md) |  | 
**user_id** | **String** | Owner user ID | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


