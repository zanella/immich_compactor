# \DeprecatedApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_partner_deprecated**](DeprecatedApi.md#create_partner_deprecated) | **POST** /partners/{id} | Create a partner
[**get_queues_legacy**](DeprecatedApi.md#get_queues_legacy) | **GET** /jobs | Retrieve queue counts and status
[**run_queue_command_legacy**](DeprecatedApi.md#run_queue_command_legacy) | **PUT** /jobs/{name} | Run jobs



## create_partner_deprecated

> models::PartnerResponseDto create_partner_deprecated(id)
Create a partner

Create a new partner to share assets with.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::PartnerResponseDto**](PartnerResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_queues_legacy

> models::QueuesResponseLegacyDto get_queues_legacy()
Retrieve queue counts and status

Retrieve the counts of the current queue, as well as the current status.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::QueuesResponseLegacyDto**](QueuesResponseLegacyDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## run_queue_command_legacy

> models::QueueResponseLegacyDto run_queue_command_legacy(name, queue_command_dto)
Run jobs

Queue all assets for a specific job type. Defaults to only queueing assets that have not yet been processed, but the force command can be used to re-process all assets.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | [**QueueName**](QueueName.md) |  | [required] |
**queue_command_dto** | [**QueueCommandDto**](QueueCommandDto.md) |  | [required] |

### Return type

[**models::QueueResponseLegacyDto**](QueueResponseLegacyDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

