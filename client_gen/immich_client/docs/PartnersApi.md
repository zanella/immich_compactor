# \PartnersApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_partner**](PartnersApi.md#create_partner) | **POST** /partners | Create a partner
[**create_partner_deprecated**](PartnersApi.md#create_partner_deprecated) | **POST** /partners/{id} | Create a partner
[**get_partners**](PartnersApi.md#get_partners) | **GET** /partners | Retrieve partners
[**remove_partner**](PartnersApi.md#remove_partner) | **DELETE** /partners/{id} | Remove a partner
[**update_partner**](PartnersApi.md#update_partner) | **PUT** /partners/{id} | Update a partner



## create_partner

> models::PartnerResponseDto create_partner(partner_create_dto)
Create a partner

Create a new partner to share assets with.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**partner_create_dto** | [**PartnerCreateDto**](PartnerCreateDto.md) |  | [required] |

### Return type

[**models::PartnerResponseDto**](PartnerResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_partner_deprecated

> models::PartnerResponseDto create_partner_deprecated(id)
Create a partner

Create a new partner to share assets with.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::PartnerResponseDto**](PartnerResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_partners

> Vec<models::PartnerResponseDto> get_partners(direction)
Retrieve partners

Retrieve a list of partners with whom assets are shared.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**direction** | [**PartnerDirection**](PartnerDirection.md) |  | [required] |

### Return type

[**Vec<models::PartnerResponseDto>**](PartnerResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_partner

> remove_partner(id)
Remove a partner

Stop sharing assets with a partner.

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


## update_partner

> models::PartnerResponseDto update_partner(id, partner_update_dto)
Update a partner

Specify whether a partner's assets should appear in the user's timeline.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**partner_update_dto** | [**PartnerUpdateDto**](PartnerUpdateDto.md) |  | [required] |

### Return type

[**models::PartnerResponseDto**](PartnerResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

