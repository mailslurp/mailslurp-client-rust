# \InboxControllerApi

All URIs are relative to *https://api.mailslurp.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_inbox**](InboxControllerApi.md#create_inbox) | **Post** /inboxes | Create an Inbox (email address)
[**create_inbox_with_options**](InboxControllerApi.md#create_inbox_with_options) | **Post** /inboxes/withOptions | Create an inbox with additional options
[**delete_all_inboxes**](InboxControllerApi.md#delete_all_inboxes) | **Delete** /inboxes | Delete all inboxes
[**delete_inbox**](InboxControllerApi.md#delete_inbox) | **Delete** /inboxes/{inboxId} | Delete inbox
[**get_all_inboxes**](InboxControllerApi.md#get_all_inboxes) | **Get** /inboxes/paginated | List All Inboxes Paginated
[**get_emails**](InboxControllerApi.md#get_emails) | **Get** /inboxes/{inboxId}/emails | Get emails in an Inbox. This method is not idempotent as it allows retries and waits if you want certain conditions to be met before returning. For simple listing and sorting of known emails use the email controller instead.
[**get_inbox**](InboxControllerApi.md#get_inbox) | **Get** /inboxes/{inboxId} | Get Inbox
[**get_inbox_emails_paginated**](InboxControllerApi.md#get_inbox_emails_paginated) | **Get** /inboxes/{inboxId}/emails/paginated | Get inbox emails paginated
[**get_inbox_sent_emails**](InboxControllerApi.md#get_inbox_sent_emails) | **Get** /inboxes/{inboxId}/sent | Get Inbox Sent Emails
[**get_inbox_tags**](InboxControllerApi.md#get_inbox_tags) | **Get** /inboxes/tags | Get inbox tags
[**get_inboxes**](InboxControllerApi.md#get_inboxes) | **Get** /inboxes | List Inboxes / Email Addresses
[**get_organization_inboxes**](InboxControllerApi.md#get_organization_inboxes) | **Get** /inboxes/organization | List Organization Inboxes Paginated
[**send_email**](InboxControllerApi.md#send_email) | **Post** /inboxes/{inboxId} | Send Email
[**send_email_and_confirm**](InboxControllerApi.md#send_email_and_confirm) | **Post** /inboxes/{inboxId}/confirm | Send email and return sent confirmation
[**set_inbox_favourited**](InboxControllerApi.md#set_inbox_favourited) | **Put** /inboxes/{inboxId}/favourite | Set inbox favourited state
[**update_inbox**](InboxControllerApi.md#update_inbox) | **Patch** /inboxes/{inboxId} | Update Inbox



## create_inbox

> crate::models::Inbox create_inbox(allow_team_access, description, email_address, expires_at, expires_in, favourite, name, tags, use_domain_pool)
Create an Inbox (email address)

Create a new inbox and with a randomized email address to send and receive from. Pass emailAddress parameter if you wish to use a specific email address. Creating an inbox is required before sending or receiving emails. If writing tests it is recommended that you create a new inbox during each test method so that it is unique and empty. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allow_team_access** | Option<**bool**> | Grant team access to this inbox and the emails that belong to it for team members of your organization. |  |
**description** | Option<**String**> | Optional description of the inbox for labelling purposes. Is shown in the dashboard and can be used with |  |
**email_address** | Option<**String**> | A custom email address to use with the inbox. Defaults to null. When null MailSlurp will assign a random email address to the inbox such as `123@mailslurp.com`. If you use the `useDomainPool` option when the email address is null it will generate an email address with a more varied domain ending such as `123@mailslurp.info` or `123@mailslurp.biz`. When a custom email address is provided the address is split into a domain and the domain is queried against your user. If you have created the domain in the MailSlurp dashboard and verified it you can use any email address that ends with the domain. Send an email to this address and the inbox will receive and store it for you. To retrieve the email use the Inbox and Email Controller endpoints with the inbox ID. |  |
**expires_at** | Option<**String**> | Optional inbox expiration date. If null then this inbox is permanent and the emails in it won't be deleted. If an expiration date is provided or is required by your plan the inbox will be closed when the expiration time is reached. Expired inboxes still contain their emails but can no longer send or receive emails. An ExpiredInboxRecord is created when an inbox and the email address and inbox ID are recorded. The expiresAt property is a timestamp string in ISO DateTime Format yyyy-MM-dd'T'HH:mm:ss.SSSXXX. |  |
**expires_in** | Option<**i64**> | Number of milliseconds that inbox should exist for |  |
**favourite** | Option<**bool**> | Is the inbox favorited. Favouriting inboxes is typically done in the dashboard for quick access or filtering |  |
**name** | Option<**String**> | Optional name of the inbox. Displayed in the dashboard for easier search |  |
**tags** | Option<[**Vec<String>**](String.md)> | Tags that inbox has been tagged with. Tags can be added to inboxes to group different inboxes within an account. You can also search for inboxes by tag in the dashboard UI. |  |
**use_domain_pool** | Option<**bool**> | Use the MailSlurp domain name pool with this inbox when creating the email address. Defaults to null. If enabled the inbox will be an email address with a domain randomly chosen from a list of the MailSlurp domains. This is useful when the default `@mailslurp.com` email addresses used with inboxes are blocked or considered spam by a provider or receiving service. When domain pool is enabled an email address will be generated ending in `@mailslurp.{world,info,xyz,...}` . This means a TLD is randomly selecting from a list of `.biz`, `.info`, `.xyz` etc to add variance to the generated email addresses. When null or false MailSlurp uses the default behavior of `@mailslurp.com` or custom email address provided by the emailAddress field. |  |

### Return type

[**crate::models::Inbox**](Inbox.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_inbox_with_options

> crate::models::Inbox create_inbox_with_options(create_inbox_dto)
Create an inbox with additional options

Additional endpoint that allows inbox creation with request body options. Can be more flexible that other methods for some clients.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_inbox_dto** | [**CreateInboxDto**](CreateInboxDto.md) | createInboxDto | [required] |

### Return type

[**crate::models::Inbox**](Inbox.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
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

Permanently delete an inbox and associated email address as well as all emails within the given inbox. This action cannot be undone. Note: deleting an inbox will not affect your account usage. Monthly inbox usage is based on how many inboxes you create within 30 days, not how many exist at time of request.

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

> crate::models::PageInboxProjection get_all_inboxes(favourite, page, search, size, sort, tag, team_access)
List All Inboxes Paginated

List inboxes in paginated form. The results are available on the `content` property of the returned object. This method allows for page index (zero based), page size (how many results to return), and a sort direction (based on createdAt time). You Can also filter by whether an inbox is favorited or use email address pattern. This method is the recommended way to query inboxes. The alternative `getInboxes` method returns a full list of inboxes but is limited to 100 results. Results do not include team access inboxes by default. Use organization method to list team inboxes or set `teamAccess` to true.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**favourite** | Option<**bool**> | Optionally filter results for favourites only |  |[default to false]
**page** | Option<**i32**> | Optional page index in inbox list pagination |  |[default to 0]
**search** | Option<**String**> | Optionally filter by search words partial matching ID, tags, name, and email address |  |
**size** | Option<**i32**> | Optional page size in inbox list pagination |  |[default to 20]
**sort** | Option<**String**> | Optional createdAt sort direction ASC or DESC |  |[default to ASC]
**tag** | Option<**String**> | Optionally filter by tags. Will return inboxes that include given tags |  |
**team_access** | Option<**bool**> | Optionally filter by team access. Defaults to false so organization inboxes are not included |  |[default to false]

### Return type

[**crate::models::PageInboxProjection**](PageInboxProjection.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_emails

> Vec<crate::models::EmailPreview> get_emails(inbox_id, limit, min_count, retry_timeout, since, size, sort)
Get emails in an Inbox. This method is not idempotent as it allows retries and waits if you want certain conditions to be met before returning. For simple listing and sorting of known emails use the email controller instead.

List emails that an inbox has received. Only emails that are sent to the inbox's email address will appear in the inbox. It may take several seconds for any email you send to an inbox's email address to appear in the inbox. To make this endpoint wait for a minimum number of emails use the `minCount` parameter. The server will retry the inbox database until the `minCount` is satisfied or the `retryTimeout` is reached

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inbox_id** | [**String**](.md) | Id of inbox that emails belongs to | [required] |
**limit** | Option<**i32**> | Limit the result set, ordered by received date time sort direction. Maximum 100. For more listing options see the email controller |  |
**min_count** | Option<**i64**> | Minimum acceptable email count. Will cause request to hang (and retry) until minCount is satisfied or retryTimeout is reached. |  |
**retry_timeout** | Option<**i64**> | Maximum milliseconds to spend retrying inbox database until minCount emails are returned |  |
**since** | Option<**String**> | Exclude emails received before this ISO 8601 date time |  |
**size** | Option<**i32**> | Alias for limit. Assessed first before assessing any passed limit. |  |
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

> Vec<crate::models::Inbox> get_inboxes(size, sort)
List Inboxes / Email Addresses

List the inboxes you have created. Note use of the more advanced `getAllEmails` is recommended. You can provide a limit and sort parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**size** | Option<**i32**> | Optional result size limit. Note an automatic limit of 100 results is applied. See the paginated `getAllEmails` for larger queries. |  |[default to 100]
**sort** | Option<**String**> | Optional createdAt sort direction ASC or DESC |  |[default to ASC]

### Return type

[**Vec<crate::models::Inbox>**](Inbox.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_inboxes

> crate::models::PageOrganizationInboxProjection get_organization_inboxes(page, size, sort)
List Organization Inboxes Paginated

List organization inboxes in paginated form. These are inboxes created with `allowTeamAccess` flag enabled. Organization inboxes are `readOnly` for non-admin users. The results are available on the `content` property of the returned object. This method allows for page index (zero based), page size (how many results to return), and a sort direction (based on createdAt time). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Optional page index in inbox list pagination |  |[default to 0]
**size** | Option<**i32**> | Optional page size in inbox list pagination |  |[default to 20]
**sort** | Option<**String**> | Optional createdAt sort direction ASC or DESC |  |[default to ASC]

### Return type

[**crate::models::PageOrganizationInboxProjection**](PageOrganizationInboxProjection.md)

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

> crate::models::SentEmailDto send_email_and_confirm(inbox_id, send_email_options)
Send email and return sent confirmation

Sister method for standard `sendEmail` method with the benefit of returning a `SentEmail` entity confirming the successful sending of the email with link the the sent object created for it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inbox_id** | [**String**](.md) | ID of the inbox you want to send the email from | [required] |
**send_email_options** | Option<[**SendEmailOptions**](SendEmailOptions.md)> | Options for the email |  |

### Return type

[**crate::models::SentEmailDto**](SentEmailDto.md)

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

