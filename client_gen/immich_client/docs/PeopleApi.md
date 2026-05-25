# \PeopleApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_person**](PeopleApi.md#create_person) | **POST** /people | Create a person
[**delete_people**](PeopleApi.md#delete_people) | **DELETE** /people | Delete people
[**delete_person**](PeopleApi.md#delete_person) | **DELETE** /people/{id} | Delete person
[**get_all_people**](PeopleApi.md#get_all_people) | **GET** /people | Get all people
[**get_person**](PeopleApi.md#get_person) | **GET** /people/{id} | Get a person
[**get_person_statistics**](PeopleApi.md#get_person_statistics) | **GET** /people/{id}/statistics | Get person statistics
[**get_person_thumbnail**](PeopleApi.md#get_person_thumbnail) | **GET** /people/{id}/thumbnail | Get person thumbnail
[**merge_person**](PeopleApi.md#merge_person) | **POST** /people/{id}/merge | Merge people
[**reassign_faces**](PeopleApi.md#reassign_faces) | **PUT** /people/{id}/reassign | Reassign faces
[**update_people**](PeopleApi.md#update_people) | **PUT** /people | Update people
[**update_person**](PeopleApi.md#update_person) | **PUT** /people/{id} | Update person



## create_person

> models::PersonResponseDto create_person(person_create_dto)
Create a person

Create a new person that can have multiple faces assigned to them.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**person_create_dto** | [**PersonCreateDto**](PersonCreateDto.md) |  | [required] |

### Return type

[**models::PersonResponseDto**](PersonResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_people

> delete_people(bulk_ids_dto)
Delete people

Bulk delete a list of people at once.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_ids_dto** | [**BulkIdsDto**](BulkIdsDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_person

> delete_person(id)
Delete person

Delete an individual person.

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


## get_all_people

> models::PeopleResponseDto get_all_people(closest_asset_id, closest_person_id, page, size, with_hidden)
Get all people

Retrieve a list of all people.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**closest_asset_id** | Option<**uuid::Uuid**> | Closest asset ID for similarity search |  |
**closest_person_id** | Option<**uuid::Uuid**> | Closest person ID for similarity search |  |
**page** | Option<**i32**> | Page number for pagination |  |[default to 1]
**size** | Option<**i32**> | Number of items per page |  |[default to 500]
**with_hidden** | Option<**bool**> | Include hidden people |  |

### Return type

[**models::PeopleResponseDto**](PeopleResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_person

> models::PersonResponseDto get_person(id)
Get a person

Retrieve a person by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::PersonResponseDto**](PersonResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_person_statistics

> models::PersonStatisticsResponseDto get_person_statistics(id)
Get person statistics

Retrieve statistics about a specific person.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::PersonStatisticsResponseDto**](PersonStatisticsResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_person_thumbnail

> std::path::PathBuf get_person_thumbnail(id)
Get person thumbnail

Retrieve the thumbnail file for a person.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## merge_person

> Vec<models::BulkIdResponseDto> merge_person(id, merge_person_dto)
Merge people

Merge a list of people into the person specified in the path parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**merge_person_dto** | [**MergePersonDto**](MergePersonDto.md) |  | [required] |

### Return type

[**Vec<models::BulkIdResponseDto>**](BulkIdResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reassign_faces

> Vec<models::PersonResponseDto> reassign_faces(id, asset_face_update_dto)
Reassign faces

Bulk reassign a list of faces to a different person.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**asset_face_update_dto** | [**AssetFaceUpdateDto**](AssetFaceUpdateDto.md) |  | [required] |

### Return type

[**Vec<models::PersonResponseDto>**](PersonResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_people

> Vec<models::BulkIdResponseDto> update_people(people_update_dto)
Update people

Bulk update multiple people at once.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**people_update_dto** | [**PeopleUpdateDto**](PeopleUpdateDto.md) |  | [required] |

### Return type

[**Vec<models::BulkIdResponseDto>**](BulkIdResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_person

> models::PersonResponseDto update_person(id, person_update_dto)
Update person

Update an individual person.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**person_update_dto** | [**PersonUpdateDto**](PersonUpdateDto.md) |  | [required] |

### Return type

[**models::PersonResponseDto**](PersonResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

