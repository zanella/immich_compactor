# UserAdminResponseDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**avatar_color** | [**models::UserAvatarColor**](UserAvatarColor.md) |  | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** | Creation date | 
**deleted_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Deletion date | 
**email** | **String** | User email | 
**id** | **uuid::Uuid** | User ID | 
**is_admin** | **bool** | Is admin user | 
**license** | Option<[**models::UserLicense**](UserLicense.md)> |  | 
**name** | **String** | User name | 
**oauth_id** | **String** | OAuth ID | 
**profile_changed_at** | **chrono::DateTime<chrono::FixedOffset>** | Profile change date | 
**profile_image_path** | **String** | Profile image path | 
**quota_size_in_bytes** | Option<**i32**> | Storage quota in bytes | 
**quota_usage_in_bytes** | Option<**i32**> | Storage usage in bytes | 
**should_change_password** | **bool** | Require password change on next login | 
**status** | [**models::UserStatus**](UserStatus.md) |  | 
**storage_label** | Option<**String**> | Storage label | 
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** | Last update date | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


