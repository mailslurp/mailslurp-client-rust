# \InboxRulesetControllerApi

All URIs are relative to *https://api.mailslurp.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_new_inbox_ruleset**](InboxRulesetControllerApi#create_new_inbox_ruleset) | **post** /rulesets | Create an inbox ruleset
[**delete_inbox_ruleset**](InboxRulesetControllerApi#delete_inbox_ruleset) | **delete** /rulesets/{id} | Delete an inbox ruleset
[**delete_inbox_rulesets**](InboxRulesetControllerApi#delete_inbox_rulesets) | **delete** /rulesets | Delete inbox rulesets
[**get_inbox_ruleset**](InboxRulesetControllerApi#get_inbox_ruleset) | **get** /rulesets/{id} | Get an inbox ruleset
[**get_inbox_rulesets**](InboxRulesetControllerApi#get_inbox_rulesets) | **get** /rulesets | List inbox rulesets



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

> crate::models::PageInboxRulesetProjection get_inbox_rulesets(inbox_id, page, size, sort)
List inbox rulesets

List all rulesets attached to an inbox

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inbox_id** | Option<[**String**]()> | Optional inbox id to get rulesets from |  |
**page** | Option<**i32**> | Optional page index in inbox ruleset list pagination |  |[default to 0]
**size** | Option<**i32**> | Optional page size in inbox ruleset list pagination |  |[default to 20]
**sort** | Option<**String**> | Optional createdAt sort direction ASC or DESC |  |[default to ASC]

### Return type

[**crate::models::PageInboxRulesetProjection**](PageInboxRulesetProjection)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)

