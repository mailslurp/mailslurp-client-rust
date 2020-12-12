# \AliasControllerApi

All URIs are relative to *https://api.mailslurp.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_alias**](AliasControllerApi.md#create_alias) | **Post** /aliases | Create an email alias. Must be verified by clicking link inside verification email that will be sent to the address. Once verified the alias will be active.
[**delete_alias**](AliasControllerApi.md#delete_alias) | **Delete** /aliases/{aliasId} | Delete an email alias
[**get_alias**](AliasControllerApi.md#get_alias) | **Get** /aliases/{aliasId} | Get an email alias
[**get_aliases**](AliasControllerApi.md#get_aliases) | **Get** /aliases | Get all email aliases you have created
[**update_alias**](AliasControllerApi.md#update_alias) | **Put** /aliases/{aliasId} | Update an email alias



## create_alias

> crate::models::Alias create_alias(create_alias_options)
Create an email alias. Must be verified by clicking link inside verification email that will be sent to the address. Once verified the alias will be active.

Email aliases use a MailSlurp randomly generated email address (or a custom domain inbox that you provide) to mask or proxy a real email address. Emails sent to the alias address will be forwarded to the hidden email address it was created for. If you want to send a reply use the threadId attached

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_alias_options** | [**CreateAliasOptions**](CreateAliasOptions.md) | createAliasOptions | [required] |

### Return type

[**crate::models::Alias**](Alias.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_alias

> delete_alias(alias_id)
Delete an email alias

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alias_id** | [**String**](.md) | aliasId | [required] |

### Return type

 (empty response body)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_alias

> crate::models::AliasDto get_alias(alias_id)
Get an email alias

Get an email alias by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alias_id** | [**String**](.md) | aliasId | [required] |

### Return type

[**crate::models::AliasDto**](AliasDto.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_aliases

> crate::models::PageAlias get_aliases(page, size, sort)
Get all email aliases you have created

Get all email aliases in paginated form

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Optional page index in alias list pagination |  |[default to 0]
**size** | Option<**i32**> | Optional page size in alias list pagination |  |[default to 20]
**sort** | Option<**String**> | Optional createdAt sort direction ASC or DESC |  |[default to ASC]

### Return type

[**crate::models::PageAlias**](PageAlias.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_alias

> update_alias(alias_id, update_alias_options)
Update an email alias

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alias_id** | [**String**](.md) | aliasId | [required] |
**update_alias_options** | [**UpdateAliasOptions**](UpdateAliasOptions.md) | updateAliasOptions | [required] |

### Return type

 (empty response body)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

