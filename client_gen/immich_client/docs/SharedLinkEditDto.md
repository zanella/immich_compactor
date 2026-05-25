# SharedLinkEditDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allow_download** | Option<**bool**> | Allow downloads | [optional]
**allow_upload** | Option<**bool**> | Allow uploads | [optional]
**change_expiry_time** | Option<**bool**> | Whether to change the expiry time. Few clients cannot send null to set the expiryTime to never. Setting this flag and not sending expiryAt is considered as null instead. Clients that can send null values can ignore this. | [optional]
**description** | Option<**String**> | Link description | [optional]
**expires_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Expiration date | [optional]
**password** | Option<**String**> | Link password | [optional]
**show_metadata** | Option<**bool**> | Show metadata | [optional]
**slug** | Option<**String**> | Custom URL slug | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


