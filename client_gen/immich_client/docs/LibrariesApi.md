# \LibrariesApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_library**](LibrariesApi.md#create_library) | **POST** /libraries | Create a library
[**delete_library**](LibrariesApi.md#delete_library) | **DELETE** /libraries/{id} | Delete a library
[**get_all_libraries**](LibrariesApi.md#get_all_libraries) | **GET** /libraries | Retrieve libraries
[**get_library**](LibrariesApi.md#get_library) | **GET** /libraries/{id} | Retrieve a library
[**get_library_statistics**](LibrariesApi.md#get_library_statistics) | **GET** /libraries/{id}/statistics | Retrieve library statistics
[**scan_library**](LibrariesApi.md#scan_library) | **POST** /libraries/{id}/scan | Scan a library
[**update_library**](LibrariesApi.md#update_library) | **PUT** /libraries/{id} | Update a library
[**validate**](LibrariesApi.md#validate) | **POST** /libraries/{id}/validate | Validate library settings



## create_library

> models::LibraryResponseDto create_library(create_library_dto)
Create a library

Create a new external library.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_library_dto** | [**CreateLibraryDto**](CreateLibraryDto.md) |  | [required] |

### Return type

[**models::LibraryResponseDto**](LibraryResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_library

> delete_library(id)
Delete a library

Delete an external library by its ID.

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


## get_all_libraries

> Vec<models::LibraryResponseDto> get_all_libraries()
Retrieve libraries

Retrieve a list of external libraries.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::LibraryResponseDto>**](LibraryResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_library

> models::LibraryResponseDto get_library(id)
Retrieve a library

Retrieve an external library by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::LibraryResponseDto**](LibraryResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_library_statistics

> models::LibraryStatsResponseDto get_library_statistics(id)
Retrieve library statistics

Retrieve statistics for a specific external library, including number of videos, images, and storage usage.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::LibraryStatsResponseDto**](LibraryStatsResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scan_library

> scan_library(id)
Scan a library

Queue a scan for the external library to find and import new assets.

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


## update_library

> models::LibraryResponseDto update_library(id, update_library_dto)
Update a library

Update an existing external library.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**update_library_dto** | [**UpdateLibraryDto**](UpdateLibraryDto.md) |  | [required] |

### Return type

[**models::LibraryResponseDto**](LibraryResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate

> models::ValidateLibraryResponseDto validate(id, validate_library_dto)
Validate library settings

Validate the settings of an external library.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**validate_library_dto** | [**ValidateLibraryDto**](ValidateLibraryDto.md) |  | [required] |

### Return type

[**models::ValidateLibraryResponseDto**](ValidateLibraryResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

