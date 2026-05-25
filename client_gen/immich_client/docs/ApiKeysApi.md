# \ApiKeysApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_api_key**](ApiKeysApi.md#create_api_key) | **POST** /api-keys | Create an API key
[**delete_api_key**](ApiKeysApi.md#delete_api_key) | **DELETE** /api-keys/{id} | Delete an API key
[**get_api_key**](ApiKeysApi.md#get_api_key) | **GET** /api-keys/{id} | Retrieve an API key
[**get_api_keys**](ApiKeysApi.md#get_api_keys) | **GET** /api-keys | List all API keys
[**get_my_api_key**](ApiKeysApi.md#get_my_api_key) | **GET** /api-keys/me | Retrieve the current API key
[**update_api_key**](ApiKeysApi.md#update_api_key) | **PUT** /api-keys/{id} | Update an API key



## create_api_key

> models::ApiKeyCreateResponseDto create_api_key(api_key_create_dto)
Create an API key

Creates a new API key. It will be limited to the permissions specified.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key_create_dto** | [**ApiKeyCreateDto**](ApiKeyCreateDto.md) |  | [required] |

### Return type

[**models::ApiKeyCreateResponseDto**](ApiKeyCreateResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api_key

> delete_api_key(id)
Delete an API key

Deletes an API key identified by its ID. The current user must own this API key.

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


## get_api_key

> models::ApiKeyResponseDto get_api_key(id)
Retrieve an API key

Retrieve an API key by its ID. The current user must own this API key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::ApiKeyResponseDto**](ApiKeyResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_keys

> Vec<models::ApiKeyResponseDto> get_api_keys()
List all API keys

Retrieve all API keys of the current user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ApiKeyResponseDto>**](ApiKeyResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_api_key

> models::ApiKeyResponseDto get_my_api_key()
Retrieve the current API key

Retrieve the API key that is used to access this endpoint.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiKeyResponseDto**](ApiKeyResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_api_key

> models::ApiKeyResponseDto update_api_key(id, api_key_update_dto)
Update an API key

Updates the name and permissions of an API key by its ID. The current user must own this API key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**api_key_update_dto** | [**ApiKeyUpdateDto**](ApiKeyUpdateDto.md) |  | [required] |

### Return type

[**models::ApiKeyResponseDto**](ApiKeyResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

