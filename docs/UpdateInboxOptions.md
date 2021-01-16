# UpdateInboxOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | Description of an inbox for labelling and searching purposes | [optional]
**expires_at** | Option<**String**> | Inbox expiration time. When, if ever, the inbox should expire and be deleted. If null then this inbox is permanent and the emails in it won't be deleted. This is the default behavior unless expiration date is set. If an expiration date is set and the time is reached MailSlurp will expire the inbox and move it to an expired inbox entity. You can still access the emails belonging to it but it can no longer send or receive email. | [optional]
**favourite** | Option<**bool**> | Is the inbox favorited. Favouriting inboxes is typically done in the dashboard for quick access or filtering | [optional]
**name** | Option<**String**> | Name of the inbox. Displayed in the dashboard for easier search | [optional]
**tags** | Option<**Vec<String>**> | Tags that inbox has been tagged with. Tags can be added to inboxes to group different inboxes within an account. You can also search for inboxes by tag in the dashboard UI. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


