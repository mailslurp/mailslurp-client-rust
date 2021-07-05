# \DomainControllerApi

All URIs are relative to *https://api.mailslurp.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_domain_wildcard_catch_all**](DomainControllerApi#add_domain_wildcard_catch_all) | **post** /domains/{id}/wildcard | Add catch all wild card inbox to domain
[**create_domain**](DomainControllerApi#create_domain) | **post** /domains | Create Domain
[**delete_domain**](DomainControllerApi#delete_domain) | **delete** /domains/{id} | Delete a domain
[**get_domain**](DomainControllerApi#get_domain) | **get** /domains/{id} | Get a domain
[**get_domains**](DomainControllerApi#get_domains) | **get** /domains | Get domains
[**update_domain**](DomainControllerApi#update_domain) | **put** /domains/{id} | Update a domain



## add_domain_wildcard_catch_all

> crate::models::DomainDto add_domain_wildcard_catch_all(id)
Add catch all wild card inbox to domain

Add a catch all inbox to a domain so that any emails sent to it that cannot be matched will be sent to the catch all inbox generated

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**]() | id | [required] |

### Return type

[**crate::models::DomainDto**](DomainDto)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## create_domain

> crate::models::DomainDto create_domain(domain_options)
Create Domain

Link a domain that you own with MailSlurp so you can create email addresses using it. Endpoint returns DNS records used for validation. You must add these verification records to your host provider's DNS setup to verify the domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_options** | [**CreateDomainOptions**](CreateDomainOptions) | domainOptions | [required] |

### Return type

[**crate::models::DomainDto**](DomainDto)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## delete_domain

> Vec<String> delete_domain(id)
Delete a domain

Delete a domain. This will disable any existing inboxes that use this domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**]() | id | [required] |

### Return type

**Vec<String>**

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_domain

> crate::models::DomainDto get_domain(id)
Get a domain

Returns domain verification status and tokens for a given domain

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**]() | id | [required] |

### Return type

[**crate::models::DomainDto**](DomainDto)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_domains

> Vec<crate::models::DomainPreview> get_domains()
Get domains

List all custom domains you have created

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::DomainPreview>**](DomainPreview)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## update_domain

> crate::models::DomainDto update_domain(id, update_domain_dto)
Update a domain

Update values on a domain. Note you cannot change the domain name as it is immutable. Recreate the domain if you need to alter this.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**]() | id | [required] |
**update_domain_dto** | [**UpdateDomainOptions**](UpdateDomainOptions) | updateDomainDto | [required] |

### Return type

[**crate::models::DomainDto**](DomainDto)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)

