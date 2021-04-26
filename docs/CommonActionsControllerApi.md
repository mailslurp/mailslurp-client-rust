# \CommonActionsControllerApi

All URIs are relative to *https://api.mailslurp.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_new_email_address**](CommonActionsControllerApi#create_new_email_address) | **post** /createInbox | Create new random inbox
[**create_new_email_address1**](CommonActionsControllerApi#create_new_email_address1) | **post** /newEmailAddress | Create new random inbox
[**empty_inbox**](CommonActionsControllerApi#empty_inbox) | **delete** /emptyInbox | Delete all emails in an inbox
[**send_email_simple**](CommonActionsControllerApi#send_email_simple) | **post** /sendEmail | Send an email



## create_new_email_address

> crate::models::Inbox create_new_email_address(allow_team_access, expires_at, expires_in, use_domain_pool)
Create new random inbox

Returns an Inbox with an `id` and an `emailAddress`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allow_team_access** | Option<**bool**> | allowTeamAccess |  |
**expires_at** | Option<**String**> | expiresAt |  |
**expires_in** | Option<**i64**> | expiresIn |  |
**use_domain_pool** | Option<**bool**> | useDomainPool |  |

### Return type

[**crate::models::Inbox**](Inbox)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## create_new_email_address1

> crate::models::Inbox create_new_email_address1(allow_team_access, expires_at, expires_in, use_domain_pool)
Create new random inbox

Returns an Inbox with an `id` and an `emailAddress`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allow_team_access** | Option<**bool**> | allowTeamAccess |  |
**expires_at** | Option<**String**> | expiresAt |  |
**expires_in** | Option<**i64**> | expiresIn |  |
**use_domain_pool** | Option<**bool**> | useDomainPool |  |

### Return type

[**crate::models::Inbox**](Inbox)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## empty_inbox

> empty_inbox(inbox_id)
Delete all emails in an inbox

Deletes all emails

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inbox_id** | [**String**]() | inboxId | [required] |

### Return type

 (empty response body)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## send_email_simple

> send_email_simple(email_options)
Send an email

If no senderId or inboxId provided a random email address will be used to send from.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_options** | [**SimpleSendEmailOptions**](SimpleSendEmailOptions) | emailOptions | [required] |

### Return type

 (empty response body)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)

