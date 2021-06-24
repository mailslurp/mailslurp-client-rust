# ReplyToAliasEmailOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**attachments** | Option<**Vec<String>**> | List of uploaded attachments to send with the reply. Optional. | [optional]
**body** | Option<**String**> | Body of the reply email you want to send | [optional]
**charset** | Option<**String**> | The charset that your message should be sent with. Optional. Default is UTF-8 | [optional]
**is_html** | Option<**bool**> | Is the reply HTML | [optional]
**send_strategy** | Option<**String**> | When to send the email. Typically immediately | [optional]
**template** | Option<**String**> | Template ID to use instead of body. Will use template variable map to fill defined variable slots. | [optional]
**template_variables** | Option<[**serde_json::Value**]()> | Template variables if using a template | [optional]
**use_inbox_name** | Option<**bool**> | Optionally use inbox name as display name for sender email address | [optional]

[[Back to Model list]](../README#documentation-for-models) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to README]](../README)


