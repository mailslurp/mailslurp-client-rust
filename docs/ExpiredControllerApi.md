# \ExpiredControllerApi

All URIs are relative to *https://api.mailslurp.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_expiration_defaults**](ExpiredControllerApi.md#get_expiration_defaults) | **get** /expired/defaults | Get default expiration settings
[**get_expired_inbox_by_inbox_id**](ExpiredControllerApi.md#get_expired_inbox_by_inbox_id) | **get** /expired/inbox/{inboxId} | Get expired inbox record for a previously existing inbox
[**get_expired_inbox_record**](ExpiredControllerApi.md#get_expired_inbox_record) | **get** /expired/{expiredId} | Get an expired inbox record
[**get_expired_inboxes**](ExpiredControllerApi.md#get_expired_inboxes) | **get** /expired | List records of expired inboxes



## get_expiration_defaults

> crate::models::ExpirationDefaults get_expiration_defaults()
Get default expiration settings

Return default times used for inbox expiration

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ExpirationDefaults**](ExpirationDefaults.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_expired_inbox_by_inbox_id

> crate::models::ExpiredInboxDto get_expired_inbox_by_inbox_id(inbox_id)
Get expired inbox record for a previously existing inbox

Use the inboxId to return an ExpiredInboxRecord if an inbox has expired. Inboxes expire and are disabled if an expiration date is set or plan requires. Returns 404 if no expired inbox is found for the inboxId

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inbox_id** | [**String**](.md) | ID of inbox you want to retrieve (not the inbox ID) | [required] |

### Return type

[**crate::models::ExpiredInboxDto**](ExpiredInboxDto.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_expired_inbox_record

> crate::models::ExpiredInboxDto get_expired_inbox_record(expired_id)
Get an expired inbox record

Inboxes created with an expiration date will expire after the given date and be moved to an ExpiredInbox entity. You can still read emails in the inbox but it can no longer send or receive emails. Fetch the expired inboxes to view the old inboxes properties

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**expired_id** | [**String**](.md) | ID of the ExpiredInboxRecord you want to retrieve. This is different from the ID of the inbox you are interested in. See other methods for getting ExpiredInboxRecord for an inbox inboxId) | [required] |

### Return type

[**crate::models::ExpiredInboxDto**](ExpiredInboxDto.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_expired_inboxes

> crate::models::PageExpiredInboxRecordProjection get_expired_inboxes(page, size, sort)
List records of expired inboxes

Inboxes created with an expiration date will expire after the given date. An ExpiredInboxRecord is created that records the inboxes old ID and email address. You can still read emails in the inbox (using the inboxes old ID) but the email address associated with the inbox can no longer send or receive emails. Fetch expired inbox records to view the old inboxes properties

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Optional page index in inbox sent email list pagination |  |[default to 0]
**size** | Option<**i32**> | Optional page size in inbox sent email list pagination |  |[default to 20]
**sort** | Option<**String**> | Optional createdAt sort direction ASC or DESC |  |[default to ASC]

### Return type

[**crate::models::PageExpiredInboxRecordProjection**](PageExpiredInboxRecordProjection.md)

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

