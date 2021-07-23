# \GroupControllerApi

All URIs are relative to *https://api.mailslurp.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_contacts_to_group**](GroupControllerApi#add_contacts_to_group) | **put** /groups/{groupId}/contacts | Add contacts to a group
[**create_group**](GroupControllerApi#create_group) | **post** /groups | Create a group
[**delete_group**](GroupControllerApi#delete_group) | **delete** /groups/{groupId} | Delete group
[**get_all_groups**](GroupControllerApi#get_all_groups) | **get** /groups/paginated | Get all Contact Groups in paginated format
[**get_group**](GroupControllerApi#get_group) | **get** /groups/{groupId} | Get group
[**get_group_with_contacts**](GroupControllerApi#get_group_with_contacts) | **get** /groups/{groupId}/contacts | Get group and contacts belonging to it
[**get_group_with_contacts_paginated**](GroupControllerApi#get_group_with_contacts_paginated) | **get** /groups/{groupId}/contacts-paginated | Get group and paginated contacts belonging to it
[**get_groups**](GroupControllerApi#get_groups) | **get** /groups | Get all groups
[**remove_contacts_from_group**](GroupControllerApi#remove_contacts_from_group) | **delete** /groups/{groupId}/contacts | Remove contacts from a group



## add_contacts_to_group

> crate::models::GroupContactsDto add_contacts_to_group(group_id, update_group_contacts_option)
Add contacts to a group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | [**String**]() | groupId | [required] |
**update_group_contacts_option** | [**UpdateGroupContacts**](UpdateGroupContacts) | updateGroupContactsOption | [required] |

### Return type

[**crate::models::GroupContactsDto**](GroupContactsDto)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## create_group

> crate::models::GroupDto create_group(create_group_options)
Create a group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_group_options** | [**CreateGroupOptions**](CreateGroupOptions) | createGroupOptions | [required] |

### Return type

[**crate::models::GroupDto**](GroupDto)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## delete_group

> delete_group(group_id)
Delete group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | [**String**]() | groupId | [required] |

### Return type

 (empty response body)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_all_groups

> crate::models::PageGroupProjection get_all_groups(page, size, sort)
Get all Contact Groups in paginated format

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | Optional page index in list pagination |  |[default to 0]
**size** | Option<**i32**> | Optional page size in list pagination |  |[default to 20]
**sort** | Option<**String**> | Optional createdAt sort direction ASC or DESC |  |[default to ASC]

### Return type

[**crate::models::PageGroupProjection**](PageGroupProjection)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_group

> crate::models::GroupDto get_group(group_id)
Get group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | [**String**]() | groupId | [required] |

### Return type

[**crate::models::GroupDto**](GroupDto)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_group_with_contacts

> crate::models::GroupContactsDto get_group_with_contacts(group_id)
Get group and contacts belonging to it

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | [**String**]() | groupId | [required] |

### Return type

[**crate::models::GroupContactsDto**](GroupContactsDto)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_group_with_contacts_paginated

> crate::models::PageContactProjection get_group_with_contacts_paginated(group_id, page, size, sort)
Get group and paginated contacts belonging to it

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | [**String**]() | groupId | [required] |
**page** | Option<**i32**> | Optional page index in group contact pagination |  |[default to 0]
**size** | Option<**i32**> | Optional page size in group contact pagination |  |[default to 20]
**sort** | Option<**String**> | Optional createdAt sort direction ASC or DESC |  |[default to ASC]

### Return type

[**crate::models::PageContactProjection**](PageContactProjection)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_groups

> Vec<crate::models::GroupProjection> get_groups()
Get all groups

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::GroupProjection>**](GroupProjection)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## remove_contacts_from_group

> crate::models::GroupContactsDto remove_contacts_from_group(group_id, update_group_contacts_option)
Remove contacts from a group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | [**String**]() | groupId | [required] |
**update_group_contacts_option** | [**UpdateGroupContacts**](UpdateGroupContacts) | updateGroupContactsOption | [required] |

### Return type

[**crate::models::GroupContactsDto**](GroupContactsDto)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)

