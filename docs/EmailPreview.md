# EmailPreview

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**attachments** | Option<**Vec<String>**> | List of IDs of attachments found in the email. Use these IDs with the Inbox and Email Controllers to download attachments and attachment meta data such as filesize, name, extension. | [optional]
**bcc** | Option<**Vec<String>**> | List of `BCC` recipients email was addressed to | [optional]
**cc** | Option<**Vec<String>**> | List of `CC` recipients email was addressed to | [optional]
**created_at** | Option<**String**> | When was the email received by MailSlurp | [optional]
**from** | Option<**String**> | Who the email was sent from | [optional]
**id** | Option<**String**> | ID of the email entity | [optional]
**read** | Option<**bool**> | Read flag. Has the email ever been viewed in the dashboard or fetched via the API? If so the email is marked as read. | [optional]
**subject** | Option<**String**> | The subject line of the email message | [optional]
**to** | Option<**Vec<String>**> | List of `To` recipients that email was addressed to | [optional]

[[Back to Model list]](../README#documentation-for-models) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to README]](../README)


