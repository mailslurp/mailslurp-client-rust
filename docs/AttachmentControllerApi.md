# \AttachmentControllerApi

All URIs are relative to *https://api.mailslurp.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**upload_attachment**](AttachmentControllerApi.md#upload_attachment) | **Post** /attachments | Upload an attachment for sending using base64 file encoding
[**upload_attachment_bytes**](AttachmentControllerApi.md#upload_attachment_bytes) | **Post** /attachments/bytes | Upload an attachment for sending using file byte stream input octet stream
[**upload_multipart_form**](AttachmentControllerApi.md#upload_multipart_form) | **Post** /attachments/multipart | Upload an attachment for sending using Multipart Form



## upload_attachment

> Vec<String> upload_attachment(upload_options)
Upload an attachment for sending using base64 file encoding

When sending emails with attachments first upload each attachment with this endpoint. Record the returned attachment IDs. Then use these attachment IDs in the SendEmailOptions when sending an email. This means that attachments can easily be reused.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**upload_options** | [**UploadAttachmentOptions**](UploadAttachmentOptions.md) | uploadOptions | [required] |

### Return type

**Vec<String>**

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_attachment_bytes

> Vec<String> upload_attachment_bytes(filename)
Upload an attachment for sending using file byte stream input octet stream

When sending emails with attachments first upload each attachment with this endpoint. Record the returned attachment IDs. Then use these attachment IDs in the SendEmailOptions when sending an email. This means that attachments can easily be reused.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filename** | Option<**String**> | Optional filename to save upload with |  |

### Return type

**Vec<String>**

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: application/octet-stream
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_multipart_form

> Vec<String> upload_multipart_form(file, content_type, filename, x_filename)
Upload an attachment for sending using Multipart Form

When sending emails with attachments first upload each attachment with this endpoint. Record the returned attachment IDs. Then use these attachment IDs in the SendEmailOptions when sending an email. This means that attachments can easily be reused.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file** | **std::path::PathBuf** | file | [required] |
**content_type** | Option<**String**> | contentType |  |
**filename** | Option<**String**> | filename |  |
**x_filename** | Option<**String**> | x-filename |  |

### Return type

**Vec<String>**

### Authorization

[API_KEY](../README.md#API_KEY)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

