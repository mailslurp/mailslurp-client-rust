# WebhookEmailReadPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> | Date time of event creation | [optional]
**email_id** | Option<**String**> | ID of the email that was received. Use this ID for fetching the email with the `EmailController`. | [optional]
**email_is_read** | Option<**bool**> | Is the email read | [optional]
**event_name** | Option<**String**> | Name of the event type webhook is being triggered for. | [optional]
**inbox_id** | Option<**String**> | Id of the inbox that received an email | [optional]
**message_id** | Option<**String**> | Idempotent message ID. Store this ID locally or in a database to prevent message duplication. | [optional]
**webhook_id** | Option<**String**> | ID of webhook entity being triggered | [optional]
**webhook_name** | Option<**String**> | Name of the webhook being triggered | [optional]

[[Back to Model list]](../README#documentation-for-models) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to README]](../README)


