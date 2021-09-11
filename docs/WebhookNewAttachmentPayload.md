# WebhookNewAttachmentPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**attachment_id** | Option<**String**> | ID of attachment. Use the `AttachmentController` to | [optional]
**content_length** | Option<**i64**> | Size of attachment in bytes | [optional]
**content_type** | Option<**String**> | Content type of attachment such as 'image/png' or 'application/pdf | [optional]
**event_name** | Option<**String**> | Name of the event type webhook is being triggered for. | [optional]
**message_id** | Option<**String**> | Idempotent message ID. Store this ID locally or in a database to prevent message duplication. | [optional]
**name** | Option<**String**> | Filename of the attachment if present | [optional]
**webhook_id** | Option<**String**> | ID of webhook entity being triggered | [optional]
**webhook_name** | Option<**String**> | Name of the webhook being triggered | [optional]

[[Back to Model list]](../README#documentation-for-models) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to README]](../README)


