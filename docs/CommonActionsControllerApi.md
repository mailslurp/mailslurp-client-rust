# \CommonActionsControllerApi

All URIs are relative to *https://api.mailslurp.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_new_email_address**](CommonActionsControllerApi.md#create_new_email_address) | **Post** /createInbox | Create new random inbox
[**create_new_email_address1**](CommonActionsControllerApi.md#create_new_email_address1) | **Post** /newEmailAddress | Create new random inbox
[**empty_inbox**](CommonActionsControllerApi.md#empty_inbox) | **Delete** /emptyInbox | Delete all emails in an inbox
[**send_email_simple**](CommonActionsControllerApi.md#send_email_simple) | **Post** /sendEmail | Send an email



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

[**crate::models::Inbox**](Inbox.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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

[**crate::models::Inbox**](Inbox.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## empty_inbox

> empty_inbox(inbox_id)
Delete all emails in an inbox

Deletes all emails

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inbox_id** | [**String**](.md) | inboxId | [required] |

### Return type

 (empty response body)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_email_simple

> send_email_simple(email_options)
Send an email

If no senderId or inboxId provided a random email address will be used to send from.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_options** | [**SimpleSendEmailOptions**](SimpleSendEmailOptions.md) | emailOptions | [required] |

### Return type

 (empty response body)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

