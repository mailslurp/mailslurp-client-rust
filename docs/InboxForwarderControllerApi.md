# \InboxForwarderControllerApi

All URIs are relative to *https://api.mailslurp.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_new_inbox_forwarder**](InboxForwarderControllerApi#create_new_inbox_forwarder) | **post** /forwarders | Create an inbox forwarder
[**delete_inbox_forwarder**](InboxForwarderControllerApi#delete_inbox_forwarder) | **delete** /forwarders/{id} | Delete an inbox forwarder
[**delete_inbox_forwarders**](InboxForwarderControllerApi#delete_inbox_forwarders) | **delete** /forwarders | Delete inbox forwarders
[**get_inbox_forwarder**](InboxForwarderControllerApi#get_inbox_forwarder) | **get** /forwarders/{id} | Get an inbox forwarder
[**get_inbox_forwarders**](InboxForwarderControllerApi#get_inbox_forwarders) | **get** /forwarders | List inbox forwarders
[**test_inbox_forwarder**](InboxForwarderControllerApi#test_inbox_forwarder) | **post** /forwarders/{id}/test | Test an inbox forwarder
[**test_inbox_forwarders_for_inbox**](InboxForwarderControllerApi#test_inbox_forwarders_for_inbox) | **put** /forwarders | Test inbox forwarders for inbox
[**test_new_inbox_forwarder**](InboxForwarderControllerApi#test_new_inbox_forwarder) | **patch** /forwarders | Test new inbox forwarder



## create_new_inbox_forwarder

> crate::models::InboxForwarderDto create_new_inbox_forwarder(create_inbox_forwarder_options, inbox_id)
Create an inbox forwarder

Create a new inbox rule for forwarding, blocking, and allowing emails when sending and receiving

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_inbox_forwarder_options** | [**CreateInboxForwarderOptions**](CreateInboxForwarderOptions) | createInboxForwarderOptions | [required] |
**inbox_id** | Option<[**String**]()> | Inbox id to attach forwarder to |  |

### Return type

[**crate::models::InboxForwarderDto**](InboxForwarderDto)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## delete_inbox_forwarder

> delete_inbox_forwarder(id)
Delete an inbox forwarder

Delete inbox forwarder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**]() | ID of inbox forwarder | [required] |

### Return type

 (empty response body)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## delete_inbox_forwarders

> delete_inbox_forwarders(inbox_id)
Delete inbox forwarders

Delete inbox forwarders. Accepts optional inboxId filter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inbox_id** | Option<[**String**]()> | Optional inbox id to attach forwarder to |  |

### Return type

 (empty response body)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_inbox_forwarder

> crate::models::InboxForwarderDto get_inbox_forwarder(id)
Get an inbox forwarder

Get inbox ruleset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**]() | ID of inbox forwarder | [required] |

### Return type

[**crate::models::InboxForwarderDto**](InboxForwarderDto)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_inbox_forwarders

> crate::models::PageInboxForwarderDto get_inbox_forwarders(inbox_id, page, search_filter, size, sort)
List inbox forwarders

List all forwarders attached to an inbox

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inbox_id** | Option<[**String**]()> | Optional inbox id to get forwarders from |  |
**page** | Option<**i32**> | Optional page index in inbox forwarder list pagination |  |[default to 0]
**search_filter** | Option<**String**> | Optional search filter |  |
**size** | Option<**i32**> | Optional page size in inbox forwarder list pagination |  |[default to 20]
**sort** | Option<**String**> | Optional createdAt sort direction ASC or DESC |  |[default to ASC]

### Return type

[**crate::models::PageInboxForwarderDto**](PageInboxForwarderDto)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## test_inbox_forwarder

> crate::models::InboxForwarderTestResult test_inbox_forwarder(id, inbox_forwarder_test_options)
Test an inbox forwarder

Test an inbox forwarder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**]() | ID of inbox forwarder | [required] |
**inbox_forwarder_test_options** | [**InboxForwarderTestOptions**](InboxForwarderTestOptions) | inboxForwarderTestOptions | [required] |

### Return type

[**crate::models::InboxForwarderTestResult**](InboxForwarderTestResult)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## test_inbox_forwarders_for_inbox

> crate::models::InboxForwarderTestResult test_inbox_forwarders_for_inbox(inbox_id, inbox_forwarder_test_options)
Test inbox forwarders for inbox

Test inbox forwarders for inbox

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inbox_id** | [**String**]() | ID of inbox | [required] |
**inbox_forwarder_test_options** | [**InboxForwarderTestOptions**](InboxForwarderTestOptions) | inboxForwarderTestOptions | [required] |

### Return type

[**crate::models::InboxForwarderTestResult**](InboxForwarderTestResult)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## test_new_inbox_forwarder

> crate::models::InboxForwarderTestResult test_new_inbox_forwarder(test_new_inbox_forwarder_options)
Test new inbox forwarder

Test new inbox forwarder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**test_new_inbox_forwarder_options** | [**TestNewInboxForwarderOptions**](TestNewInboxForwarderOptions) | testNewInboxForwarderOptions | [required] |

### Return type

[**crate::models::InboxForwarderTestResult**](InboxForwarderTestResult)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)

