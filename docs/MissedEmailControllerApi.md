# \MissedEmailControllerApi

All URIs are relative to *https://api.mailslurp.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_missed_emails**](MissedEmailControllerApi#get_all_missed_emails) | **get** /missed-emails | Get all MissedEmails in paginated format
[**get_missed_email**](MissedEmailControllerApi#get_missed_email) | **get** /missed-emails/{MissedEmailId} | Get MissedEmail



## get_all_missed_emails

> crate::models::PageMissedEmailProjection get_all_missed_emails(page, size, sort)
Get all MissedEmails in paginated format

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Optional page index in inbox list pagination |  |[default to 0]
**size** | Option<**i32**> | Optional page size in inbox list pagination |  |[default to 20]
**sort** | Option<**String**> | Optional createdAt sort direction ASC or DESC |  |[default to ASC]

### Return type

[**crate::models::PageMissedEmailProjection**](PageMissedEmailProjection)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_missed_email

> crate::models::MissedEmail get_missed_email(missed_email_id)
Get MissedEmail

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**missed_email_id** | [**String**]() | MissedEmailId | [required] |

### Return type

[**crate::models::MissedEmail**](MissedEmail)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)

