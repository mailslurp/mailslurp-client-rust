# \WebhookControllerApi

All URIs are relative to *https://api.mailslurp.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_webhook**](WebhookControllerApi#create_webhook) | **post** /inboxes/{inboxId}/webhooks | Attach a WebHook URL to an inbox
[**delete_webhook**](WebhookControllerApi#delete_webhook) | **delete** /inboxes/{inboxId}/webhooks/{webhookId} | Delete and disable a Webhook for an Inbox
[**get_all_webhook_results**](WebhookControllerApi#get_all_webhook_results) | **get** /webhooks/results | Get results for all webhooks
[**get_all_webhooks**](WebhookControllerApi#get_all_webhooks) | **get** /webhooks/paginated | List Webhooks Paginated
[**get_inbox_webhooks_paginated**](WebhookControllerApi#get_inbox_webhooks_paginated) | **get** /inboxes/{inboxId}/webhooks/paginated | Get paginated webhooks for an Inbox
[**get_json_schema_for_webhook_payload**](WebhookControllerApi#get_json_schema_for_webhook_payload) | **post** /webhooks/{webhookId}/schema | Get JSON Schema definition for webhook payload
[**get_test_webhook_payload**](WebhookControllerApi#get_test_webhook_payload) | **get** /webhooks/test | Get test webhook payload example. Response content depends on eventName passed. Uses `EMAIL_RECEIVED` as default.
[**get_test_webhook_payload_email_opened**](WebhookControllerApi#get_test_webhook_payload_email_opened) | **get** /webhooks/test/email-opened-payload | Get webhook test payload for email opened event
[**get_test_webhook_payload_email_read**](WebhookControllerApi#get_test_webhook_payload_email_read) | **get** /webhooks/test/email-read-payload | Get webhook test payload for email opened event
[**get_test_webhook_payload_for_webhook**](WebhookControllerApi#get_test_webhook_payload_for_webhook) | **post** /webhooks/{webhookId}/example | Get example payload for webhook
[**get_test_webhook_payload_new_attachment**](WebhookControllerApi#get_test_webhook_payload_new_attachment) | **get** /webhooks/test/new-attachment-payload | Get webhook test payload for new attachment event
[**get_test_webhook_payload_new_contact**](WebhookControllerApi#get_test_webhook_payload_new_contact) | **get** /webhooks/test/new-contact-payload | Get webhook test payload for new contact event
[**get_test_webhook_payload_new_email**](WebhookControllerApi#get_test_webhook_payload_new_email) | **get** /webhooks/test/new-email-payload | Get webhook test payload for new email event
[**get_webhook**](WebhookControllerApi#get_webhook) | **get** /webhooks/{webhookId} | Get a webhook for an Inbox
[**get_webhook_result**](WebhookControllerApi#get_webhook_result) | **get** /webhooks/results/{webhookResultId} | Get a webhook result for a webhook
[**get_webhook_results**](WebhookControllerApi#get_webhook_results) | **get** /webhooks/{webhookId}/results | Get a webhook results for a webhook
[**get_webhooks**](WebhookControllerApi#get_webhooks) | **get** /inboxes/{inboxId}/webhooks | Get all webhooks for an Inbox
[**redrive_webhook_result**](WebhookControllerApi#redrive_webhook_result) | **post** /webhooks/results/{webhookResultId}/redrive | Get a webhook result and try to resend the original webhook payload
[**send_test_data**](WebhookControllerApi#send_test_data) | **post** /webhooks/{webhookId}/test | Send webhook test data



## create_webhook

> crate::models::WebhookDto create_webhook(inbox_id, webhook_options)
Attach a WebHook URL to an inbox

Get notified whenever an inbox receives an email via a WebHook URL. An emailID will be posted to this URL every time an email is received for this inbox. The URL must be publicly reachable by the MailSlurp server. You can provide basicAuth values if you wish to secure this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inbox_id** | [**String**]() | inboxId | [required] |
**webhook_options** | [**CreateWebhookOptions**](CreateWebhookOptions) | webhookOptions | [required] |

### Return type

[**crate::models::WebhookDto**](WebhookDto)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## delete_webhook

> delete_webhook(inbox_id, webhook_id)
Delete and disable a Webhook for an Inbox

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inbox_id** | [**String**]() | inboxId | [required] |
**webhook_id** | [**String**]() | webhookId | [required] |

### Return type

 (empty response body)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_all_webhook_results

> crate::models::PageWebhookResult get_all_webhook_results(before, page, search_filter, since, size, sort)
Get results for all webhooks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**before** | Option<**String**> | Filter by created at before the given timestamp |  |
**page** | Option<**i32**> | Optional page index in list pagination |  |[default to 0]
**search_filter** | Option<**String**> | Optional search filter |  |
**since** | Option<**String**> | Filter by created at after the given timestamp |  |
**size** | Option<**i32**> | Optional page size in list pagination |  |[default to 20]
**sort** | Option<**String**> | Optional createdAt sort direction ASC or DESC |  |[default to ASC]

### Return type

[**crate::models::PageWebhookResult**](PageWebhookResult)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_all_webhooks

> crate::models::PageWebhookProjection get_all_webhooks(before, page, search_filter, since, size, sort)
List Webhooks Paginated

List webhooks in paginated form. Allows for page index, page size, and sort direction.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**before** | Option<**String**> | Filter by created at before the given timestamp |  |
**page** | Option<**i32**> | Optional page index in list pagination |  |[default to 0]
**search_filter** | Option<**String**> | Optional search filter |  |
**since** | Option<**String**> | Filter by created at after the given timestamp |  |
**size** | Option<**i32**> | Optional page size for paginated result list. |  |[default to 20]
**sort** | Option<**String**> | Optional createdAt sort direction ASC or DESC |  |[default to DESC]

### Return type

[**crate::models::PageWebhookProjection**](PageWebhookProjection)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_inbox_webhooks_paginated

> crate::models::PageWebhookProjection get_inbox_webhooks_paginated(inbox_id, before, page, search_filter, since, size, sort)
Get paginated webhooks for an Inbox

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inbox_id** | [**String**]() | inboxId | [required] |
**before** | Option<**String**> | Filter by created at before the given timestamp |  |
**page** | Option<**i32**> | Optional page index in list pagination |  |[default to 0]
**search_filter** | Option<**String**> | Optional search filter |  |
**since** | Option<**String**> | Filter by created at after the given timestamp |  |
**size** | Option<**i32**> | Optional page size in list pagination |  |[default to 20]
**sort** | Option<**String**> | Optional createdAt sort direction ASC or DESC |  |[default to ASC]

### Return type

[**crate::models::PageWebhookProjection**](PageWebhookProjection)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_json_schema_for_webhook_payload

> crate::models::JsonSchemaDto get_json_schema_for_webhook_payload(webhook_id)
Get JSON Schema definition for webhook payload

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | [**String**]() | webhookId | [required] |

### Return type

[**crate::models::JsonSchemaDto**](JSONSchemaDto)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_test_webhook_payload

> crate::models::AbstractWebhookPayload get_test_webhook_payload(event_name)
Get test webhook payload example. Response content depends on eventName passed. Uses `EMAIL_RECEIVED` as default.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_name** | Option<**String**> | eventName |  |

### Return type

[**crate::models::AbstractWebhookPayload**](AbstractWebhookPayload)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_test_webhook_payload_email_opened

> crate::models::WebhookEmailOpenedPayload get_test_webhook_payload_email_opened()
Get webhook test payload for email opened event

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::WebhookEmailOpenedPayload**](WebhookEmailOpenedPayload)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_test_webhook_payload_email_read

> crate::models::WebhookEmailReadPayload get_test_webhook_payload_email_read()
Get webhook test payload for email opened event

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::WebhookEmailReadPayload**](WebhookEmailReadPayload)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_test_webhook_payload_for_webhook

> crate::models::AbstractWebhookPayload get_test_webhook_payload_for_webhook(webhook_id)
Get example payload for webhook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | [**String**]() | webhookId | [required] |

### Return type

[**crate::models::AbstractWebhookPayload**](AbstractWebhookPayload)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_test_webhook_payload_new_attachment

> crate::models::WebhookNewAttachmentPayload get_test_webhook_payload_new_attachment()
Get webhook test payload for new attachment event

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::WebhookNewAttachmentPayload**](WebhookNewAttachmentPayload)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_test_webhook_payload_new_contact

> crate::models::WebhookNewContactPayload get_test_webhook_payload_new_contact()
Get webhook test payload for new contact event

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::WebhookNewContactPayload**](WebhookNewContactPayload)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_test_webhook_payload_new_email

> crate::models::WebhookNewEmailPayload get_test_webhook_payload_new_email()
Get webhook test payload for new email event

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::WebhookNewEmailPayload**](WebhookNewEmailPayload)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_webhook

> crate::models::WebhookDto get_webhook(webhook_id)
Get a webhook for an Inbox

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | [**String**]() | webhookId | [required] |

### Return type

[**crate::models::WebhookDto**](WebhookDto)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_webhook_result

> crate::models::WebhookResultDto get_webhook_result(webhook_result_id)
Get a webhook result for a webhook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_result_id** | [**String**]() | Webhook Result ID | [required] |

### Return type

[**crate::models::WebhookResultDto**](WebhookResultDto)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_webhook_results

> crate::models::PageWebhookResult get_webhook_results(webhook_id, before, page, search_filter, since, size, sort)
Get a webhook results for a webhook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | [**String**]() | ID of webhook to get results for | [required] |
**before** | Option<**String**> | Filter by created at before the given timestamp |  |
**page** | Option<**i32**> | Optional page index in list pagination |  |[default to 0]
**search_filter** | Option<**String**> | Optional search filter |  |
**since** | Option<**String**> | Filter by created at after the given timestamp |  |
**size** | Option<**i32**> | Optional page size in list pagination |  |[default to 20]
**sort** | Option<**String**> | Optional createdAt sort direction ASC or DESC |  |[default to ASC]

### Return type

[**crate::models::PageWebhookResult**](PageWebhookResult)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_webhooks

> Vec<crate::models::WebhookDto> get_webhooks(inbox_id)
Get all webhooks for an Inbox

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inbox_id** | [**String**]() | inboxId | [required] |

### Return type

[**Vec<crate::models::WebhookDto>**](WebhookDto)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## redrive_webhook_result

> crate::models::WebhookRedriveResult redrive_webhook_result(webhook_result_id)
Get a webhook result and try to resend the original webhook payload

Allows you to resend a webhook payload that was already sent. Webhooks that fail are retried automatically for 24 hours and then put in a dead letter queue. You can retry results manually using this method.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_result_id** | [**String**]() | Webhook Result ID | [required] |

### Return type

[**crate::models::WebhookRedriveResult**](WebhookRedriveResult)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## send_test_data

> crate::models::WebhookTestResult send_test_data(webhook_id)
Send webhook test data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | [**String**]() | webhookId | [required] |

### Return type

[**crate::models::WebhookTestResult**](WebhookTestResult)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)

