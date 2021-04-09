# \SentEmailsControllerApi

All URIs are relative to *https://api.mailslurp.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_sent_email**](SentEmailsControllerApi.md#get_sent_email) | **get** /sent/{id} | Get sent email receipt
[**get_sent_emails**](SentEmailsControllerApi.md#get_sent_emails) | **get** /sent | Get all sent emails in paginated form
[**get_sent_organization_emails**](SentEmailsControllerApi.md#get_sent_organization_emails) | **get** /sent/organization | Get all sent organization emails in paginated form



## get_sent_email

> crate::models::SentEmailDto get_sent_email(id)
Get sent email receipt

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**](.md) | id | [required] |

### Return type

[**crate::models::SentEmailDto**](SentEmailDto.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sent_emails

> crate::models::PageSentEmailProjection get_sent_emails(inbox_id, page, size, sort)
Get all sent emails in paginated form

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inbox_id** | Option<[**String**](.md)> | Optional inboxId to filter sender of sent emails by |  |
**page** | Option<**i32**> | Optional page index in inbox sent email list pagination |  |[default to 0]
**size** | Option<**i32**> | Optional page size in inbox sent email list pagination |  |[default to 20]
**sort** | Option<**String**> | Optional createdAt sort direction ASC or DESC |  |[default to ASC]

### Return type

[**crate::models::PageSentEmailProjection**](PageSentEmailProjection.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sent_organization_emails

> crate::models::PageSentEmailProjection get_sent_organization_emails(inbox_id, page, size, sort)
Get all sent organization emails in paginated form

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inbox_id** | Option<[**String**](.md)> | Optional inboxId to filter sender of sent emails by |  |
**page** | Option<**i32**> | Optional page index in inbox sent email list pagination |  |[default to 0]
**size** | Option<**i32**> | Optional page size in inbox sent email list pagination |  |[default to 20]
**sort** | Option<**String**> | Optional createdAt sort direction ASC or DESC |  |[default to ASC]

### Return type

[**crate::models::PageSentEmailProjection**](PageSentEmailProjection.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

