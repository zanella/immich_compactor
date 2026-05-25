# \AlbumsApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_assets_to_album**](AlbumsApi.md#add_assets_to_album) | **PUT** /albums/{id}/assets | Add assets to an album
[**add_assets_to_albums**](AlbumsApi.md#add_assets_to_albums) | **PUT** /albums/assets | Add assets to albums
[**add_users_to_album**](AlbumsApi.md#add_users_to_album) | **PUT** /albums/{id}/users | Share album with users
[**create_album**](AlbumsApi.md#create_album) | **POST** /albums | Create an album
[**delete_album**](AlbumsApi.md#delete_album) | **DELETE** /albums/{id} | Delete an album
[**get_album_info**](AlbumsApi.md#get_album_info) | **GET** /albums/{id} | Retrieve an album
[**get_album_map_markers**](AlbumsApi.md#get_album_map_markers) | **GET** /albums/{id}/map-markers | Retrieve album map markers
[**get_album_statistics**](AlbumsApi.md#get_album_statistics) | **GET** /albums/statistics | Retrieve album statistics
[**get_all_albums**](AlbumsApi.md#get_all_albums) | **GET** /albums | List all albums
[**remove_asset_from_album**](AlbumsApi.md#remove_asset_from_album) | **DELETE** /albums/{id}/assets | Remove assets from an album
[**remove_user_from_album**](AlbumsApi.md#remove_user_from_album) | **DELETE** /albums/{id}/user/{userId} | Remove user from album
[**update_album_info**](AlbumsApi.md#update_album_info) | **PATCH** /albums/{id} | Update an album
[**update_album_user**](AlbumsApi.md#update_album_user) | **PUT** /albums/{id}/user/{userId} | Update user role



## add_assets_to_album

> Vec<models::BulkIdResponseDto> add_assets_to_album(id, bulk_ids_dto)
Add assets to an album

Add multiple assets to a specific album by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**bulk_ids_dto** | [**BulkIdsDto**](BulkIdsDto.md) |  | [required] |

### Return type

[**Vec<models::BulkIdResponseDto>**](BulkIdResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_assets_to_albums

> models::AlbumsAddAssetsResponseDto add_assets_to_albums(albums_add_assets_dto)
Add assets to albums

Send a list of asset IDs and album IDs to add each asset to each album.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**albums_add_assets_dto** | [**AlbumsAddAssetsDto**](AlbumsAddAssetsDto.md) |  | [required] |

### Return type

[**models::AlbumsAddAssetsResponseDto**](AlbumsAddAssetsResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_users_to_album

> models::AlbumResponseDto add_users_to_album(id, add_users_dto)
Share album with users

Share an album with multiple users. Each user can be given a specific role in the album.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**add_users_dto** | [**AddUsersDto**](AddUsersDto.md) |  | [required] |

### Return type

[**models::AlbumResponseDto**](AlbumResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_album

> models::AlbumResponseDto create_album(create_album_dto)
Create an album

Create a new album. The album can also be created with initial users and assets.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_album_dto** | [**CreateAlbumDto**](CreateAlbumDto.md) |  | [required] |

### Return type

[**models::AlbumResponseDto**](AlbumResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_album

> delete_album(id)
Delete an album

Delete a specific album by its ID. Note the album is initially trashed and then immediately scheduled for deletion, but relies on a background job to complete the process.

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


## get_album_info

> models::AlbumResponseDto get_album_info(id, key, slug)
Retrieve an album

Retrieve information about a specific album by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**key** | Option<**String**> |  |  |
**slug** | Option<**String**> |  |  |

### Return type

[**models::AlbumResponseDto**](AlbumResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_album_map_markers

> Vec<models::MapMarkerResponseDto> get_album_map_markers(id, key, slug)
Retrieve album map markers

Retrieve map marker information for a specific album by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**key** | Option<**String**> |  |  |
**slug** | Option<**String**> |  |  |

### Return type

[**Vec<models::MapMarkerResponseDto>**](MapMarkerResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_album_statistics

> models::AlbumStatisticsResponseDto get_album_statistics()
Retrieve album statistics

Returns statistics about the albums available to the authenticated user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AlbumStatisticsResponseDto**](AlbumStatisticsResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_albums

> Vec<models::AlbumResponseDto> get_all_albums(asset_id, is_owned, is_shared)
List all albums

Retrieve a list of albums available to the authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_id** | Option<**uuid::Uuid**> | Filter albums containing this asset ID (ignores other parameters) |  |
**is_owned** | Option<**bool**> | Filter by ownership: true = only owned, false = only shared-with-me, undefined = no filter |  |
**is_shared** | Option<**bool**> | Filter by shared status: true = only shared, false = not shared, undefined = no filter |  |

### Return type

[**Vec<models::AlbumResponseDto>**](AlbumResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_asset_from_album

> Vec<models::BulkIdResponseDto> remove_asset_from_album(id, bulk_ids_dto)
Remove assets from an album

Remove multiple assets from a specific album by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**bulk_ids_dto** | [**BulkIdsDto**](BulkIdsDto.md) |  | [required] |

### Return type

[**Vec<models::BulkIdResponseDto>**](BulkIdResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_user_from_album

> remove_user_from_album(id, user_id)
Remove user from album

Remove a user from an album. Use an ID of \"me\" to leave a shared album.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**user_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_album_info

> models::AlbumResponseDto update_album_info(id, update_album_dto)
Update an album

Update the information of a specific album by its ID. This endpoint can be used to update the album name, description, sort order, etc. However, it is not used to add or remove assets or users from the album.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**update_album_dto** | [**UpdateAlbumDto**](UpdateAlbumDto.md) |  | [required] |

### Return type

[**models::AlbumResponseDto**](AlbumResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_album_user

> update_album_user(id, user_id, update_album_user_dto)
Update user role

Change the role for a specific user in a specific album.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**user_id** | **String** |  | [required] |
**update_album_user_dto** | [**UpdateAlbumUserDto**](UpdateAlbumUserDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

