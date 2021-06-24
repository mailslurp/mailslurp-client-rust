# DownloadAttachmentDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**base64_file_contents** | Option<**String**> | Base64 encoded string of attachment bytes. Decode the base64 encoded string to get the raw contents. If the file has a content type such as `text/html` you can read the contents directly by converting it to string using `utf-8` encoding. | [optional]
**content_type** | Option<**String**> | Content type of attachment. Examples are `image/png`, `application/msword`, `text/csv` etc. | [optional]
**size_bytes** | Option<**i64**> | Size in bytes of attachment content | [optional]

[[Back to Model list]](../README#documentation-for-models) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to README]](../README)


