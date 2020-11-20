# \MailServerControllerApi

All URIs are relative to *https://api.mailslurp.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**describe_mail_server_domain**](MailServerControllerApi.md#describe_mail_server_domain) | **Post** /mail-server/describe/domain | Get DNS Mail Server records for a domain
[**verify_email_address**](MailServerControllerApi.md#verify_email_address) | **Post** /mail-server/verify/email-address | Verify the existence of an email address at a given mail server.



## describe_mail_server_domain

> crate::models::DescribeMailServerDomainResult describe_mail_server_domain(describe_options)
Get DNS Mail Server records for a domain

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**describe_options** | [**DescribeDomainOptions**](DescribeDomainOptions.md) | describeOptions | [required] |

### Return type

[**crate::models::DescribeMailServerDomainResult**](DescribeMailServerDomainResult.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_email_address

> crate::models::EmailVerificationResult verify_email_address(verify_options)
Verify the existence of an email address at a given mail server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verify_options** | [**VerifyEmailAddressOptions**](VerifyEmailAddressOptions.md) | verifyOptions | [required] |

### Return type

[**crate::models::EmailVerificationResult**](EmailVerificationResult.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

