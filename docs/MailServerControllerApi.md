# \MailServerControllerApi

All URIs are relative to *https://api.mailslurp.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**describe_mail_server_domain**](MailServerControllerApi#describe_mail_server_domain) | **post** /mail-server/describe/domain | Get DNS Mail Server records for a domain
[**get_dns_lookup**](MailServerControllerApi#get_dns_lookup) | **post** /mail-server/describe/dns-lookup | Lookup DNS records for a domain
[**get_ip_address**](MailServerControllerApi#get_ip_address) | **post** /mail-server/describe/ip-address | Get IP address for a domain
[**verify_email_address**](MailServerControllerApi#verify_email_address) | **post** /mail-server/verify/email-address | Verify the existence of an email address at a given mail server.



## describe_mail_server_domain

> crate::models::DescribeMailServerDomainResult describe_mail_server_domain(describe_options)
Get DNS Mail Server records for a domain

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**describe_options** | [**DescribeDomainOptions**](DescribeDomainOptions) | describeOptions | [required] |

### Return type

[**crate::models::DescribeMailServerDomainResult**](DescribeMailServerDomainResult)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_dns_lookup

> crate::models::DnsLookupResults get_dns_lookup(dns_lookup_options)
Lookup DNS records for a domain

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_lookup_options** | [**DnsLookupOptions**](DnsLookupOptions) | dnsLookupOptions | [required] |

### Return type

[**crate::models::DnsLookupResults**](DNSLookupResults)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_ip_address

> crate::models::IpAddressResult get_ip_address(name)
Get IP address for a domain

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name | [required] |

### Return type

[**crate::models::IpAddressResult**](IPAddressResult)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## verify_email_address

> crate::models::EmailVerificationResult verify_email_address(verify_options)
Verify the existence of an email address at a given mail server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verify_options** | [**VerifyEmailAddressOptions**](VerifyEmailAddressOptions) | verifyOptions | [required] |

### Return type

[**crate::models::EmailVerificationResult**](EmailVerificationResult)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)

