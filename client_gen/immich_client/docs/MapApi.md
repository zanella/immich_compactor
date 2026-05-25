# \MapApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_map_markers**](MapApi.md#get_map_markers) | **GET** /map/markers | Retrieve map markers
[**reverse_geocode**](MapApi.md#reverse_geocode) | **GET** /map/reverse-geocode | Reverse geocode coordinates



## get_map_markers

> Vec<models::MapMarkerResponseDto> get_map_markers(file_created_after, file_created_before, is_archived, is_favorite, with_partners, with_shared_albums)
Retrieve map markers

Retrieve a list of latitude and longitude coordinates for every asset with location data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_created_after** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Filter assets created after this date |  |
**file_created_before** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Filter assets created before this date |  |
**is_archived** | Option<**bool**> | Filter by archived status |  |
**is_favorite** | Option<**bool**> | Filter by favorite status |  |
**with_partners** | Option<**bool**> | Include partner assets |  |
**with_shared_albums** | Option<**bool**> | Include shared album assets |  |

### Return type

[**Vec<models::MapMarkerResponseDto>**](MapMarkerResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reverse_geocode

> Vec<models::MapReverseGeocodeResponseDto> reverse_geocode(lat, lon)
Reverse geocode coordinates

Retrieve location information (e.g., city, country) for given latitude and longitude coordinates.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lat** | **f64** | Latitude (-90 to 90) | [required] |
**lon** | **f64** | Longitude (-180 to 180) | [required] |

### Return type

[**Vec<models::MapReverseGeocodeResponseDto>**](MapReverseGeocodeResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

