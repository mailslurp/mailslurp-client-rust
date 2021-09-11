# Email

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**analysis** | Option<[**crate::models::EmailAnalysis**](EmailAnalysis)> |  | [optional]
**attachments** | Option<**Vec<String>**> | List of IDs of attachments found in the email. Use these IDs with the Inbox and Email Controllers to download attachments and attachment meta data such as filesize, name, extension. | [optional]
**bcc** | Option<**Vec<String>**> | List of `BCC` recipients email addresses that the email was addressed to. See recipients object for names. | [optional]
**body** | Option<**String**> | The body of the email message as text parsed from the SMTP message body (does not include attachments). Fetch the raw content to access the SMTP message and use the attachments property to access attachments. The body is stored separately to the email entity so the body is not returned in paginated results only in full single email or wait requests. | [optional]
**body_excerpt** | Option<**String**> | An excerpt of the body of the email message for quick preview . | [optional]
**body_md5_hash** | Option<**String**> | A hash signature of the email message using MD5. Useful for comparing emails without fetching full body. | [optional]
**cc** | Option<**Vec<String>**> | List of `CC` recipients email addresses that the email was addressed to. See recipients object for names. | [optional]
**charset** | Option<**String**> | Detected character set of the email body such as UTF-8 | [optional]
**created_at** | Option<**String**> | When was the email received by MailSlurp | [optional]
**from** | Option<**String**> | Who the email was sent from. An email address - see fromName for the sender name. | [optional]
**headers** | Option<**::std::collections::HashMap<String, String>**> | Collection of SMTP headers attached to email | [optional]
**id** | Option<**String**> | ID of the email entity | [optional]
**inbox_id** | Option<**String**> | ID of the inbox that received the email | [optional]
**is_html** | Option<**bool**> | Is the email body HTML | [optional]
**read** | Option<**bool**> | Read flag. Has the email ever been viewed in the dashboard or fetched via the API with a hydrated body? If so the email is marked as read. Paginated results do not affect read status. Read status is different to email opened event as it depends on your own account accessing the email. Email opened is determined by tracking pixels sent to other uses if enable during sending. You can listened for both email read and email opened events using webhooks. | [optional]
**recipients** | Option<[**crate::models::EmailRecipients**](EmailRecipients)> |  | [optional]
**reply_to** | Option<**String**> | The `replyTo` field on the received email message | [optional]
**sender** | Option<[**crate::models::Sender**](Sender)> |  | [optional]
**subject** | Option<**String**> | The subject line of the email message as specified by SMTP subject header | [optional]
**team_access** | Option<**bool**> | Can the email be accessed by organization team members | [optional]
**to** | Option<**Vec<String>**> | List of `To` recipient email addresses that the email was addressed to. See recipients object for names. | [optional]
**updated_at** | Option<**String**> | When was the email last updated | [optional]
**user_id** | Option<**String**> | ID of user that email belongs to | [optional]

[[Back to Model list]](../README#documentation-for-models) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to README]](../README)


