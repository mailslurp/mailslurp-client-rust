# Email

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**analysis** | Option<[**crate::models::EmailAnalysis**](EmailAnalysis)> |  | [optional]
**attachments** | Option<**Vec<String>**> | List of IDs of attachments found in the email. Use these IDs with the Inbox and Email Controllers to download attachments and attachment meta data such as filesize, name, extension. | [optional]
**bcc** | Option<**Vec<String>**> | List of `BCC` recipients email was addressed to | [optional]
**body** | Option<**String**> | The body of the email message | [optional]
**body_md5_hash** | Option<**String**> | A hash signature of the email message | [optional]
**cc** | Option<**Vec<String>**> | List of `CC` recipients email was addressed to | [optional]
**charset** | Option<**String**> | Detected character set of the email body such as UTF-8 | [optional]
**created_at** | Option<**String**> | When was the email received by MailSlurp | [optional]
**from** | Option<**String**> | Who the email was sent from | [optional]
**headers** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**id** | Option<**String**> | ID of the email | [optional]
**inbox_id** | Option<**String**> | ID of the inbox that received the email | [optional]
**is_html** | Option<**bool**> | Was HTML sent in the email body | [optional]
**read** | Option<**bool**> | Has the email been viewed ever. This means viewed in the dashboard or requested via the full email entity endpoints | [optional]
**reply_to** | Option<**String**> | The replyTo field on the received email | [optional]
**subject** | Option<**String**> | The subject line of the email message | [optional]
**team_access** | Option<**bool**> | Can the email be accessed by organization team members | [optional]
**to** | Option<**Vec<String>**> | List of `To` recipients email was addressed to | [optional]
**updated_at** | Option<**String**> | When was the email last updated | [optional]
**user_id** | Option<**String**> | ID of user that email belongs | [optional]

[[Back to Model list]](../README#documentation-for-models) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to README]](../README)


