# CreateInboxDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | Optional description of an inbox for labelling purposes | [optional]
**email_address** | Option<**String**> | Optionally specify an email address you want the inbox to have. When left blank an email address will be randomly assigned to the inbox usually ending in `@mailslurp.com`. Custom email addresses must include your own custom domain that you have configured in MailSlurp. So if your domain is `mysite.com` you can created any email address ending in `@mysite.com`. All email addresses are transformed to lowercase! | [optional]
**expires_at** | Option<**String**> | When, if ever, will the inbox expire and be deleted. If null then this inbox is permanent and the emails in it won't be deleted. Timestamp passed as string. | [optional]
**favourite** | Option<**bool**> | Is the inbox favorited. Favouriting inboxes is typically done in the dashboard for quick access | [optional]
**name** | Option<**String**> | Optional name of the inbox. Displayed in the dashboard for easier search | [optional]
**tags** | Option<**Vec<String>**> | Tags that inbox has been tagged with. Tags can be added to inboxes to group different inboxes within an account. You can also search for inboxes by tag in the dashboard UI. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


