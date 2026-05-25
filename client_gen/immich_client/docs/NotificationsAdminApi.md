# \NotificationsAdminApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_notification**](NotificationsAdminApi.md#create_notification) | **POST** /admin/notifications | Create a notification
[**get_notification_template_admin**](NotificationsAdminApi.md#get_notification_template_admin) | **POST** /admin/notifications/templates/{name} | Render email template
[**send_test_email_admin**](NotificationsAdminApi.md#send_test_email_admin) | **POST** /admin/notifications/test-email | Send test email



## create_notification

> models::NotificationDto create_notification(notification_create_dto)
Create a notification

Create a new notification for a specific user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification_create_dto** | [**NotificationCreateDto**](NotificationCreateDto.md) |  | [required] |

### Return type

[**models::NotificationDto**](NotificationDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notification_template_admin

> models::TemplateResponseDto get_notification_template_admin(name, template_dto)
Render email template

Retrieve a preview of the provided email template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**template_dto** | [**TemplateDto**](TemplateDto.md) |  | [required] |

### Return type

[**models::TemplateResponseDto**](TemplateResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_test_email_admin

> models::TestEmailResponseDto send_test_email_admin(system_config_smtp_dto)
Send test email

Send a test email using the provided SMTP configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_config_smtp_dto** | [**SystemConfigSmtpDto**](SystemConfigSmtpDto.md) |  | [required] |

### Return type

[**models::TestEmailResponseDto**](TestEmailResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

