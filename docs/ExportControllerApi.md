# \ExportControllerApi

All URIs are relative to *https://api.mailslurp.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**export_entities**](ExportControllerApi#export_entities) | **get** /export | Export inboxes link callable via browser
[**get_export_link**](ExportControllerApi#get_export_link) | **post** /export | Get export link



## export_entities

> String export_entities(api_key, export_type, output_format, created_earliest_time, created_oldest_time, exclude_previously_exported, filter, list_separator_token)
Export inboxes link callable via browser

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key** | **String** | apiKey | [required] |
**export_type** | **String** | exportType | [required] |
**output_format** | **String** | outputFormat | [required] |
**created_earliest_time** | Option<**String**> | createdEarliestTime |  |
**created_oldest_time** | Option<**String**> | createdOldestTime |  |
**exclude_previously_exported** | Option<**bool**> | excludePreviouslyExported |  |
**filter** | Option<**String**> | filter |  |
**list_separator_token** | Option<**String**> | listSeparatorToken |  |

### Return type

**String**

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_export_link

> crate::models::ExportLink get_export_link(export_type, export_options, api_key)
Get export link

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**export_type** | **String** | exportType | [required] |
**export_options** | [**ExportOptions**](ExportOptions) | exportOptions | [required] |
**api_key** | Option<**String**> | apiKey |  |

### Return type

[**crate::models::ExportLink**](ExportLink)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)

