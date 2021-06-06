# CreateWebhookOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**basic_auth** | Option<[**crate::models::BasicAuthOptions**](BasicAuthOptions)> |  | [optional]
**event_name** | Option<**String**> | Optional webhook event name. Default is `EMAIL_RECEIVED`. Payload differ according to the webhook event name. | [optional]
**name** | Option<**String**> | Optional name for the webhook | [optional]
**url** | Option<**String**> | Public URL on your server that MailSlurp can post WebhookNotification payload to when an email is received. The payload of the submitted JSON is described by https://api.mailslurp.com/schemas/webhook-payload | [optional]

[[Back to Model list]](../README#documentation-for-models) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to README]](../README)


