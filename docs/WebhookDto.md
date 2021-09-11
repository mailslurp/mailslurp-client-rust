# WebhookDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**basic_auth** | Option<**bool**> | Does webhook expect basic authentication? If true it means you created this webhook with a username and password. MailSlurp will use these in the URL to authenticate itself. | [optional]
**created_at** | Option<**String**> | When the webhook was created | [optional]
**event_name** | Option<**String**> |  | [optional]
**id** | Option<**String**> | ID of the Webhook | [optional]
**inbox_id** | Option<**String**> | The inbox that the Webhook will be triggered by | [optional]
**method** | Option<**String**> | HTTP method that your server endpoint must listen for | [optional]
**name** | Option<**String**> | Name of the webhook | [optional]
**payload_json_schema** | Option<**String**> | Deprecated. Fetch JSON Schema for webhook using the getJsonSchemaForWebhookPayload method | [optional]
**updated_at** | **String** |  | 
**url** | Option<**String**> | URL of your server that the webhook will be sent to. The schema of the JSON that is sent is described by the payloadJsonSchema. | [optional]
**user_id** | Option<**String**> | User ID of the Webhook | [optional]

[[Back to Model list]](../README#documentation-for-models) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to README]](../README)


