# \WaitForControllerApi

All URIs are relative to *https://api.mailslurp.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**wait_for**](WaitForControllerApi#wait_for) | **post** /waitFor | Wait for an email to match the provided filter conditions such as subject contains keyword.
[**wait_for_email_count**](WaitForControllerApi#wait_for_email_count) | **get** /waitForEmailCount | Wait for and return count number of emails. Hold connection until inbox count matches expected or timeout occurs
[**wait_for_latest_email**](WaitForControllerApi#wait_for_latest_email) | **get** /waitForLatestEmail | Fetch inbox's latest email or if empty wait for an email to arrive
[**wait_for_matching_email**](WaitForControllerApi#wait_for_matching_email) | **post** /waitForMatchingEmails | Wait or return list of emails that match simple matching patterns
[**wait_for_matching_first_email**](WaitForControllerApi#wait_for_matching_first_email) | **post** /waitForMatchingFirstEmail | Wait for or return the first email that matches provided MatchOptions array
[**wait_for_nth_email**](WaitForControllerApi#wait_for_nth_email) | **get** /waitForNthEmail | Wait for or fetch the email with a given index in the inbox specified. If index doesn't exist waits for it to exist or timeout to occur.



## wait_for

> Vec<crate::models::EmailPreview> wait_for(wait_for_conditions)
Wait for an email to match the provided filter conditions such as subject contains keyword.

Generic waitFor method that will wait until an inbox meets given conditions or return immediately if already met

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**wait_for_conditions** | Option<[**WaitForConditions**](WaitForConditions)> | Conditions to apply to emails that you are waiting for |  |

### Return type

[**Vec<crate::models::EmailPreview>**](EmailPreview)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## wait_for_email_count

> Vec<crate::models::EmailPreview> wait_for_email_count(count, delay, inbox_id, since, sort, timeout, unread_only)
Wait for and return count number of emails. Hold connection until inbox count matches expected or timeout occurs

If inbox contains count or more emails at time of request then return count worth of emails. If not wait until the count is reached and return those or return an error if timeout is exceeded.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**count** | Option<**i32**> | Number of emails to wait for. Must be greater that 1 |  |
**delay** | Option<**i64**> | Max milliseconds delay between calls |  |
**inbox_id** | Option<[**String**]()> | Id of the inbox we are fetching emails from |  |
**since** | Option<**String**> | Filter for emails that were received after the given timestamp |  |
**sort** | Option<**String**> | Sort direction |  |
**timeout** | Option<**i64**> | Max milliseconds to wait |  |
**unread_only** | Option<**bool**> | Optional filter for unread only |  |[default to false]

### Return type

[**Vec<crate::models::EmailPreview>**](EmailPreview)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## wait_for_latest_email

> crate::models::Email wait_for_latest_email(delay, inbox_id, since, sort, timeout, unread_only)
Fetch inbox's latest email or if empty wait for an email to arrive

Will return either the last received email or wait for an email to arrive and return that. If you need to wait for an email for a non-empty inbox set `unreadOnly=true` or see the other receive methods such as `waitForNthEmail` or `waitForEmailCount`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delay** | Option<**i64**> | Max milliseconds delay between calls |  |
**inbox_id** | Option<[**String**]()> | Id of the inbox we are fetching emails from |  |
**since** | Option<**String**> | Filter for emails that were received after the given timestamp |  |
**sort** | Option<**String**> | Sort direction |  |
**timeout** | Option<**i64**> | Max milliseconds to wait |  |
**unread_only** | Option<**bool**> | Optional filter for unread only. |  |[default to false]

### Return type

[**crate::models::Email**](Email)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## wait_for_matching_email

> Vec<crate::models::EmailPreview> wait_for_matching_email(match_options, count, delay, inbox_id, since, sort, timeout, unread_only)
Wait or return list of emails that match simple matching patterns

Perform a search of emails in an inbox with the given patterns. If results match expected count then return or else retry the search until results are found or timeout is reached. Match options allow simple CONTAINS or EQUALS filtering on SUBJECT, TO, BCC, CC, and FROM. See the `MatchOptions` object for options. An example payload is `{ matches: [{field: 'SUBJECT',should:'CONTAIN',value:'needle'}] }`. You can use an array of matches and they will be applied sequentially to filter out emails. If you want to perform matches and extractions of content using Regex patterns see the EmailController `getEmailContentMatch` method.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**match_options** | [**MatchOptions**](MatchOptions) | matchOptions | [required] |
**count** | Option<**i32**> | Number of emails to wait for. Must be greater or equal to 1 |  |
**delay** | Option<**i64**> | Max milliseconds delay between calls |  |
**inbox_id** | Option<[**String**]()> | Id of the inbox we are fetching emails from |  |
**since** | Option<**String**> | Filter for emails that were received after the given timestamp |  |
**sort** | Option<**String**> | Sort direction |  |
**timeout** | Option<**i64**> | Max milliseconds to wait |  |
**unread_only** | Option<**bool**> | Optional filter for unread only |  |[default to false]

### Return type

[**Vec<crate::models::EmailPreview>**](EmailPreview)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## wait_for_matching_first_email

> crate::models::Email wait_for_matching_first_email(match_options, delay, inbox_id, since, sort, timeout, unread_only)
Wait for or return the first email that matches provided MatchOptions array

Perform a search of emails in an inbox with the given patterns. If a result if found then return or else retry the search until a result is found or timeout is reached. Match options allow simple CONTAINS or EQUALS filtering on SUBJECT, TO, BCC, CC, and FROM. See the `MatchOptions` object for options. An example payload is `{ matches: [{field: 'SUBJECT',should:'CONTAIN',value:'needle'}] }`. You can use an array of matches and they will be applied sequentially to filter out emails. If you want to perform matches and extractions of content using Regex patterns see the EmailController `getEmailContentMatch` method.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**match_options** | [**MatchOptions**](MatchOptions) | matchOptions | [required] |
**delay** | Option<**i64**> | Max milliseconds delay between calls |  |
**inbox_id** | Option<[**String**]()> | Id of the inbox we are matching an email for |  |
**since** | Option<**String**> | Filter for emails that were received after the given timestamp |  |
**sort** | Option<**String**> | Sort direction |  |
**timeout** | Option<**i64**> | Max milliseconds to wait |  |
**unread_only** | Option<**bool**> | Optional filter for unread only |  |[default to false]

### Return type

[**crate::models::Email**](Email)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## wait_for_nth_email

> crate::models::Email wait_for_nth_email(delay, inbox_id, index, since, sort, timeout, unread_only)
Wait for or fetch the email with a given index in the inbox specified. If index doesn't exist waits for it to exist or timeout to occur.

If nth email is already present in inbox then return it. If not hold the connection open until timeout expires or the nth email is received and returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delay** | Option<**i64**> | Max milliseconds delay between calls |  |
**inbox_id** | Option<[**String**]()> | Id of the inbox you are fetching emails from |  |
**index** | Option<**i32**> | Zero based index of the email to wait for. If an inbox has 1 email already and you want to wait for the 2nd email pass index=1 |  |[default to 0]
**since** | Option<**String**> | Filter for emails that were received after the given timestamp |  |
**sort** | Option<**String**> | Sort direction |  |
**timeout** | Option<**i64**> | Max milliseconds to wait for the nth email if not already present |  |
**unread_only** | Option<**bool**> | Optional filter for unread only |  |[default to false]

### Return type

[**crate::models::Email**](Email)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)

