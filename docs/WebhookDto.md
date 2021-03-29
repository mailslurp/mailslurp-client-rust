# WebhookDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**basic_auth** | Option<**bool**> | Does webhook expect basic authentication? If true it means you created this webhook with a username and password. MailSlurp will use these in the URL to authenticate itself. | [optional]
**created_at** | Option<**String**> | When the webhook was created | [optional]
**id** | Option<**String**> | ID of the Webhook | [optional]
**inbox_id** | Option<**String**> | The inbox that the Webhook will be triggered by | [optional]
**method** | Option<**String**> | HTTP method that your server endpoint must listen for | [optional]
**name** | Option<**String**> | Name of the webhook | [optional]
**payload_json_schema** | Option<**String**> | JSON Schema for the payload that will be sent to your URL via the HTTP method described. | [optional]
**updated_at** | **String** |  | 
**url** | Option<**String**> | URL of your server that the webhook will be sent to. The schema of the JSON that is sent is described by the payloadJsonSchema. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


