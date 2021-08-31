# \BulkActionsControllerApi

All URIs are relative to *https://api.mailslurp.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bulk_create_inboxes**](BulkActionsControllerApi#bulk_create_inboxes) | **post** /bulk/inboxes | Bulk create Inboxes (email addresses)
[**bulk_delete_inboxes**](BulkActionsControllerApi#bulk_delete_inboxes) | **delete** /bulk/inboxes | Bulk Delete Inboxes
[**bulk_send_emails**](BulkActionsControllerApi#bulk_send_emails) | **post** /bulk/send | Bulk Send Emails



## bulk_create_inboxes

> Vec<crate::models::Inbox> bulk_create_inboxes(count)
Bulk create Inboxes (email addresses)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**count** | **i32** | Number of inboxes to be created in bulk | [required] |

### Return type

[**Vec<crate::models::Inbox>**](Inbox)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## bulk_delete_inboxes

> bulk_delete_inboxes(ids)
Bulk Delete Inboxes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String) | ids | [required] |

### Return type

 (empty response body)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## bulk_send_emails

> bulk_send_emails(bulk_send_email_options)
Bulk Send Emails

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_send_email_options** | [**BulkSendEmailOptions**](BulkSendEmailOptions) | bulkSendEmailOptions | [required] |

### Return type

 (empty response body)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)

