# \SyncApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_sync_ack**](SyncApi.md#delete_sync_ack) | **DELETE** /sync/ack | Delete acknowledgements
[**get_sync_ack**](SyncApi.md#get_sync_ack) | **GET** /sync/ack | Retrieve acknowledgements
[**get_sync_stream**](SyncApi.md#get_sync_stream) | **POST** /sync/stream | Stream sync changes
[**send_sync_ack**](SyncApi.md#send_sync_ack) | **POST** /sync/ack | Acknowledge changes



## delete_sync_ack

> delete_sync_ack(sync_ack_delete_dto)
Delete acknowledgements

Delete specific synchronization acknowledgments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sync_ack_delete_dto** | [**SyncAckDeleteDto**](SyncAckDeleteDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sync_ack

> Vec<models::SyncAckDto> get_sync_ack()
Retrieve acknowledgements

Retrieve the synchronization acknowledgments for the current session.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::SyncAckDto>**](SyncAckDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sync_stream

> get_sync_stream(sync_stream_dto)
Stream sync changes

Retrieve a JSON lines streamed response of changes for synchronization. This endpoint is used by the mobile app to efficiently stay up to date with changes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sync_stream_dto** | [**SyncStreamDto**](SyncStreamDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_sync_ack

> send_sync_ack(sync_ack_set_dto)
Acknowledge changes

Send a list of synchronization acknowledgements to confirm that the latest changes have been received.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sync_ack_set_dto** | [**SyncAckSetDto**](SyncAckSetDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

