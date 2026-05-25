# \SearchApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_assets_by_city**](SearchApi.md#get_assets_by_city) | **GET** /search/cities | Retrieve assets by city
[**get_explore_data**](SearchApi.md#get_explore_data) | **GET** /search/explore | Retrieve explore data
[**get_search_suggestions**](SearchApi.md#get_search_suggestions) | **GET** /search/suggestions | Retrieve search suggestions
[**search_asset_statistics**](SearchApi.md#search_asset_statistics) | **POST** /search/statistics | Search asset statistics
[**search_assets**](SearchApi.md#search_assets) | **POST** /search/metadata | Search assets by metadata
[**search_large_assets**](SearchApi.md#search_large_assets) | **POST** /search/large-assets | Search large assets
[**search_person**](SearchApi.md#search_person) | **GET** /search/person | Search people
[**search_places**](SearchApi.md#search_places) | **GET** /search/places | Search places
[**search_random**](SearchApi.md#search_random) | **POST** /search/random | Search random assets
[**search_smart**](SearchApi.md#search_smart) | **POST** /search/smart | Smart asset search



## get_assets_by_city

> Vec<models::AssetResponseDto> get_assets_by_city()
Retrieve assets by city

Retrieve a list of assets with each asset belonging to a different city. This endpoint is used on the places pages to show a single thumbnail for each city the user has assets in.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::AssetResponseDto>**](AssetResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_explore_data

> Vec<models::SearchExploreResponseDto> get_explore_data()
Retrieve explore data

Retrieve data for the explore section, such as popular people and places.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::SearchExploreResponseDto>**](SearchExploreResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_suggestions

> Vec<String> get_search_suggestions(r#type, country, include_null, lens_model, make, model, state)
Retrieve search suggestions

Retrieve search suggestions based on partial input. This endpoint is used for typeahead search features.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | [**SearchSuggestionType**](SearchSuggestionType.md) |  | [required] |
**country** | Option<**String**> | Filter by country |  |
**include_null** | Option<**bool**> | Include null values in suggestions |  |
**lens_model** | Option<**String**> | Filter by lens model |  |
**make** | Option<**String**> | Filter by camera make |  |
**model** | Option<**String**> | Filter by camera model |  |
**state** | Option<**String**> | Filter by state/province |  |

### Return type

**Vec<String>**

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_asset_statistics

> models::SearchStatisticsResponseDto search_asset_statistics(statistics_search_dto)
Search asset statistics

Retrieve statistical data about assets based on search criteria, such as the total matching count.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**statistics_search_dto** | [**StatisticsSearchDto**](StatisticsSearchDto.md) |  | [required] |

### Return type

[**models::SearchStatisticsResponseDto**](SearchStatisticsResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_assets

> models::SearchResponseDto search_assets(metadata_search_dto)
Search assets by metadata

Search for assets based on various metadata criteria.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**metadata_search_dto** | [**MetadataSearchDto**](MetadataSearchDto.md) |  | [required] |

### Return type

[**models::SearchResponseDto**](SearchResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_large_assets

> Vec<models::AssetResponseDto> search_large_assets(album_ids, city, country, created_after, created_before, is_encoded, is_favorite, is_motion, is_not_in_album, is_offline, lens_model, library_id, make, min_file_size, model, ocr, person_ids, rating, size, state, tag_ids, taken_after, taken_before, trashed_after, trashed_before, r#type, updated_after, updated_before, visibility, with_deleted, with_exif)
Search large assets

Search for assets that are considered large based on specified criteria.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**album_ids** | Option<[**Vec<uuid::Uuid>**](Uuid__Uuid.md)> | Filter by album IDs |  |
**city** | Option<**String**> | Filter by city name |  |
**country** | Option<**String**> | Filter by country name |  |
**created_after** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Filter by creation date (after) |  |
**created_before** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Filter by creation date (before) |  |
**is_encoded** | Option<**bool**> | Filter by encoded status |  |
**is_favorite** | Option<**bool**> | Filter by favorite status |  |
**is_motion** | Option<**bool**> | Filter by motion photo status |  |
**is_not_in_album** | Option<**bool**> | Filter assets not in any album |  |
**is_offline** | Option<**bool**> | Filter by offline status |  |
**lens_model** | Option<**String**> | Filter by lens model |  |
**library_id** | Option<**uuid::Uuid**> | Library ID to filter by |  |
**make** | Option<**String**> | Filter by camera make |  |
**min_file_size** | Option<**i32**> | Minimum file size in bytes |  |
**model** | Option<**String**> | Filter by camera model |  |
**ocr** | Option<**String**> | Filter by OCR text content |  |
**person_ids** | Option<[**Vec<uuid::Uuid>**](Uuid__Uuid.md)> | Filter by person IDs |  |
**rating** | Option<**i32**> | Filter by rating [1-5], or null for unrated |  |
**size** | Option<**i32**> | Number of results to return |  |
**state** | Option<**String**> | Filter by state/province name |  |
**tag_ids** | Option<[**Vec<uuid::Uuid>**](Uuid__Uuid.md)> | Filter by tag IDs |  |
**taken_after** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Filter by taken date (after) |  |
**taken_before** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Filter by taken date (before) |  |
**trashed_after** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Filter by trash date (after) |  |
**trashed_before** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Filter by trash date (before) |  |
**r#type** | Option<[**AssetTypeEnum**](AssetTypeEnum.md)> |  |  |
**updated_after** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Filter by update date (after) |  |
**updated_before** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Filter by update date (before) |  |
**visibility** | Option<[**AssetVisibility**](AssetVisibility.md)> |  |  |
**with_deleted** | Option<**bool**> | Include deleted assets |  |
**with_exif** | Option<**bool**> | Include EXIF data in response |  |

### Return type

[**Vec<models::AssetResponseDto>**](AssetResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_person

> Vec<models::PersonResponseDto> search_person(name, with_hidden)
Search people

Search for people by name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Person name to search for | [required] |
**with_hidden** | Option<**bool**> | Include hidden people |  |

### Return type

[**Vec<models::PersonResponseDto>**](PersonResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_places

> Vec<models::PlacesResponseDto> search_places(name)
Search places

Search for places by name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Place name to search for | [required] |

### Return type

[**Vec<models::PlacesResponseDto>**](PlacesResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_random

> Vec<models::AssetResponseDto> search_random(random_search_dto)
Search random assets

Retrieve a random selection of assets based on the provided criteria.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**random_search_dto** | [**RandomSearchDto**](RandomSearchDto.md) |  | [required] |

### Return type

[**Vec<models::AssetResponseDto>**](AssetResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_smart

> models::SearchResponseDto search_smart(smart_search_dto)
Smart asset search

Perform a smart search for assets by using machine learning vectors to determine relevance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**smart_search_dto** | [**SmartSearchDto**](SmartSearchDto.md) |  | [required] |

### Return type

[**models::SearchResponseDto**](SearchResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

