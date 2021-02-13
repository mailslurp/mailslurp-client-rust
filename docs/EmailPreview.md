# EmailPreview

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**attachments** | Option<**Vec<String>**> | List of IDs of attachments found in the email. Use these IDs with the Inbox and Email Controllers to download attachments and attachment meta data such as filesize, name, extension. | [optional]
**bcc** | Option<**Vec<String>**> | List of `BCC` recipients email was addressed to | [optional]
**cc** | Option<**Vec<String>**> | List of `CC` recipients email was addressed to | [optional]
**created_at** | Option<**String**> | When was the email received by MailSlurp | [optional]
**from** | Option<**String**> | Who the email was sent from | [optional]
**id** | Option<**String**> | ID of the email | [optional]
**read** | Option<**bool**> | Has the email been viewed ever | [optional]
**subject** | Option<**String**> | The subject line of the email message | [optional]
**to** | Option<**Vec<String>**> | List of `To` recipients email was addressed to | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


