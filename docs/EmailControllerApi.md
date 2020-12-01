# \EmailControllerApi

All URIs are relative to *https://api.mailslurp.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_all_emails**](EmailControllerApi.md#delete_all_emails) | **Delete** /emails | Delete all emails
[**delete_email**](EmailControllerApi.md#delete_email) | **Delete** /emails/{emailId} | Delete an email
[**download_attachment**](EmailControllerApi.md#download_attachment) | **Get** /emails/{emailId}/attachments/{attachmentId} | Get email attachment bytes. If you have trouble with byte responses try the `downloadAttachmentBase64` response endpoints.
[**download_attachment_base64**](EmailControllerApi.md#download_attachment_base64) | **Get** /emails/{emailId}/attachments/{attachmentId}/base64 | Get email attachment as base64 encoded string (alternative to binary responses)
[**forward_email**](EmailControllerApi.md#forward_email) | **Post** /emails/{emailId}/forward | Forward email
[**get_attachment_meta_data**](EmailControllerApi.md#get_attachment_meta_data) | **Get** /emails/{emailId}/attachments/{attachmentId}/metadata | Get email attachment metadata
[**get_attachments**](EmailControllerApi.md#get_attachments) | **Get** /emails/{emailId}/attachments | Get all email attachment metadata
[**get_email**](EmailControllerApi.md#get_email) | **Get** /emails/{emailId} | Get email content
[**get_email_content_match**](EmailControllerApi.md#get_email_content_match) | **Post** /emails/{emailId}/contentMatch | Get email content regex pattern match results. Runs regex against email body and returns match groups.
[**get_email_html**](EmailControllerApi.md#get_email_html) | **Get** /emails/{emailId}/html | Get email content as HTML
[**get_emails_paginated**](EmailControllerApi.md#get_emails_paginated) | **Get** /emails | Get all emails
[**get_raw_email_contents**](EmailControllerApi.md#get_raw_email_contents) | **Get** /emails/{emailId}/raw | Get raw email string
[**get_raw_email_json**](EmailControllerApi.md#get_raw_email_json) | **Get** /emails/{emailId}/raw/json | Get raw email in JSON
[**get_unread_email_count**](EmailControllerApi.md#get_unread_email_count) | **Get** /emails/unreadCount | Get unread email count
[**validate_email**](EmailControllerApi.md#validate_email) | **Post** /emails/{emailId}/validate | Validate email



## delete_all_emails

> delete_all_emails()
Delete all emails

Deletes all emails in your account. Be careful as emails cannot be recovered

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


## delete_email

> delete_email(email_id)
Delete an email

Deletes an email and removes it from the inbox. Deleted emails cannot be recovered.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_id** | [**String**](.md) | ID of email to delete | [required] |

### Return type

 (empty response body)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_attachment

> String download_attachment(attachment_id, email_id, api_key)
Get email attachment bytes. If you have trouble with byte responses try the `downloadAttachmentBase64` response endpoints.

Returns the specified attachment for a given email as a stream / array of bytes. You can find attachment ids in email responses endpoint responses. The response type is application/octet-stream.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attachment_id** | **String** | ID of attachment | [required] |
**email_id** | [**String**](.md) | ID of email | [required] |
**api_key** | Option<**String**> | Can pass apiKey in url for this request if you wish to download the file in a browser. Content type will be set to original content type of the attachment file. This is so that browsers can download the file correctly. |  |

### Return type

**String**

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_attachment_base64

> crate::models::DownloadAttachmentDto download_attachment_base64(attachment_id, email_id)
Get email attachment as base64 encoded string (alternative to binary responses)

Returns the specified attachment for a given email as a base 64 encoded string. The response type is application/json. This method is similar to the `downloadAttachment` method but allows some clients to get around issues with binary responses.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attachment_id** | **String** | ID of attachment | [required] |
**email_id** | [**String**](.md) | ID of email | [required] |

### Return type

[**crate::models::DownloadAttachmentDto**](DownloadAttachmentDto.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## forward_email

> forward_email(email_id, forward_email_options)
Forward email

Forward an existing email to new recipients.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_id** | [**String**](.md) | ID of email | [required] |
**forward_email_options** | [**ForwardEmailOptions**](ForwardEmailOptions.md) | forwardEmailOptions | [required] |

### Return type

 (empty response body)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_attachment_meta_data

> crate::models::AttachmentMetaData get_attachment_meta_data(attachment_id, email_id)
Get email attachment metadata

Returns the metadata such as name and content-type for a given attachment and email.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attachment_id** | **String** | ID of attachment | [required] |
**email_id** | [**String**](.md) | ID of email | [required] |

### Return type

[**crate::models::AttachmentMetaData**](AttachmentMetaData.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_attachments

> Vec<crate::models::AttachmentMetaData> get_attachments(email_id)
Get all email attachment metadata

Returns an array of attachment metadata such as name and content-type for a given email if present.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_id** | [**String**](.md) | ID of email | [required] |

### Return type

[**Vec<crate::models::AttachmentMetaData>**](AttachmentMetaData.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_email

> crate::models::Email get_email(email_id, decode)
Get email content

Returns a email summary object with headers and content. To retrieve the raw unparsed email use the getRawEmail endpoints

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_id** | [**String**](.md) | emailId | [required] |
**decode** | Option<**bool**> | Decode email body quoted-printable encoding to plain text. SMTP servers often encode text using quoted-printable format (for instance `=D7`). This can be a pain for testing |  |[default to false]

### Return type

[**crate::models::Email**](Email.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_email_content_match

> crate::models::EmailContentMatchResult get_email_content_match(email_id, content_match_options)
Get email content regex pattern match results. Runs regex against email body and returns match groups.

Return the matches for a given Java style regex pattern. Do not include the typical `/` at start or end of regex in some languages. Given an example `your code is: 12345` the pattern to extract match looks like `code is: (\\d{6})`. This will return an array of matches with the first matching the entire pattern and the subsequent matching the groups: `['code is: 123456', '123456']` See https://docs.oracle.com/javase/8/docs/api/java/util/regex/Pattern.html for more information of available patterns. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_id** | [**String**](.md) | ID of email to match against | [required] |
**content_match_options** | [**ContentMatchOptions**](ContentMatchOptions.md) | contentMatchOptions | [required] |

### Return type

[**crate::models::EmailContentMatchResult**](EmailContentMatchResult.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_email_html

> String get_email_html(email_id, decode)
Get email content as HTML

Retrieve email content as HTML response for viewing in browsers. Decodes quoted-printable entities and converts charset to UTF-8. Pass your API KEY as a request parameter when viewing in a browser: `?apiKey=xxx`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_id** | [**String**](.md) | emailId | [required] |
**decode** | Option<**bool**> | decode |  |[default to false]

### Return type

**String**

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_emails_paginated

> crate::models::PageEmailProjection get_emails_paginated(inbox_id, page, size, sort, unread_only)
Get all emails

By default returns all emails across all inboxes sorted by ascending created at date. Responses are paginated. You can restrict results to a list of inbox IDs. You can also filter out read messages

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inbox_id** | Option<[**Vec<String>**](String.md)> | Optional inbox ids to filter by. Can be repeated. By default will use all inboxes belonging to your account. |  |
**page** | Option<**i32**> | Optional page index in email list pagination |  |[default to 0]
**size** | Option<**i32**> | Optional page size in email list pagination. Maximum size is 100. Use page index and sort to page through larger results |  |[default to 20]
**sort** | Option<**String**> | Optional createdAt sort direction ASC or DESC |  |[default to ASC]
**unread_only** | Option<**bool**> | Optional filter for unread emails only. All emails are considered unread until they are viewed in the dashboard or requested directly |  |[default to false]

### Return type

[**crate::models::PageEmailProjection**](PageEmailProjection.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_raw_email_contents

> String get_raw_email_contents(email_id)
Get raw email string

Returns a raw, unparsed, and unprocessed email. If your client has issues processing the response it is likely due to the response content-type which is text/plain. If you need a JSON response content-type use the getRawEmailJson endpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_id** | [**String**](.md) | ID of email | [required] |

### Return type

**String**

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_raw_email_json

> crate::models::RawEmailJson get_raw_email_json(email_id)
Get raw email in JSON

Returns a raw, unparsed, and unprocessed email wrapped in a JSON response object for easier handling when compared with the getRawEmail text/plain response

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_id** | [**String**](.md) | ID of email | [required] |

### Return type

[**crate::models::RawEmailJson**](RawEmailJson.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_unread_email_count

> crate::models::UnreadCount get_unread_email_count()
Get unread email count

Get number of emails unread

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::UnreadCount**](UnreadCount.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_email

> crate::models::ValidationDto validate_email(email_id)
Validate email

Validate the HTML content of email if HTML is found. Considered valid if no HTML.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_id** | [**String**](.md) | ID of email | [required] |

### Return type

[**crate::models::ValidationDto**](ValidationDto.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

