# \TrackingControllerApi

All URIs are relative to *https://api.mailslurp.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_tracking_pixel**](TrackingControllerApi#create_tracking_pixel) | **post** /tracking/pixels | Create tracking pixel
[**get_all_tracking_pixels**](TrackingControllerApi#get_all_tracking_pixels) | **get** /tracking/pixels | Get tracking pixels
[**get_tracking_pixel**](TrackingControllerApi#get_tracking_pixel) | **get** /tracking/pixels/{id} | Get pixel



## create_tracking_pixel

> crate::models::TrackingPixelDto create_tracking_pixel(create_tracking_pixel_options)
Create tracking pixel

Create a tracking pixel. A tracking pixel is an image that can be embedded in an email. When the email is viewed and the image is seen MailSlurp will mark the pixel as seen. Use tracking pixels to monitor email open events. You can receive open notifications via webhook or by fetching the pixel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_tracking_pixel_options** | [**CreateTrackingPixelOptions**](CreateTrackingPixelOptions) | createTrackingPixelOptions | [required] |

### Return type

[**crate::models::TrackingPixelDto**](TrackingPixelDto)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_all_tracking_pixels

> crate::models::PageTrackingPixelProjection get_all_tracking_pixels(before, page, search_filter, since, size, sort)
Get tracking pixels

List tracking pixels in paginated form

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**before** | Option<**String**> | Filter by created at before the given timestamp |  |
**page** | Option<**i32**> | Optional page index in list pagination |  |[default to 0]
**search_filter** | Option<**String**> | Optional search filter |  |
**since** | Option<**String**> | Filter by created at after the given timestamp |  |
**size** | Option<**i32**> | Optional page size in list pagination |  |[default to 20]
**sort** | Option<**String**> | Optional createdAt sort direction ASC or DESC |  |[default to ASC]

### Return type

[**crate::models::PageTrackingPixelProjection**](PageTrackingPixelProjection)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)


## get_tracking_pixel

> crate::models::TrackingPixelDto get_tracking_pixel(id)
Get pixel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**String**]() | id | [required] |

### Return type

[**crate::models::TrackingPixelDto**](TrackingPixelDto)

### Authorization

[API_KEY](../README#API_KEY)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to Model list]](../README#documentation-for-models) [[Back to README]](../README)

