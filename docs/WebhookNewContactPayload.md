# WebhookNewContactPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**company** | Option<**String**> |  | [optional]
**contact_id** | **String** |  | 
**created_at** | **String** |  | 
**email_addresses** | **Vec<String>** |  | 
**event_name** | Option<**String**> | Name of the event type webhook is being triggered for. | [optional]
**first_name** | Option<**String**> |  | [optional]
**group_id** | Option<**String**> |  | [optional]
**last_name** | Option<**String**> |  | [optional]
**message_id** | Option<**String**> | Idempotent message ID. Store this ID locally or in a database to prevent message duplication. | [optional]
**meta_data** | Option<[**serde_json::Value**]()> |  | [optional]
**opt_out** | Option<**bool**> |  | [optional]
**primary_email_address** | Option<**String**> |  | [optional]
**tags** | **Vec<String>** |  | 
**webhook_id** | Option<**String**> | ID of webhook entity being triggered | [optional]
**webhook_name** | Option<**String**> | Name of the webhook being triggered | [optional]

[[Back to Model list]](../README#documentation-for-models) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to README]](../README)


