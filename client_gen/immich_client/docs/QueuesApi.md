# \QueuesApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**empty_queue**](QueuesApi.md#empty_queue) | **DELETE** /queues/{name}/jobs | Empty a queue
[**get_queue**](QueuesApi.md#get_queue) | **GET** /queues/{name} | Retrieve a queue
[**get_queue_jobs**](QueuesApi.md#get_queue_jobs) | **GET** /queues/{name}/jobs | Retrieve queue jobs
[**get_queues**](QueuesApi.md#get_queues) | **GET** /queues | List all queues
[**update_queue**](QueuesApi.md#update_queue) | **PUT** /queues/{name} | Update a queue



## empty_queue

> empty_queue(name, queue_delete_dto)
Empty a queue

Removes all jobs from the specified queue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | [**QueueName**](QueueName.md) |  | [required] |
**queue_delete_dto** | [**QueueDeleteDto**](QueueDeleteDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_queue

> models::QueueResponseDto get_queue(name)
Retrieve a queue

Retrieves a specific queue by its name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | [**QueueName**](QueueName.md) |  | [required] |

### Return type

[**models::QueueResponseDto**](QueueResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_queue_jobs

> Vec<models::QueueJobResponseDto> get_queue_jobs(name, status)
Retrieve queue jobs

Retrieves a list of queue jobs from the specified queue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | [**QueueName**](QueueName.md) |  | [required] |
**status** | Option<[**Vec<models::QueueJobStatus>**](Models__QueueJobStatus.md)> | Filter jobs by status |  |

### Return type

[**Vec<models::QueueJobResponseDto>**](QueueJobResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_queues

> Vec<models::QueueResponseDto> get_queues()
List all queues

Retrieves a list of queues.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::QueueResponseDto>**](QueueResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_queue

> models::QueueResponseDto update_queue(name, queue_update_dto)
Update a queue

Change the paused status of a specific queue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | [**QueueName**](QueueName.md) |  | [required] |
**queue_update_dto** | [**QueueUpdateDto**](QueueUpdateDto.md) |  | [required] |

### Return type

[**models::QueueResponseDto**](QueueResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

