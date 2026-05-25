# NotificationCreateDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**data** | Option<**std::collections::HashMap<String, serde_json::Value>**> | Additional notification data | [optional]
**description** | Option<**String**> | Notification description | [optional]
**level** | Option<[**models::NotificationLevel**](NotificationLevel.md)> |  | [optional]
**read_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Date when notification was read | [optional]
**title** | **String** | Notification title | 
**r#type** | Option<[**models::NotificationType**](NotificationType.md)> |  | [optional]
**user_id** | **uuid::Uuid** | User ID to send notification to | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


