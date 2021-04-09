# \DomainControllerApi

All URIs are relative to *https://api.mailslurp.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_domain_wildcard_catch_all**](DomainControllerApi.md#add_domain_wildcard_catch_all) | **post** /domains/{id}/wildcard | Add catch all wild card inbox to domain
[**create_domain**](DomainControllerApi.md#create_domain) | **post** /domains | Create Domain
[**delete_domain**](DomainControllerApi.md#delete_domain) | **delete** /domains/{id} | Delete a domain
[**get_domain**](DomainControllerApi.md#get_domain) | **get** /domains/{id} | Get a domain
[**get_domains**](DomainControllerApi.md#get_domains) | **get** /domains | Get domains
[**update_domain**](DomainControllerApi.md#update_domain) | **put** /domains/{id} | Update a domain



## add_domain_wildcard_catch_all

> crate::models::DomainDto add_domain_wildcard_catch_all(id)
Add catch all wild card inbox to domain

Add a catch all inbox to a domain so that any emails sent to it that cannot be matched will be sent to the catch all inbox generated

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


## update_domain

> crate::models::DomainDto update_domain(id, update_domain_dto)
Update a domain

Update values on a domain. Note you cannot change the domain name as it is immutable. Recreate the domain if you need to alter this.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | id | [required] |
**update_domain_dto** | [**UpdateDomainOptions**](UpdateDomainOptions.md) | updateDomainDto | [required] |

### Return type

[**crate::models::DomainDto**](DomainDto.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

