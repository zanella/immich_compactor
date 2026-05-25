# \SharedLinksApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_shared_link_assets**](SharedLinksApi.md#add_shared_link_assets) | **PUT** /shared-links/{id}/assets | Add assets to a shared link
[**create_shared_link**](SharedLinksApi.md#create_shared_link) | **POST** /shared-links | Create a shared link
[**get_all_shared_links**](SharedLinksApi.md#get_all_shared_links) | **GET** /shared-links | Retrieve all shared links
[**get_my_shared_link**](SharedLinksApi.md#get_my_shared_link) | **GET** /shared-links/me | Retrieve current shared link
[**get_shared_link_by_id**](SharedLinksApi.md#get_shared_link_by_id) | **GET** /shared-links/{id} | Retrieve a shared link
[**remove_shared_link**](SharedLinksApi.md#remove_shared_link) | **DELETE** /shared-links/{id} | Delete a shared link
[**remove_shared_link_assets**](SharedLinksApi.md#remove_shared_link_assets) | **DELETE** /shared-links/{id}/assets | Remove assets from a shared link
[**shared_link_login**](SharedLinksApi.md#shared_link_login) | **POST** /shared-links/login | Shared link login
[**update_shared_link**](SharedLinksApi.md#update_shared_link) | **PATCH** /shared-links/{id} | Update a shared link



## add_shared_link_assets

> Vec<models::AssetIdsResponseDto> add_shared_link_assets(id, asset_ids_dto)
Add assets to a shared link

Add assets to a specific shared link by its ID. This endpoint is only relevant for shared link of type individual.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**asset_ids_dto** | [**AssetIdsDto**](AssetIdsDto.md) |  | [required] |

### Return type

[**Vec<models::AssetIdsResponseDto>**](AssetIdsResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_shared_link

> models::SharedLinkResponseDto create_shared_link(shared_link_create_dto)
Create a shared link

Create a new shared link.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shared_link_create_dto** | [**SharedLinkCreateDto**](SharedLinkCreateDto.md) |  | [required] |

### Return type

[**models::SharedLinkResponseDto**](SharedLinkResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_shared_links

> Vec<models::SharedLinkResponseDto> get_all_shared_links(album_id, id)
Retrieve all shared links

Retrieve a list of all shared links.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**album_id** | Option<**uuid::Uuid**> | Filter by album ID |  |
**id** | Option<**uuid::Uuid**> | Filter by shared link ID |  |

### Return type

[**Vec<models::SharedLinkResponseDto>**](SharedLinkResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_shared_link

> models::SharedLinkResponseDto get_my_shared_link(key, slug)
Retrieve current shared link

Retrieve the current shared link associated with authentication method.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | Option<**String**> |  |  |
**slug** | Option<**String**> |  |  |

### Return type

[**models::SharedLinkResponseDto**](SharedLinkResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_shared_link_by_id

> models::SharedLinkResponseDto get_shared_link_by_id(id)
Retrieve a shared link

Retrieve a specific shared link by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::SharedLinkResponseDto**](SharedLinkResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_shared_link

> remove_shared_link(id)
Delete a shared link

Delete a specific shared link by its ID.

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


## remove_shared_link_assets

> Vec<models::AssetIdsResponseDto> remove_shared_link_assets(id, asset_ids_dto)
Remove assets from a shared link

Remove assets from a specific shared link by its ID. This endpoint is only relevant for shared link of type individual.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**asset_ids_dto** | [**AssetIdsDto**](AssetIdsDto.md) |  | [required] |

### Return type

[**Vec<models::AssetIdsResponseDto>**](AssetIdsResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## shared_link_login

> models::SharedLinkResponseDto shared_link_login(shared_link_login_dto, key, slug)
Shared link login

Login to a password protected shared link

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shared_link_login_dto** | [**SharedLinkLoginDto**](SharedLinkLoginDto.md) |  | [required] |
**key** | Option<**String**> |  |  |
**slug** | Option<**String**> |  |  |

### Return type

[**models::SharedLinkResponseDto**](SharedLinkResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_shared_link

> models::SharedLinkResponseDto update_shared_link(id, shared_link_edit_dto)
Update a shared link

Update an existing shared link by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**shared_link_edit_dto** | [**SharedLinkEditDto**](SharedLinkEditDto.md) |  | [required] |

### Return type

[**models::SharedLinkResponseDto**](SharedLinkResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

