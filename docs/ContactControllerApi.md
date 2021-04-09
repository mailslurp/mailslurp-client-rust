# \ContactControllerApi

All URIs are relative to *https://api.mailslurp.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_contact**](ContactControllerApi.md#create_contact) | **post** /contacts | Create a contact
[**delete_contact**](ContactControllerApi.md#delete_contact) | **delete** /contacts/{contactId} | Delete contact
[**get_all_contacts**](ContactControllerApi.md#get_all_contacts) | **get** /contacts/paginated | Get all contacts
[**get_contact**](ContactControllerApi.md#get_contact) | **get** /contacts/{contactId} | Get contact
[**get_contacts**](ContactControllerApi.md#get_contacts) | **get** /contacts | Get all contacts



## create_contact

> crate::models::ContactDto create_contact(create_contact_options)
Create a contact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_contact_options** | [**CreateContactOptions**](CreateContactOptions.md) | createContactOptions | [required] |

### Return type

[**crate::models::ContactDto**](ContactDto.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_contact

> delete_contact(contact_id)
Delete contact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contact_id** | [**String**](.md) | contactId | [required] |

### Return type

 (empty response body)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_contacts

> crate::models::PageContactProjection get_all_contacts(page, size, sort)
Get all contacts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Optional page index in inbox list pagination |  |[default to 0]
**size** | Option<**i32**> | Optional page size in inbox list pagination |  |[default to 20]
**sort** | Option<**String**> | Optional createdAt sort direction ASC or DESC |  |[default to ASC]

### Return type

[**crate::models::PageContactProjection**](PageContactProjection.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contact

> crate::models::ContactDto get_contact(contact_id)
Get contact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contact_id** | [**String**](.md) | contactId | [required] |

### Return type

[**crate::models::ContactDto**](ContactDto.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contacts

> Vec<crate::models::ContactProjection> get_contacts()
Get all contacts

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::ContactProjection>**](ContactProjection.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

