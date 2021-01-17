# \DomainControllerApi

All URIs are relative to *https://api.mailslurp.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_domain**](DomainControllerApi.md#create_domain) | **Post** /domains | Create Domain
[**delete_domain**](DomainControllerApi.md#delete_domain) | **Delete** /domains/{id} | Delete a domain
[**get_domain**](DomainControllerApi.md#get_domain) | **Get** /domains/{id} | Get a domain
[**get_domains**](DomainControllerApi.md#get_domains) | **Get** /domains | Get domains



## create_domain

> crate::models::DomainDto create_domain(domain_options)
Create Domain

Link a domain that you own with MailSlurp so you can create email addresses using it. Endpoint returns DNS records used for validation. You must add these verification records to your host provider's DNS setup to verify the domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_options** | [**CreateDomainOptions**](CreateDomainOptions.md) | domainOptions | [required] |

### Return type

[**crate::models::DomainDto**](DomainDto.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_domain

> Vec<String> delete_domain(id)
Delete a domain

Delete a domain. This will disable any existing inboxes that use this domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | id | [required] |

### Return type

**Vec<String>**

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_domain

> crate::models::DomainDto get_domain(id)
Get a domain

Returns domain verification status and tokens for a given domain

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | id | [required] |

### Return type

[**crate::models::DomainDto**](DomainDto.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_domains

> Vec<crate::models::DomainPreview> get_domains()
Get domains

List all custom domains you have created

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::DomainPreview>**](DomainPreview.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

