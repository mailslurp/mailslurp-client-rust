# \AliasControllerApi

All URIs are relative to *https://api.mailslurp.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_alias**](AliasControllerApi.md#create_alias) | **Post** /aliases | Create an email alias. Must be verified by clicking link inside verification email that will be sent to the address. Once verified the alias will be active.
[**delete_alias**](AliasControllerApi.md#delete_alias) | **Delete** /aliases/{aliasId} | Delete an email alias
[**get_alias**](AliasControllerApi.md#get_alias) | **Get** /aliases/{aliasId} | Get an email alias
[**get_alias_emails**](AliasControllerApi.md#get_alias_emails) | **Get** /aliases/{aliasId}/emails | Get emails for an alias
[**get_alias_threads**](AliasControllerApi.md#get_alias_threads) | **Get** /aliases/{aliasId}/threads | Get threads created for an alias
[**get_aliases**](AliasControllerApi.md#get_aliases) | **Get** /aliases | Get all email aliases you have created
[**reply_to_alias_email**](AliasControllerApi.md#reply_to_alias_email) | **Put** /aliases/{aliasId}/emails/{emailId} | Reply to an email
[**send_alias_email**](AliasControllerApi.md#send_alias_email) | **Post** /aliases/{aliasId}/emails | Send an email from an alias inbox
[**update_alias**](AliasControllerApi.md#update_alias) | **Put** /aliases/{aliasId} | Update an email alias



## create_alias

> crate::models::AliasDto create_alias(create_alias_options)
Create an email alias. Must be verified by clicking link inside verification email that will be sent to the address. Once verified the alias will be active.

Email aliases use a MailSlurp randomly generated email address (or a custom domain inbox that you provide) to mask or proxy a real email address. Emails sent to the alias address will be forwarded to the hidden email address it was created for. If you want to send a reply use the threadId attached

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_alias_options** | [**CreateAliasOptions**](CreateAliasOptions.md) | createAliasOptions | [required] |

### Return type

[**crate::models::AliasDto**](AliasDto.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_alias

> delete_alias(alias_id)
Delete an email alias

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alias_id** | [**String**](.md) | aliasId | [required] |

### Return type

 (empty response body)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_alias

> crate::models::AliasDto get_alias(alias_id)
Get an email alias

Get an email alias by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alias_id** | [**String**](.md) | aliasId | [required] |

### Return type

[**crate::models::AliasDto**](AliasDto.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_alias_emails

> crate::models::PageEmailProjection get_alias_emails(alias_id, page, size, sort)
Get emails for an alias

Get paginated emails for an alias by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alias_id** | [**String**](.md) | aliasId | [required] |
**page** | Option<**i32**> | Optional page index alias email list pagination |  |[default to 0]
**size** | Option<**i32**> | Optional page size alias email list pagination |  |[default to 20]
**sort** | Option<**String**> | Optional createdAt sort direction ASC or DESC |  |[default to ASC]

### Return type

[**crate::models::PageEmailProjection**](PageEmailProjection.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_alias_threads

> crate::models::PageThreadProjection get_alias_threads(alias_id, page, size, sort)
Get threads created for an alias

Returns threads created for an email alias in paginated form

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alias_id** | [**String**](.md) | aliasId | [required] |
**page** | Option<**i32**> | Optional page index in thread list pagination |  |[default to 0]
**size** | Option<**i32**> | Optional page size in thread list pagination |  |[default to 20]
**sort** | Option<**String**> | Optional createdAt sort direction ASC or DESC |  |[default to ASC]

### Return type

[**crate::models::PageThreadProjection**](PageThreadProjection.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_aliases

> crate::models::PageAlias get_aliases(page, size, sort)
Get all email aliases you have created

Get all email aliases in paginated form

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Optional page index in alias list pagination |  |[default to 0]
**size** | Option<**i32**> | Optional page size in alias list pagination |  |[default to 20]
**sort** | Option<**String**> | Optional createdAt sort direction ASC or DESC |  |[default to ASC]

### Return type

[**crate::models::PageAlias**](PageAlias.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reply_to_alias_email

> crate::models::SentEmailDto reply_to_alias_email(alias_id, email_id, reply_to_alias_email_options)
Reply to an email

Send the reply to the email sender or reply-to and include same subject cc bcc etc. Reply to an email and the contents will be sent with the existing subject to the emails `to`, `cc`, and `bcc`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alias_id** | [**String**](.md) | ID of the alias that email belongs to | [required] |
**email_id** | [**String**](.md) | ID of the email that should be replied to | [required] |
**reply_to_alias_email_options** | [**ReplyToAliasEmailOptions**](ReplyToAliasEmailOptions.md) | replyToAliasEmailOptions | [required] |

### Return type

[**crate::models::SentEmailDto**](SentEmailDto.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_alias_email

> crate::models::SentEmailDto send_alias_email(alias_id, send_email_options)
Send an email from an alias inbox

Send an email from an alias. Replies to the email will be forwared to the alias masked email address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alias_id** | [**String**](.md) | aliasId | [required] |
**send_email_options** | Option<[**SendEmailOptions**](SendEmailOptions.md)> | Options for the email to be sent |  |

### Return type

[**crate::models::SentEmailDto**](SentEmailDto.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_alias

> update_alias(alias_id, update_alias_options)
Update an email alias

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alias_id** | [**String**](.md) | aliasId | [required] |
**update_alias_options** | [**UpdateAliasOptions**](UpdateAliasOptions.md) | updateAliasOptions | [required] |

### Return type

 (empty response body)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

