# \MissedEmailControllerApi

All URIs are relative to *https://api.mailslurp.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_missed_emails**](MissedEmailControllerApi#get_all_missed_emails) | **get** /missed-emails | Get all MissedEmails in paginated format
[**get_all_unknown_missed_emails**](MissedEmailControllerApi#get_all_unknown_missed_emails) | **get** /missed-emails/unknown | Get all unknown missed emails in paginated format
[**get_missed_email**](MissedEmailControllerApi#get_missed_email) | **get** /missed-emails/{missedEmailId} | Get MissedEmail
[**wait_for_nth_missed_email**](MissedEmailControllerApi#wait_for_nth_missed_email) | **get** /missed-emails/waitForNthMissedEmail | Wait for Nth missed email



## get_all_missed_emails

> crate::models::PageMissedEmailProjection get_all_missed_emails(inbox_id, page, search_filter, size, sort)
Get all MissedEmails in paginated format

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inbox_id** | Option<[**String**]()> | Optional inbox ID filter |  |
**page** | Option<**i32**> | Optional page index in list pagination |  |[default to 0]
**search_filter** | Option<**String**> | Optional search filter |  |
**size** | Option<**i32**> | Optional page size in list pagination |  |[default to 20]
**sort** | Option<**String**> | Optional createdAt sort direction ASC or DESC |  |[default to ASC]

### Return type

[**crate::models::PageMissedEmailProjection**](PageMissedEmailProjection)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_all_unknown_missed_emails

> crate::models::PageUnknownMissedEmailProjection get_all_unknown_missed_emails(inbox_id, page, search_filter, size, sort)
Get all unknown missed emails in paginated format

Unknown missed emails are emails that were sent to MailSlurp but could not be assigned to an existing inbox.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inbox_id** | Option<[**String**]()> | Optional inbox ID filter |  |
**page** | Option<**i32**> | Optional page index in list pagination |  |[default to 0]
**search_filter** | Option<**String**> | Optional search filter |  |
**size** | Option<**i32**> | Optional page size in list pagination |  |[default to 20]
**sort** | Option<**String**> | Optional createdAt sort direction ASC or DESC |  |[default to ASC]

### Return type

[**crate::models::PageUnknownMissedEmailProjection**](PageUnknownMissedEmailProjection)

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
**missed_email_id** | [**String**]() | missedEmailId | [required] |

### Return type

[**crate::models::MissedEmail**](MissedEmail)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## wait_for_nth_missed_email

> crate::models::MissedEmail wait_for_nth_missed_email(inbox_id, index, timeout)
Wait for Nth missed email

Wait for 0 based index missed email

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inbox_id** | Option<[**String**]()> | Optional inbox ID filter |  |
**index** | Option<**i32**> | Zero based index of the email to wait for. If 1 missed email already and you want to wait for the 2nd email pass index=1 |  |
**timeout** | Option<**i64**> | Optional timeout milliseconds |  |

### Return type

[**crate::models::MissedEmail**](MissedEmail)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)

