# Inbox

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**String**> | When was the inbox created | [optional]
**description** | Option<**String**> | Optional description of an inbox for labelling purposes | [optional]
**email_address** | Option<**String**> | The inbox's email address. Send an email to this address and the inbox will receive and store it for you. To retrieve the email use the Inbox and Email Controller endpoints. | [optional]
**expires_at** | Option<**String**> | When, if ever, will the inbox expire and be deleted. If null then this inbox is permanent and the emails in it won't be deleted. Timestamp passed as string. | [optional]
**favourite** | Option<**bool**> | Is the inbox favourited | [optional]
**id** | Option<**String**> | ID of the inbox | [optional]
**name** | Option<**String**> | Optional name of the inbox. Displayed in the dashboard for easier search | [optional]
**tags** | Option<**Vec<String>**> | Tags that inbox has been tagged with | [optional]
**user_id** | Option<**String**> | ID of user that inbox belongs to | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


