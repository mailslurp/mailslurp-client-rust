# CreateInboxDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allow_team_access** | Option<**bool**> | Grant team access to this inbox and the emails that belong to it for team members of your organization. | [optional]
**description** | Option<**String**> | Optional description of the inbox for labelling purposes. Is shown in the dashboard and can be used with | [optional]
**email_address** | Option<**String**> | A custom email address to use with the inbox. Defaults to null. When null MailSlurp will assign a random email address to the inbox such as `123@mailslurp.com`. If you use the `useDomainPool` option when the email address is null it will generate an email address with a more varied domain ending such as `123@mailslurp.info` or `123@mailslurp.biz`. When a custom email address is provided the address is split into a domain and the domain is queried against your user. If you have created the domain in the MailSlurp dashboard and verified it you can use any email address that ends with the domain. Send an email to this address and the inbox will receive and store it for you. To retrieve the email use the Inbox and Email Controller endpoints with the inbox ID. | [optional]
**expires_at** | Option<**String**> | Optional inbox expiration date. If null then this inbox is permanent and the emails in it won't be deleted. If an expiration date is provided or is required by your plan the inbox will be closed when the expiration time is reached. Expired inboxes still contain their emails but can no longer send or receive emails. An ExpiredInboxRecord is created when an inbox and the email address and inbox ID are recorded. The expiresAt property is a timestamp string in ISO DateTime Format yyyy-MM-dd'T'HH:mm:ss.SSSXXX. | [optional]
**expires_in** | Option<**i64**> | Number of milliseconds that inbox should exist for | [optional]
**favourite** | Option<**bool**> | Is the inbox favorited. Favouriting inboxes is typically done in the dashboard for quick access or filtering | [optional]
**name** | Option<**String**> | Optional name of the inbox. Displayed in the dashboard for easier search | [optional]
**tags** | Option<**Vec<String>**> | Tags that inbox has been tagged with. Tags can be added to inboxes to group different inboxes within an account. You can also search for inboxes by tag in the dashboard UI. | [optional]
**use_domain_pool** | Option<**bool**> | Use the MailSlurp domain name pool with this inbox when creating the email address. Defaults to null. If enabled the inbox will be an email address with a domain randomly chosen from a list of the MailSlurp domains. This is useful when the default `@mailslurp.com` email addresses used with inboxes are blocked or considered spam by a provider or receiving service. When domain pool is enabled an email address will be generated ending in `@mailslurp.{world,info,xyz,...}` . This means a TLD is randomly selecting from a list of `.biz`, `.info`, `.xyz` etc to add variance to the generated email addresses. When null or false MailSlurp uses the default behavior of `@mailslurp.com` or custom email address provided by the emailAddress field. | [optional]

[[Back to Model list]](../README#documentation-for-models) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to README]](../README)


