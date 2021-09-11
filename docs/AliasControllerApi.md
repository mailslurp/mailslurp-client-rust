# \AliasControllerApi

All URIs are relative to *https://api.mailslurp.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_alias**](AliasControllerApi#create_alias) | **post** /aliases | Create an email alias. Must be verified by clicking link inside verification email that will be sent to the address. Once verified the alias will be active.
[**delete_alias**](AliasControllerApi#delete_alias) | **delete** /aliases/{aliasId} | Delete an email alias
[**get_alias**](AliasControllerApi#get_alias) | **get** /aliases/{aliasId} | Get an email alias
[**get_alias_emails**](AliasControllerApi#get_alias_emails) | **get** /aliases/{aliasId}/emails | Get emails for an alias
[**get_alias_threads**](AliasControllerApi#get_alias_threads) | **get** /aliases/{aliasId}/threads | Get threads created for an alias
[**get_aliases**](AliasControllerApi#get_aliases) | **get** /aliases | Get all email aliases you have created
[**reply_to_alias_email**](AliasControllerApi#reply_to_alias_email) | **put** /aliases/{aliasId}/emails/{emailId} | Reply to an email
[**send_alias_email**](AliasControllerApi#send_alias_email) | **post** /aliases/{aliasId}/emails | Send an email from an alias inbox
[**update_alias**](AliasControllerApi#update_alias) | **put** /aliases/{aliasId} | Update an email alias



## create_alias

> crate::models::AliasDto create_alias(create_alias_options)
Create an email alias. Must be verified by clicking link inside verification email that will be sent to the address. Once verified the alias will be active.

Email aliases use a MailSlurp randomly generated email address (or a custom domain inbox that you provide) to mask or proxy a real email address. Emails sent to the alias address will be forwarded to the hidden email address it was created for. If you want to send a reply use the threadId attached

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_alias_options** | [**CreateAliasOptions**](CreateAliasOptions) | createAliasOptions | [required] |

### Return type

[**crate::models::AliasDto**](AliasDto)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## delete_alias

> delete_alias(alias_id)
Delete an email alias

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alias_id** | [**String**]() | aliasId | [required] |

### Return type

 (empty response body)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_alias

> crate::models::AliasDto get_alias(alias_id)
Get an email alias

Get an email alias by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alias_id** | [**String**]() | aliasId | [required] |

### Return type

[**crate::models::AliasDto**](AliasDto)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_alias_emails

> crate::models::PageEmailProjection get_alias_emails(alias_id, before, page, since, size, sort)
Get emails for an alias

Get paginated emails for an alias by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alias_id** | [**String**]() | aliasId | [required] |
**before** | Option<**String**> | Optional filter by sent before given date time |  |
**page** | Option<**i32**> | Optional page index alias email list pagination |  |[default to 0]
**since** | Option<**String**> | Optional filter by sent after given date time |  |
**size** | Option<**i32**> | Optional page size alias email list pagination |  |[default to 20]
**sort** | Option<**String**> | Optional createdAt sort direction ASC or DESC |  |[default to ASC]

### Return type

[**crate::models::PageEmailProjection**](PageEmailProjection)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_alias_threads

> crate::models::PageThreadProjection get_alias_threads(alias_id, before, page, since, size, sort)
Get threads created for an alias

Returns threads created for an email alias in paginated form

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alias_id** | [**String**]() | aliasId | [required] |
**before** | Option<**String**> | Optional filter by sent before given date time |  |
**page** | Option<**i32**> | Optional page index in thread list pagination |  |[default to 0]
**since** | Option<**String**> | Optional filter by sent after given date time |  |
**size** | Option<**i32**> | Optional page size in thread list pagination |  |[default to 20]
**sort** | Option<**String**> | Optional createdAt sort direction ASC or DESC |  |[default to ASC]

### Return type

[**crate::models::PageThreadProjection**](PageThreadProjection)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_aliases

> crate::models::PageAlias get_aliases(before, page, since, size, sort)
Get all email aliases you have created

Get all email aliases in paginated form

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**before** | Option<**String**> | Filter by created at before the given timestamp |  |
**page** | Option<**i32**> | Optional page index in alias list pagination |  |[default to 0]
**since** | Option<**String**> | Filter by created at after the given timestamp |  |
**size** | Option<**i32**> | Optional page size in alias list pagination |  |[default to 20]
**sort** | Option<**String**> | Optional createdAt sort direction ASC or DESC |  |[default to ASC]

### Return type

[**crate::models::PageAlias**](PageAlias)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## reply_to_alias_email

> crate::models::SentEmailDto reply_to_alias_email(alias_id, email_id, reply_to_alias_email_options)
Reply to an email

Send the reply to the email sender or reply-to and include same subject cc bcc etc. Reply to an email and the contents will be sent with the existing subject to the emails `to`, `cc`, and `bcc`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alias_id** | [**String**]() | ID of the alias that email belongs to | [required] |
**email_id** | [**String**]() | ID of the email that should be replied to | [required] |
**reply_to_alias_email_options** | [**ReplyToAliasEmailOptions**](ReplyToAliasEmailOptions) | replyToAliasEmailOptions | [required] |

### Return type

[**crate::models::SentEmailDto**](SentEmailDto)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## send_alias_email

> crate::models::SentEmailDto send_alias_email(alias_id, send_email_options)
Send an email from an alias inbox

Send an email from an alias. Replies to the email will be forwarded to the alias masked email address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alias_id** | [**String**]() | aliasId | [required] |
**send_email_options** | Option<[**SendEmailOptions**](SendEmailOptions)> | Options for the email to be sent |  |

### Return type

[**crate::models::SentEmailDto**](SentEmailDto)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## update_alias

> update_alias(alias_id, update_alias_options)
Update an email alias

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alias_id** | [**String**]() | aliasId | [required] |
**update_alias_options** | [**UpdateAliasOptions**](UpdateAliasOptions) | updateAliasOptions | [required] |

### Return type

 (empty response body)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)

