# \SessionsApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_session**](SessionsApi.md#create_session) | **POST** /sessions | Create a session
[**delete_all_sessions**](SessionsApi.md#delete_all_sessions) | **DELETE** /sessions | Delete all sessions
[**delete_session**](SessionsApi.md#delete_session) | **DELETE** /sessions/{id} | Delete a session
[**get_sessions**](SessionsApi.md#get_sessions) | **GET** /sessions | Retrieve sessions
[**lock_session**](SessionsApi.md#lock_session) | **POST** /sessions/{id}/lock | Lock a session
[**update_session**](SessionsApi.md#update_session) | **PUT** /sessions/{id} | Update a session



## create_session

> models::SessionCreateResponseDto create_session(session_create_dto)
Create a session

Create a session as a child to the current session. This endpoint is used for casting.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_create_dto** | [**SessionCreateDto**](SessionCreateDto.md) |  | [required] |

### Return type

[**models::SessionCreateResponseDto**](SessionCreateResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_all_sessions

> delete_all_sessions()
Delete all sessions

Delete all sessions for the user. This will not delete the current session.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_session

> delete_session(id)
Delete a session

Delete a specific session by id.

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


## get_sessions

> Vec<models::SessionResponseDto> get_sessions()
Retrieve sessions

Retrieve a list of sessions for the user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::SessionResponseDto>**](SessionResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lock_session

> lock_session(id)
Lock a session

Lock a specific session by id.

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


## update_session

> models::SessionResponseDto update_session(id, session_update_dto)
Update a session

Update a specific session identified by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**session_update_dto** | [**SessionUpdateDto**](SessionUpdateDto.md) |  | [required] |

### Return type

[**models::SessionResponseDto**](SessionResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

