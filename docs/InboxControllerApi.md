# \InboxControllerApi

All URIs are relative to *https://api.mailslurp.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_inbox**](InboxControllerApi.md#create_inbox) | **Post** /inboxes | Create an Inbox (email address)
[**delete_all_inboxes**](InboxControllerApi.md#delete_all_inboxes) | **Delete** /inboxes | Delete all inboxes
[**delete_inbox**](InboxControllerApi.md#delete_inbox) | **Delete** /inboxes/{inboxId} | Delete inbox
[**get_all_inboxes**](InboxControllerApi.md#get_all_inboxes) | **Get** /inboxes/paginated | List Inboxes Paginated
[**get_emails**](InboxControllerApi.md#get_emails) | **Get** /inboxes/{inboxId}/emails | Get emails in an Inbox
[**get_inbox**](InboxControllerApi.md#get_inbox) | **Get** /inboxes/{inboxId} | Get Inbox
[**get_inbox_emails_paginated**](InboxControllerApi.md#get_inbox_emails_paginated) | **Get** /inboxes/{inboxId}/emails/paginated | Get inbox emails paginated
[**get_inbox_sent_emails**](InboxControllerApi.md#get_inbox_sent_emails) | **Get** /inboxes/{inboxId}/sent | Get Inbox Sent Emails
[**get_inbox_tags**](InboxControllerApi.md#get_inbox_tags) | **Get** /inboxes/tags | Get inbox tags
[**get_inboxes**](InboxControllerApi.md#get_inboxes) | **Get** /inboxes | List Inboxes / Email Addresses
[**send_email**](InboxControllerApi.md#send_email) | **Post** /inboxes/{inboxId} | Send Email
[**send_email_and_confirm**](InboxControllerApi.md#send_email_and_confirm) | **Post** /inboxes/{inboxId}/confirm | Send email and return sent confirmation
[**set_inbox_favourited**](InboxControllerApi.md#set_inbox_favourited) | **Put** /inboxes/{inboxId}/favourite | Set inbox favourited state
[**update_inbox**](InboxControllerApi.md#update_inbox) | **Patch** /inboxes/{inboxId} | Update Inbox



## create_inbox

> crate::models::Inbox create_inbox(description, email_address, expires_at, favourite, name, tags)
Create an Inbox (email address)

Create a new inbox and with a randomized email address to send and receive from. Pass emailAddress parameter if you wish to use a specific email address. Creating an inbox is required before sending or receiving emails. If writing tests it is recommended that you create a new inbox during each test method so that it is unique and empty. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**description** | Option<**String**> | Optional description for an inbox. |  |
**email_address** | Option<**String**> | Optional email address including domain you wish inbox to use (eg: test123@mydomain.com). Only supports domains that you have registered and verified with MailSlurp using dashboard or `createDomain` method. |  |
**expires_at** | Option<**String**> | Optional expires at timestamp. If your plan supports this feature you can specify when an inbox should expire. If left empty inbox will exist permanently or expire when your plan dictates |  |
**favourite** | Option<**bool**> | Is inbox favourited. |  |
**name** | Option<**String**> | Optional name for an inbox. |  |
**tags** | Option<[**Vec<String>**](String.md)> | Optional tags for an inbox. Can be used for searching and filtering inboxes. |  |

### Return type

[**crate::models::Inbox**](Inbox.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_all_inboxes

> delete_all_inboxes()
Delete all inboxes

Permanently delete all inboxes and associated email addresses. This will also delete all emails within the inboxes. Be careful as inboxes cannot be recovered once deleted. Note: deleting inboxes will not impact your usage limits. Monthly inbox creation limits are based on how many inboxes were created in the last 30 days, not how many inboxes you currently have.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_inbox

> delete_inbox(inbox_id)
Delete inbox

Permanently delete an inbox and associated email address aswell as all emails within the given inbox. This action cannot be undone. Note: deleting an inbox will not affect your account usage. Monthly inbox usage is based on how many inboxes you create within 30 days, not how many exist at time of request.

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


## get_all_inboxes

> crate::models::PageInboxProjection get_all_inboxes(favourite, page, search, size, sort, tag)
List Inboxes Paginated

List inboxes in paginated form. Allows for page index, page size, and sort direction. Can also filter by favourited or email address like pattern.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**favourite** | Option<**bool**> | Optionally filter results for favourites only |  |[default to false]
**page** | Option<**i32**> | Optional page index in inbox list pagination |  |[default to 0]
**search** | Option<**String**> | Optionally filter by search words partial matching ID, tags, name, and email address |  |
**size** | Option<**i32**> | Optional page size in inbox list pagination |  |[default to 20]
**sort** | Option<**String**> | Optional createdAt sort direction ASC or DESC |  |[default to ASC]
**tag** | Option<**String**> | Optionally filter by tags |  |

### Return type

[**crate::models::PageInboxProjection**](PageInboxProjection.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_emails

> Vec<crate::models::EmailPreview> get_emails(inbox_id, limit, min_count, retry_timeout, since, sort)
Get emails in an Inbox

List emails that an inbox has received. Only emails that are sent to the inbox's email address will appear in the inbox. It may take several seconds for any email you send to an inbox's email address to appear in the inbox. To make this endpoint wait for a minimum number of emails use the `minCount` parameter. The server will retry the inbox database until the `minCount` is satisfied or the `retryTimeout` is reached

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inbox_id** | [**String**](.md) | Id of inbox that emails belongs to | [required] |
**limit** | Option<**i32**> | Limit the result set, ordered by received date time sort direction |  |
**min_count** | Option<**i64**> | Minimum acceptable email count. Will cause request to hang (and retry) until minCount is satisfied or retryTimeout is reached. |  |
**retry_timeout** | Option<**i64**> | Maximum milliseconds to spend retrying inbox database until minCount emails are returned |  |
**since** | Option<**String**> | Exclude emails received before this ISO 8601 date time |  |
**sort** | Option<**String**> | Sort the results by received date and direction ASC or DESC |  |

### Return type

[**Vec<crate::models::EmailPreview>**](EmailPreview.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_inbox

> crate::models::Inbox get_inbox(inbox_id)
Get Inbox

Returns an inbox's properties, including its email address and ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inbox_id** | [**String**](.md) | inboxId | [required] |

### Return type

[**crate::models::Inbox**](Inbox.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_inbox_emails_paginated

> crate::models::PageEmailPreview get_inbox_emails_paginated(inbox_id, page, size, sort)
Get inbox emails paginated

Get a paginated list of emails in an inbox. Does not hold connections open.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inbox_id** | [**String**](.md) | Id of inbox that emails belongs to | [required] |
**page** | Option<**i32**> | Optional page index in inbox emails list pagination |  |[default to 0]
**size** | Option<**i32**> | Optional page size in inbox emails list pagination |  |[default to 20]
**sort** | Option<**String**> | Optional createdAt sort direction ASC or DESC |  |[default to ASC]

### Return type

[**crate::models::PageEmailPreview**](PageEmailPreview.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_inbox_sent_emails

> crate::models::PageSentEmailProjection get_inbox_sent_emails(inbox_id, page, size, sort)
Get Inbox Sent Emails

Returns an inbox's sent email receipts. Call individual sent email endpoints for more details. Note for privacy reasons the full body of sent emails is never stored. An MD5 hash hex is available for comparison instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inbox_id** | [**String**](.md) | inboxId | [required] |
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


## get_inbox_tags

> Vec<String> get_inbox_tags()
Get inbox tags

Get all inbox tags

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_inboxes

> Vec<crate::models::Inbox> get_inboxes()
List Inboxes / Email Addresses

List the inboxes you have created

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Inbox>**](Inbox.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_email

> send_email(inbox_id, send_email_options)
Send Email

Send an email from an inbox's email address.  The request body should contain the `SendEmailOptions` that include recipients, attachments, body etc. See `SendEmailOptions` for all available properties. Note the `inboxId` refers to the inbox's id not the inbox's email address. See https://www.mailslurp.com/guides/ for more information on how to send emails. This method does not return a sent email entity due to legacy reasons. To send and get a sent email as returned response use the sister method `sendEmailAndConfirm`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inbox_id** | [**String**](.md) | ID of the inbox you want to send the email from | [required] |
**send_email_options** | Option<[**SendEmailOptions**](SendEmailOptions.md)> | Options for the email |  |

### Return type

 (empty response body)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_email_and_confirm

> crate::models::SentEmail send_email_and_confirm(inbox_id, send_email_options)
Send email and return sent confirmation

Sister method for standard `sendEmail` method with the benefit of returning a `SentEmail` entity confirming the successful sending of the email with link the the sent object created for it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inbox_id** | [**String**](.md) | ID of the inbox you want to send the email from | [required] |
**send_email_options** | Option<[**SendEmailOptions**](SendEmailOptions.md)> | Options for the email |  |

### Return type

[**crate::models::SentEmail**](SentEmail.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_inbox_favourited

> crate::models::Inbox set_inbox_favourited(inbox_id, set_inbox_favourited_options)
Set inbox favourited state

Set and return new favourite state for an inbox

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inbox_id** | [**String**](.md) | inboxId | [required] |
**set_inbox_favourited_options** | [**SetInboxFavouritedOptions**](SetInboxFavouritedOptions.md) | setInboxFavouritedOptions | [required] |

### Return type

[**crate::models::Inbox**](Inbox.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_inbox

> crate::models::Inbox update_inbox(inbox_id, update_inbox_options)
Update Inbox

Update editable fields on an inbox

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inbox_id** | [**String**](.md) | inboxId | [required] |
**update_inbox_options** | [**UpdateInboxOptions**](UpdateInboxOptions.md) | updateInboxOptions | [required] |

### Return type

[**crate::models::Inbox**](Inbox.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

