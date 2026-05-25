# \FacesApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_face**](FacesApi.md#create_face) | **POST** /faces | Create a face
[**delete_face**](FacesApi.md#delete_face) | **DELETE** /faces/{id} | Delete a face
[**get_faces**](FacesApi.md#get_faces) | **GET** /faces | Retrieve faces for asset
[**reassign_faces_by_id**](FacesApi.md#reassign_faces_by_id) | **PUT** /faces/{id} | Re-assign a face to another person



## create_face

> create_face(asset_face_create_dto)
Create a face

Create a new face that has not been discovered by facial recognition. The content of the bounding box is considered a face.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_face_create_dto** | [**AssetFaceCreateDto**](AssetFaceCreateDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_face

> delete_face(id, asset_face_delete_dto)
Delete a face

Delete a face identified by the id. Optionally can be force deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**asset_face_delete_dto** | [**AssetFaceDeleteDto**](AssetFaceDeleteDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_faces

> Vec<models::AssetFaceResponseDto> get_faces(id)
Retrieve faces for asset

Retrieve all faces belonging to an asset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Face ID | [required] |

### Return type

[**Vec<models::AssetFaceResponseDto>**](AssetFaceResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reassign_faces_by_id

> models::PersonResponseDto reassign_faces_by_id(id, face_dto)
Re-assign a face to another person

Re-assign the face provided in the body to the person identified by the id in the path parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**face_dto** | [**FaceDto**](FaceDto.md) |  | [required] |

### Return type

[**models::PersonResponseDto**](PersonResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

