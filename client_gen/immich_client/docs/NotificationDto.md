# NotificationDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **chrono::DateTime<chrono::FixedOffset>** | Creation date | 
**data** | Option<**std::collections::HashMap<String, serde_json::Value>**> | Additional notification data | [optional]
**description** | Option<**String**> | Notification description | [optional]
**id** | **String** | Notification ID | 
**level** | [**models::NotificationLevel**](NotificationLevel.md) |  | 
**read_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Date when notification was read | [optional]
**title** | **String** | Notification title | 
**r#type** | [**models::NotificationType**](NotificationType.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


