# \NotificationsApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_notification**](NotificationsApi.md#delete_notification) | **DELETE** /notifications/{id} | Delete a notification
[**delete_notifications**](NotificationsApi.md#delete_notifications) | **DELETE** /notifications | Delete notifications
[**get_notification**](NotificationsApi.md#get_notification) | **GET** /notifications/{id} | Get a notification
[**get_notifications**](NotificationsApi.md#get_notifications) | **GET** /notifications | Retrieve notifications
[**update_notification**](NotificationsApi.md#update_notification) | **PUT** /notifications/{id} | Update a notification
[**update_notifications**](NotificationsApi.md#update_notifications) | **PUT** /notifications | Update notifications



## delete_notification

> delete_notification(id)
Delete a notification

Delete a specific notification.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_notifications

> delete_notifications(notification_delete_all_dto)
Delete notifications

Delete a list of notifications at once.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification_delete_all_dto** | [**NotificationDeleteAllDto**](NotificationDeleteAllDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notification

> models::NotificationDto get_notification(id)
Get a notification

Retrieve a specific notification identified by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::NotificationDto**](NotificationDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notifications

> Vec<models::NotificationDto> get_notifications(id, level, r#type, unread)
Retrieve notifications

Retrieve a list of notifications.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**uuid::Uuid**> | Filter by notification ID |  |
**level** | Option<[**NotificationLevel**](NotificationLevel.md)> |  |  |
**r#type** | Option<[**NotificationType**](NotificationType.md)> |  |  |
**unread** | Option<**bool**> | Filter by unread status |  |

### Return type

[**Vec<models::NotificationDto>**](NotificationDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_notification

> models::NotificationDto update_notification(id, notification_update_dto)
Update a notification

Update a specific notification to set its read status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**notification_update_dto** | [**NotificationUpdateDto**](NotificationUpdateDto.md) |  | [required] |

### Return type

[**models::NotificationDto**](NotificationDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_notifications

> update_notifications(notification_update_all_dto)
Update notifications

Update a list of notifications. Allows to bulk-set the read status of notifications.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification_update_all_dto** | [**NotificationUpdateAllDto**](NotificationUpdateAllDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

