# \JobsApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_job**](JobsApi.md#create_job) | **POST** /jobs | Create a manual job
[**get_queues_legacy**](JobsApi.md#get_queues_legacy) | **GET** /jobs | Retrieve queue counts and status
[**run_queue_command_legacy**](JobsApi.md#run_queue_command_legacy) | **PUT** /jobs/{name} | Run jobs



## create_job

> create_job(job_create_dto)
Create a manual job

Run a specific job. Most jobs are queued automatically, but this endpoint allows for manual creation of a handful of jobs, including various cleanup tasks, as well as creating a new database backup.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_create_dto** | [**JobCreateDto**](JobCreateDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

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

