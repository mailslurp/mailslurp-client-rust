# \EmailControllerApi

All URIs are relative to *https://api.mailslurp.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_all_emails**](EmailControllerApi#delete_all_emails) | **delete** /emails | Delete all emails in all inboxes.
[**delete_email**](EmailControllerApi#delete_email) | **delete** /emails/{emailId} | Delete an email
[**download_attachment**](EmailControllerApi#download_attachment) | **get** /emails/{emailId}/attachments/{attachmentId} | Get email attachment bytes. Returned as `octet-stream` with content type header. If you have trouble with byte responses try the `downloadAttachmentBase64` response endpoints and convert the base 64 encoded content to a file or string.
[**download_attachment_base64**](EmailControllerApi#download_attachment_base64) | **get** /emails/{emailId}/attachments/{attachmentId}/base64 | Get email attachment as base64 encoded string as an alternative to binary responses. Decode the `base64FileContents` as a `utf-8` encoded string or array of bytes depending on the `contentType`.
[**download_body**](EmailControllerApi#download_body) | **get** /emails/{emailId}/body | Get email body as string. Returned as `plain/text` with content type header.
[**download_body_bytes**](EmailControllerApi#download_body_bytes) | **get** /emails/{emailId}/body-bytes | Get email body in bytes. Returned as `octet-stream` with content type header.
[**forward_email**](EmailControllerApi#forward_email) | **post** /emails/{emailId}/forward | Forward email to recipients
[**get_attachment_meta_data**](EmailControllerApi#get_attachment_meta_data) | **get** /emails/{emailId}/attachments/{attachmentId}/metadata | Get email attachment metadata. This is the `contentType` and `contentLength` of an attachment. To get the individual attachments  use the `downloadAttachment` methods.
[**get_attachments1**](EmailControllerApi#get_attachments1) | **get** /emails/{emailId}/attachments | Get all email attachment metadata. Metadata includes name and size of attachments.
[**get_email**](EmailControllerApi#get_email) | **get** /emails/{emailId} | Get email content including headers and body. Expects email to exist by ID. For emails that may not have arrived yet use the WaitForController.
[**get_email_content_match**](EmailControllerApi#get_email_content_match) | **post** /emails/{emailId}/contentMatch | Get email content regex pattern match results. Runs regex against email body and returns match groups.
[**get_email_html**](EmailControllerApi#get_email_html) | **get** /emails/{emailId}/html | Get email content as HTML. For displaying emails in browser context.
[**get_email_html_query**](EmailControllerApi#get_email_html_query) | **get** /emails/{emailId}/htmlQuery | Parse and return text from an email, stripping HTML and decoding encoded characters
[**get_email_text_lines**](EmailControllerApi#get_email_text_lines) | **get** /emails/{emailId}/textLines | Parse and return text from an email, stripping HTML and decoding encoded characters
[**get_emails_paginated**](EmailControllerApi#get_emails_paginated) | **get** /emails | Get all emails in all inboxes in paginated form. Email API list all.
[**get_latest_email**](EmailControllerApi#get_latest_email) | **get** /emails/latest | Get latest email in all inboxes. Most recently received.
[**get_latest_email_in_inbox**](EmailControllerApi#get_latest_email_in_inbox) | **get** /emails/latestIn | Get latest email in an inbox. Use `WaitForController` to get emails that may not have arrived yet.
[**get_organization_emails_paginated**](EmailControllerApi#get_organization_emails_paginated) | **get** /emails/organization | Get all organization emails. List team or shared test email accounts
[**get_raw_email_contents**](EmailControllerApi#get_raw_email_contents) | **get** /emails/{emailId}/raw | Get raw email string. Returns unparsed raw SMTP message with headers and body.
[**get_raw_email_json**](EmailControllerApi#get_raw_email_json) | **get** /emails/{emailId}/raw/json | Get raw email in JSON. Unparsed SMTP message in JSON wrapper format.
[**get_unread_email_count**](EmailControllerApi#get_unread_email_count) | **get** /emails/unreadCount | Get unread email count
[**reply_to_email**](EmailControllerApi#reply_to_email) | **put** /emails/{emailId} | Reply to an email
[**send_email_source_optional**](EmailControllerApi#send_email_source_optional) | **post** /emails | Send email
[**validate_email**](EmailControllerApi#validate_email) | **post** /emails/{emailId}/validate | Validate email HTML contents



## delete_all_emails

> delete_all_emails()
Delete all emails in all inboxes.

Deletes all emails in your account. Be careful as emails cannot be recovered

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## delete_email

> delete_email(email_id)
Delete an email

Deletes an email and removes it from the inbox. Deleted emails cannot be recovered.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_id** | [**String**]() | ID of email to delete | [required] |

### Return type

 (empty response body)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## download_attachment

> String download_attachment(attachment_id, email_id, api_key)
Get email attachment bytes. Returned as `octet-stream` with content type header. If you have trouble with byte responses try the `downloadAttachmentBase64` response endpoints and convert the base 64 encoded content to a file or string.

Returns the specified attachment for a given email as a stream / array of bytes. You can find attachment ids in email responses endpoint responses. The response type is application/octet-stream.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attachment_id** | **String** | ID of attachment | [required] |
**email_id** | [**String**]() | ID of email | [required] |
**api_key** | Option<**String**> | Can pass apiKey in url for this request if you wish to download the file in a browser. Content type will be set to original content type of the attachment file. This is so that browsers can download the file correctly. |  |

### Return type

**String**

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## download_attachment_base64

> crate::models::DownloadAttachmentDto download_attachment_base64(attachment_id, email_id)
Get email attachment as base64 encoded string as an alternative to binary responses. Decode the `base64FileContents` as a `utf-8` encoded string or array of bytes depending on the `contentType`.

Returns the specified attachment for a given email as a base 64 encoded string. The response type is application/json. This method is similar to the `downloadAttachment` method but allows some clients to get around issues with binary responses.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attachment_id** | **String** | ID of attachment | [required] |
**email_id** | [**String**]() | ID of email | [required] |

### Return type

[**crate::models::DownloadAttachmentDto**](DownloadAttachmentDto)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## download_body

> String download_body(email_id)
Get email body as string. Returned as `plain/text` with content type header.

Returns the specified email body for a given email as a string

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_id** | [**String**]() | ID of email | [required] |

### Return type

**String**

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/html, text/plain

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## download_body_bytes

> String download_body_bytes(email_id)
Get email body in bytes. Returned as `octet-stream` with content type header.

Returns the specified email body for a given email as a stream / array of bytes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_id** | [**String**]() | ID of email | [required] |

### Return type

**String**

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## forward_email

> forward_email(email_id, forward_email_options)
Forward email to recipients

Forward an existing email to new recipients. The sender of the email will be the inbox that received the email you are forwarding. You can override the sender with the `from` option. Note you must have access to the from address in MailSlurp to use the override. For more control consider fetching the email and sending it a new using the send email endpoints.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_id** | [**String**]() | ID of email | [required] |
**forward_email_options** | [**ForwardEmailOptions**](ForwardEmailOptions) | forwardEmailOptions | [required] |

### Return type

 (empty response body)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_attachment_meta_data

> crate::models::AttachmentMetaData get_attachment_meta_data(attachment_id, email_id)
Get email attachment metadata. This is the `contentType` and `contentLength` of an attachment. To get the individual attachments  use the `downloadAttachment` methods.

Returns the metadata such as name and content-type for a given attachment and email.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attachment_id** | **String** | ID of attachment | [required] |
**email_id** | [**String**]() | ID of email | [required] |

### Return type

[**crate::models::AttachmentMetaData**](AttachmentMetaData)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_attachments1

> Vec<crate::models::AttachmentMetaData> get_attachments1(email_id)
Get all email attachment metadata. Metadata includes name and size of attachments.

Returns an array of attachment metadata such as name and content-type for a given email if present.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_id** | [**String**]() | ID of email | [required] |

### Return type

[**Vec<crate::models::AttachmentMetaData>**](AttachmentMetaData)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_email

> crate::models::Email get_email(email_id, decode)
Get email content including headers and body. Expects email to exist by ID. For emails that may not have arrived yet use the WaitForController.

Returns a email summary object with headers and content. To retrieve the raw unparsed email use the getRawEmail endpoints

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_id** | [**String**]() | emailId | [required] |
**decode** | Option<**bool**> | Decode email body quoted-printable encoding to plain text. SMTP servers often encode text using quoted-printable format (for instance `=D7`). This can be a pain for testing |  |[default to false]

### Return type

[**crate::models::Email**](Email)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_email_content_match

> crate::models::EmailContentMatchResult get_email_content_match(email_id, content_match_options)
Get email content regex pattern match results. Runs regex against email body and returns match groups.

Return the matches for a given Java style regex pattern. Do not include the typical `/` at start or end of regex in some languages. Given an example `your code is: 12345` the pattern to extract match looks like `code is: (\\d{6})`. This will return an array of matches with the first matching the entire pattern and the subsequent matching the groups: `['code is: 123456', '123456']` See https://docs.oracle.com/javase/8/docs/api/java/util/regex/Pattern.html for more information of available patterns. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_id** | [**String**]() | ID of email to match against | [required] |
**content_match_options** | [**ContentMatchOptions**](ContentMatchOptions) | contentMatchOptions | [required] |

### Return type

[**crate::models::EmailContentMatchResult**](EmailContentMatchResult)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_email_html

> String get_email_html(email_id, decode)
Get email content as HTML. For displaying emails in browser context.

Retrieve email content as HTML response for viewing in browsers. Decodes quoted-printable entities and converts charset to UTF-8. Pass your API KEY as a request parameter when viewing in a browser: `?apiKey=xxx`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_id** | [**String**]() | emailId | [required] |
**decode** | Option<**bool**> | decode |  |[default to false]

### Return type

**String**

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/html

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_email_html_query

> crate::models::EmailTextLinesResult get_email_html_query(email_id, html_selector)
Parse and return text from an email, stripping HTML and decoding encoded characters

Parse an email body and return the content as an array of text. HTML parsing uses JSoup which supports JQuery/CSS style selectors

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_id** | [**String**]() | ID of email to perform HTML query on | [required] |
**html_selector** | Option<**String**> | HTML selector to search for. Uses JQuery/JSoup/CSS style selector like '.my-div' to match content. See https://jsoup.org/apidocs/org/jsoup/select/Selector.html for more information. |  |

### Return type

[**crate::models::EmailTextLinesResult**](EmailTextLinesResult)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_email_text_lines

> crate::models::EmailTextLinesResult get_email_text_lines(email_id, decode_html_entities, line_separator)
Parse and return text from an email, stripping HTML and decoding encoded characters

Parse an email body and return the content as an array of strings. HTML parsing uses JSoup and UNIX line separators.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_id** | [**String**]() | ID of email to fetch text for | [required] |
**decode_html_entities** | Option<**bool**> | Decode HTML entities |  |
**line_separator** | Option<**String**> | Line separator character |  |

### Return type

[**crate::models::EmailTextLinesResult**](EmailTextLinesResult)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_emails_paginated

> crate::models::PageEmailProjection get_emails_paginated(inbox_id, page, search_filter, size, sort, unread_only)
Get all emails in all inboxes in paginated form. Email API list all.

By default returns all emails across all inboxes sorted by ascending created at date. Responses are paginated. You can restrict results to a list of inbox IDs. You can also filter out read messages

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inbox_id** | Option<[**Vec<String>**](String)> | Optional inbox ids to filter by. Can be repeated. By default will use all inboxes belonging to your account. |  |
**page** | Option<**i32**> | Optional page index in email list pagination |  |[default to 0]
**search_filter** | Option<**String**> | Optional search filter. Searches email recipients, sender, subject, email address and ID. Does not search email body |  |[default to false]
**size** | Option<**i32**> | Optional page size in email list pagination. Maximum size is 100. Use page index and sort to page through larger results |  |[default to 20]
**sort** | Option<**String**> | Optional createdAt sort direction ASC or DESC |  |[default to ASC]
**unread_only** | Option<**bool**> | Optional filter for unread emails only. All emails are considered unread until they are viewed in the dashboard or requested directly |  |[default to false]

### Return type

[**crate::models::PageEmailProjection**](PageEmailProjection)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_latest_email

> crate::models::Email get_latest_email(inbox_ids)
Get latest email in all inboxes. Most recently received.

Get the newest email in all inboxes or in a passed set of inbox IDs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inbox_ids** | Option<[**Vec<String>**](String)> | Optional set of inboxes to filter by. Only get the latest email from these inbox IDs |  |

### Return type

[**crate::models::Email**](Email)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_latest_email_in_inbox

> crate::models::Email get_latest_email_in_inbox(inbox_id)
Get latest email in an inbox. Use `WaitForController` to get emails that may not have arrived yet.

Get the newest email in all inboxes or in a passed set of inbox IDs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inbox_id** | Option<[**String**]()> | ID of the inbox you want to get the latest email from |  |

### Return type

[**crate::models::Email**](Email)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_organization_emails_paginated

> crate::models::PageEmailProjection get_organization_emails_paginated(inbox_id, page, search_filter, size, sort, unread_only)
Get all organization emails. List team or shared test email accounts

By default returns all emails across all team inboxes sorted by ascending created at date. Responses are paginated. You can restrict results to a list of inbox IDs. You can also filter out read messages

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inbox_id** | Option<[**Vec<String>**](String)> | Optional inbox ids to filter by. Can be repeated. By default will use all inboxes belonging to your account. |  |
**page** | Option<**i32**> | Optional page index in email list pagination |  |[default to 0]
**search_filter** | Option<**String**> | Optional search filter search filter for emails. |  |
**size** | Option<**i32**> | Optional page size in email list pagination. Maximum size is 100. Use page index and sort to page through larger results |  |[default to 20]
**sort** | Option<**String**> | Optional createdAt sort direction ASC or DESC |  |[default to ASC]
**unread_only** | Option<**bool**> | Optional filter for unread emails only. All emails are considered unread until they are viewed in the dashboard or requested directly |  |[default to false]

### Return type

[**crate::models::PageEmailProjection**](PageEmailProjection)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_raw_email_contents

> String get_raw_email_contents(email_id)
Get raw email string. Returns unparsed raw SMTP message with headers and body.

Returns a raw, unparsed, and unprocessed email. If your client has issues processing the response it is likely due to the response content-type which is text/plain. If you need a JSON response content-type use the getRawEmailJson endpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_id** | [**String**]() | ID of email | [required] |

### Return type

**String**

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_raw_email_json

> crate::models::RawEmailJson get_raw_email_json(email_id)
Get raw email in JSON. Unparsed SMTP message in JSON wrapper format.

Returns a raw, unparsed, and unprocessed email wrapped in a JSON response object for easier handling when compared with the getRawEmail text/plain response

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_id** | [**String**]() | ID of email | [required] |

### Return type

[**crate::models::RawEmailJson**](RawEmailJson)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_unread_email_count

> crate::models::UnreadCount get_unread_email_count()
Get unread email count

Get number of emails unread. Unread means has not been viewed in dashboard or returned in an email API response

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::UnreadCount**](UnreadCount)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## reply_to_email

> crate::models::SentEmailDto reply_to_email(email_id, reply_to_email_options)
Reply to an email

Send the reply to the email sender or reply-to and include same subject cc bcc etc. Reply to an email and the contents will be sent with the existing subject to the emails `to`, `cc`, and `bcc`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_id** | [**String**]() | ID of the email that should be replied to | [required] |
**reply_to_email_options** | [**ReplyToEmailOptions**](ReplyToEmailOptions) | replyToEmailOptions | [required] |

### Return type

[**crate::models::SentEmailDto**](SentEmailDto)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## send_email_source_optional

> send_email_source_optional(inbox_id, use_domain_pool, send_email_options)
Send email

Alias for `InboxController.sendEmail` method - see original method for full details. Sends an email from a given inbox that you have created. If no inbox is supplied a random inbox will be created for you and used to send the email.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inbox_id** | Option<[**String**]()> | ID of the inbox you want to send the email from |  |
**use_domain_pool** | Option<**bool**> | Use domain pool. Optionally create inbox to send from using the mailslurp domain pool. |  |
**send_email_options** | Option<[**SendEmailOptions**](SendEmailOptions)> | Options for the email |  |

### Return type

 (empty response body)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## validate_email

> crate::models::ValidationDto validate_email(email_id)
Validate email HTML contents

Validate the HTML content of email if HTML is found. Considered valid if no HTML is present.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_id** | [**String**]() | ID of email | [required] |

### Return type

[**crate::models::ValidationDto**](ValidationDto)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)

