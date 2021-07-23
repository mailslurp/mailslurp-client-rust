# \InboxRulesetControllerApi

All URIs are relative to *https://api.mailslurp.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_new_inbox_ruleset**](InboxRulesetControllerApi#create_new_inbox_ruleset) | **post** /rulesets | Create an inbox ruleset
[**delete_inbox_ruleset**](InboxRulesetControllerApi#delete_inbox_ruleset) | **delete** /rulesets/{id} | Delete an inbox ruleset
[**delete_inbox_rulesets**](InboxRulesetControllerApi#delete_inbox_rulesets) | **delete** /rulesets | Delete inbox rulesets
[**get_inbox_ruleset**](InboxRulesetControllerApi#get_inbox_ruleset) | **get** /rulesets/{id} | Get an inbox ruleset
[**get_inbox_rulesets**](InboxRulesetControllerApi#get_inbox_rulesets) | **get** /rulesets | List inbox rulesets
[**test_inbox_ruleset**](InboxRulesetControllerApi#test_inbox_ruleset) | **post** /rulesets/{id}/test | Test an inbox ruleset
[**test_inbox_rulesets_for_inbox**](InboxRulesetControllerApi#test_inbox_rulesets_for_inbox) | **put** /rulesets | Test inbox rulesets for inbox
[**test_new_inbox_ruleset**](InboxRulesetControllerApi#test_new_inbox_ruleset) | **patch** /rulesets | Test new inbox ruleset



## create_new_inbox_ruleset

> crate::models::InboxRulesetDto create_new_inbox_ruleset(create_inbox_ruleset_options, inbox_id)
Create an inbox ruleset

Create a new inbox rule for forwarding, blocking, and allowing emails when sending and receiving

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_inbox_ruleset_options** | [**CreateInboxRulesetOptions**](CreateInboxRulesetOptions) | createInboxRulesetOptions | [required] |
**inbox_id** | Option<[**String**]()> | Inbox id to attach ruleset to |  |

### Return type

[**crate::models::InboxRulesetDto**](InboxRulesetDto)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## delete_inbox_ruleset

> delete_inbox_ruleset(id)
Delete an inbox ruleset

Delete inbox ruleset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**]() | ID of inbox ruleset | [required] |

### Return type

 (empty response body)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## delete_inbox_rulesets

> delete_inbox_rulesets(inbox_id)
Delete inbox rulesets

Delete inbox rulesets. Accepts optional inboxId filter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inbox_id** | Option<[**String**]()> | Optional inbox id to attach ruleset to |  |

### Return type

 (empty response body)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_inbox_ruleset

> crate::models::InboxRulesetDto get_inbox_ruleset(id)
Get an inbox ruleset

Get inbox ruleset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**]() | ID of inbox ruleset | [required] |

### Return type

[**crate::models::InboxRulesetDto**](InboxRulesetDto)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_inbox_rulesets

> crate::models::PageInboxRulesetDto get_inbox_rulesets(inbox_id, page, search_filter, size, sort)
List inbox rulesets

List all rulesets attached to an inbox

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inbox_id** | Option<[**String**]()> | Optional inbox id to get rulesets from |  |
**page** | Option<**i32**> | Optional page index in inbox ruleset list pagination |  |[default to 0]
**search_filter** | Option<**String**> | Optional search filter |  |
**size** | Option<**i32**> | Optional page size in inbox ruleset list pagination |  |[default to 20]
**sort** | Option<**String**> | Optional createdAt sort direction ASC or DESC |  |[default to ASC]

### Return type

[**crate::models::PageInboxRulesetDto**](PageInboxRulesetDto)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## test_inbox_ruleset

> crate::models::InboxRulesetTestResult test_inbox_ruleset(id, inbox_ruleset_test_options)
Test an inbox ruleset

Test an inbox ruleset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**]() | ID of inbox ruleset | [required] |
**inbox_ruleset_test_options** | [**InboxRulesetTestOptions**](InboxRulesetTestOptions) | inboxRulesetTestOptions | [required] |

### Return type

[**crate::models::InboxRulesetTestResult**](InboxRulesetTestResult)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## test_inbox_rulesets_for_inbox

> crate::models::InboxRulesetTestResult test_inbox_rulesets_for_inbox(inbox_id, inbox_ruleset_test_options)
Test inbox rulesets for inbox

Test inbox rulesets for inbox

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inbox_id** | [**String**]() | ID of inbox | [required] |
**inbox_ruleset_test_options** | [**InboxRulesetTestOptions**](InboxRulesetTestOptions) | inboxRulesetTestOptions | [required] |

### Return type

[**crate::models::InboxRulesetTestResult**](InboxRulesetTestResult)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## test_new_inbox_ruleset

> crate::models::InboxRulesetTestResult test_new_inbox_ruleset(test_new_inbox_ruleset_options)
Test new inbox ruleset

Test new inbox ruleset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**test_new_inbox_ruleset_options** | [**TestNewInboxRulesetOptions**](TestNewInboxRulesetOptions) | testNewInboxRulesetOptions | [required] |

### Return type

[**crate::models::InboxRulesetTestResult**](InboxRulesetTestResult)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)

