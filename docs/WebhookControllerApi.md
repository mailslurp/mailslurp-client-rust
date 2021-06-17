# \WebhookControllerApi

All URIs are relative to *https://api.mailslurp.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_webhook**](WebhookControllerApi#create_webhook) | **post** /inboxes/{inboxId}/webhooks | Attach a WebHook URL to an inbox
[**delete_webhook**](WebhookControllerApi#delete_webhook) | **delete** /inboxes/{inboxId}/webhooks/{webhookId} | Delete and disable a Webhook for an Inbox
[**get_all_webhook_results**](WebhookControllerApi#get_all_webhook_results) | **get** /webhooks/results | Get results for all webhooks
[**get_all_webhooks**](WebhookControllerApi#get_all_webhooks) | **get** /webhooks/paginated | List Webhooks Paginated
[**get_test_webhook_payload**](WebhookControllerApi#get_test_webhook_payload) | **get** /webhooks/test | Get test webhook payload example. Response content depends on eventName passed. Uses `EMAIL_RECEIVED` as default.
[**get_test_webhook_payload_new_attachment**](WebhookControllerApi#get_test_webhook_payload_new_attachment) | **get** /webhooks/test/new-attachment-payload | Get webhook test payload for new attachment event
[**get_test_webhook_payload_new_contact**](WebhookControllerApi#get_test_webhook_payload_new_contact) | **get** /webhooks/test/new-contact-payload | Get webhook test payload for new contact event
[**get_test_webhook_payload_new_email**](WebhookControllerApi#get_test_webhook_payload_new_email) | **get** /webhooks/test/new-email-payload | Get webhook test payload for new email event
[**get_webhook**](WebhookControllerApi#get_webhook) | **get** /webhooks/{webhookId} | Get a webhook for an Inbox
[**get_webhook_result**](WebhookControllerApi#get_webhook_result) | **get** /webhooks/results/{webhookResultId} | Get a webhook result for a webhook
[**get_webhook_results**](WebhookControllerApi#get_webhook_results) | **get** /webhooks/{webhookId}/results | Get a webhook results for a webhook
[**get_webhooks**](WebhookControllerApi#get_webhooks) | **get** /inboxes/{inboxId}/webhooks | Get all webhooks for an Inbox
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

> crate::models::PageWebhookResult get_all_webhook_results(page, size, sort)
Get results for all webhooks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Optional page index in list pagination |  |[default to 0]
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

> crate::models::PageWebhookProjection get_all_webhooks(page, size, sort)
List Webhooks Paginated

List webhooks in paginated form. Allows for page index, page size, and sort direction.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Optional page index in list pagination |  |[default to 0]
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

> crate::models::WebhookResultEntity get_webhook_result(webhook_result_id)
Get a webhook result for a webhook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_result_id** | [**String**]() | Webhook Result ID | [required] |

### Return type

[**crate::models::WebhookResultEntity**](WebhookResultEntity)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_webhook_results

> crate::models::PageWebhookResult get_webhook_results(webhook_id, page, size, sort)
Get a webhook results for a webhook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | [**String**]() | ID of webhook to get results for | [required] |
**page** | Option<**i32**> | Optional page index in list pagination |  |[default to 0]
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

