# Rust API client for mailslurp

MailSlurp is an API for sending and receiving emails from dynamically allocated email addresses. It's designed for developers and QA teams to test applications, process inbound emails, send templated notifications, attachments, and more. 

## Resources
- [Homepage](https://www.mailslurp.com)
- Get an [API KEY](https://app.mailslurp.com/sign-up/)
- Generated [SDK Clients](https://www.mailslurp.com/docs/)
- [Examples](https://github.com/mailslurp/examples) repository


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 6.5.2
- Package version: 11.5.4
- Build package: org.openapitools.codegen.languages.RustClientCodegen

## Installation

Put the package under your project folder and add the following to `Cargo.toml` under `[dependencies]`:

```
    openapi = { path = "./generated" }
```

## Documentation for API Endpoints

All URIs are relative to *https://api.mailslurp.com*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AliasControllerApi* | [**create_alias**](docs/AliasControllerApi.md#create_alias) | **Post** /aliases | Create an email alias. Must be verified by clicking link inside verification email that will be sent to the address. Once verified the alias will be active.
*AliasControllerApi* | [**delete_alias**](docs/AliasControllerApi.md#delete_alias) | **Delete** /aliases/{aliasId} | Delete an email alias
*AliasControllerApi* | [**get_alias**](docs/AliasControllerApi.md#get_alias) | **Get** /aliases/{aliasId} | Get an email alias
*AliasControllerApi* | [**get_alias_emails**](docs/AliasControllerApi.md#get_alias_emails) | **Get** /aliases/{aliasId}/emails | Get emails for an alias
*AliasControllerApi* | [**get_alias_threads**](docs/AliasControllerApi.md#get_alias_threads) | **Get** /aliases/{aliasId}/threads | Get threads created for an alias
*AliasControllerApi* | [**get_aliases**](docs/AliasControllerApi.md#get_aliases) | **Get** /aliases | Get all email aliases you have created
*AliasControllerApi* | [**reply_to_alias_email**](docs/AliasControllerApi.md#reply_to_alias_email) | **Put** /aliases/{aliasId}/emails/{emailId} | Reply to an email
*AliasControllerApi* | [**send_alias_email**](docs/AliasControllerApi.md#send_alias_email) | **Post** /aliases/{aliasId}/emails | Send an email from an alias inbox
*AliasControllerApi* | [**update_alias**](docs/AliasControllerApi.md#update_alias) | **Put** /aliases/{aliasId} | Update an email alias
*AttachmentControllerApi* | [**upload_attachment**](docs/AttachmentControllerApi.md#upload_attachment) | **Post** /attachments | Upload an attachment for sending using base64 file encoding. Returns an array whose first element is the ID of the uploaded attachment.
*AttachmentControllerApi* | [**upload_attachment_bytes**](docs/AttachmentControllerApi.md#upload_attachment_bytes) | **Post** /attachments/bytes | Upload an attachment for sending using file byte stream input octet stream. Returns an array whose first element is the ID of the uploaded attachment.
*AttachmentControllerApi* | [**upload_multipart_form**](docs/AttachmentControllerApi.md#upload_multipart_form) | **Post** /attachments/multipart | Upload an attachment for sending using a Multipart Form request. Returns an array whose first element is the ID of the uploaded attachment.
*BulkActionsControllerApi* | [**bulk_create_inboxes**](docs/BulkActionsControllerApi.md#bulk_create_inboxes) | **Post** /bulk/inboxes | Bulk create Inboxes (email addresses)
*BulkActionsControllerApi* | [**bulk_delete_inboxes**](docs/BulkActionsControllerApi.md#bulk_delete_inboxes) | **Delete** /bulk/inboxes | Bulk Delete Inboxes
*BulkActionsControllerApi* | [**bulk_send_emails**](docs/BulkActionsControllerApi.md#bulk_send_emails) | **Post** /bulk/send | Bulk Send Emails
*CommonActionsControllerApi* | [**create_new_email_address**](docs/CommonActionsControllerApi.md#create_new_email_address) | **Post** /createInbox | Create new random inbox
*CommonActionsControllerApi* | [**create_new_email_address1**](docs/CommonActionsControllerApi.md#create_new_email_address1) | **Post** /newEmailAddress | Create new random inbox
*CommonActionsControllerApi* | [**empty_inbox**](docs/CommonActionsControllerApi.md#empty_inbox) | **Delete** /emptyInbox | Delete all emails in an inbox
*CommonActionsControllerApi* | [**send_email_simple**](docs/CommonActionsControllerApi.md#send_email_simple) | **Post** /sendEmail | Send an email
*ContactControllerApi* | [**create_contact**](docs/ContactControllerApi.md#create_contact) | **Post** /contacts | Create a contact
*ContactControllerApi* | [**delete_contact**](docs/ContactControllerApi.md#delete_contact) | **Delete** /contacts/{contactId} | Delete contact
*ContactControllerApi* | [**get_all_contacts**](docs/ContactControllerApi.md#get_all_contacts) | **Get** /contacts/paginated | Get all contacts
*ContactControllerApi* | [**get_contact**](docs/ContactControllerApi.md#get_contact) | **Get** /contacts/{contactId} | Get contact
*ContactControllerApi* | [**get_contacts**](docs/ContactControllerApi.md#get_contacts) | **Get** /contacts | Get all contacts
*DomainControllerApi* | [**add_domain_wildcard_catch_all**](docs/DomainControllerApi.md#add_domain_wildcard_catch_all) | **Post** /domains/{id}/wildcard | Add catch all wild card inbox to domain
*DomainControllerApi* | [**create_domain**](docs/DomainControllerApi.md#create_domain) | **Post** /domains | Create Domain
*DomainControllerApi* | [**delete_domain**](docs/DomainControllerApi.md#delete_domain) | **Delete** /domains/{id} | Delete a domain
*DomainControllerApi* | [**get_domain**](docs/DomainControllerApi.md#get_domain) | **Get** /domains/{id} | Get a domain
*DomainControllerApi* | [**get_domains**](docs/DomainControllerApi.md#get_domains) | **Get** /domains | Get domains
*DomainControllerApi* | [**update_domain**](docs/DomainControllerApi.md#update_domain) | **Put** /domains/{id} | Update a domain
*EmailControllerApi* | [**delete_all_emails**](docs/EmailControllerApi.md#delete_all_emails) | **Delete** /emails | Delete all emails
*EmailControllerApi* | [**delete_email**](docs/EmailControllerApi.md#delete_email) | **Delete** /emails/{emailId} | Delete an email
*EmailControllerApi* | [**download_attachment**](docs/EmailControllerApi.md#download_attachment) | **Get** /emails/{emailId}/attachments/{attachmentId} | Get email attachment bytes. If you have trouble with byte responses try the `downloadAttachmentBase64` response endpoints.
*EmailControllerApi* | [**download_attachment_base64**](docs/EmailControllerApi.md#download_attachment_base64) | **Get** /emails/{emailId}/attachments/{attachmentId}/base64 | Get email attachment as base64 encoded string (alternative to binary responses)
*EmailControllerApi* | [**forward_email**](docs/EmailControllerApi.md#forward_email) | **Post** /emails/{emailId}/forward | Forward email
*EmailControllerApi* | [**get_attachment_meta_data**](docs/EmailControllerApi.md#get_attachment_meta_data) | **Get** /emails/{emailId}/attachments/{attachmentId}/metadata | Get email attachment metadata
*EmailControllerApi* | [**get_attachments**](docs/EmailControllerApi.md#get_attachments) | **Get** /emails/{emailId}/attachments | Get all email attachment metadata
*EmailControllerApi* | [**get_email**](docs/EmailControllerApi.md#get_email) | **Get** /emails/{emailId} | Get email content
*EmailControllerApi* | [**get_email_content_match**](docs/EmailControllerApi.md#get_email_content_match) | **Post** /emails/{emailId}/contentMatch | Get email content regex pattern match results. Runs regex against email body and returns match groups.
*EmailControllerApi* | [**get_email_html**](docs/EmailControllerApi.md#get_email_html) | **Get** /emails/{emailId}/html | Get email content as HTML
*EmailControllerApi* | [**get_email_html_query**](docs/EmailControllerApi.md#get_email_html_query) | **Get** /emails/{emailId}/htmlQuery | Parse and return text from an email, stripping HTML and decoding encoded characters
*EmailControllerApi* | [**get_email_text_lines**](docs/EmailControllerApi.md#get_email_text_lines) | **Get** /emails/{emailId}/textLines | Parse and return text from an email, stripping HTML and decoding encoded characters
*EmailControllerApi* | [**get_emails_paginated**](docs/EmailControllerApi.md#get_emails_paginated) | **Get** /emails | Get all emails
*EmailControllerApi* | [**get_latest_email**](docs/EmailControllerApi.md#get_latest_email) | **Get** /emails/latest | Get latest email
*EmailControllerApi* | [**get_latest_email_in_inbox**](docs/EmailControllerApi.md#get_latest_email_in_inbox) | **Get** /emails/latestIn | Get latest email
*EmailControllerApi* | [**get_organization_emails_paginated**](docs/EmailControllerApi.md#get_organization_emails_paginated) | **Get** /emails/organization | Get all organization emails
*EmailControllerApi* | [**get_raw_email_contents**](docs/EmailControllerApi.md#get_raw_email_contents) | **Get** /emails/{emailId}/raw | Get raw email string
*EmailControllerApi* | [**get_raw_email_json**](docs/EmailControllerApi.md#get_raw_email_json) | **Get** /emails/{emailId}/raw/json | Get raw email in JSON
*EmailControllerApi* | [**get_unread_email_count**](docs/EmailControllerApi.md#get_unread_email_count) | **Get** /emails/unreadCount | Get unread email count
*EmailControllerApi* | [**reply_to_email**](docs/EmailControllerApi.md#reply_to_email) | **Put** /emails/{emailId} | Reply to an email
*EmailControllerApi* | [**validate_email**](docs/EmailControllerApi.md#validate_email) | **Post** /emails/{emailId}/validate | Validate email
*ExpiredControllerApi* | [**get_expiration_defaults**](docs/ExpiredControllerApi.md#get_expiration_defaults) | **Get** /expired/defaults | Get default expiration settings
*ExpiredControllerApi* | [**get_expired_inbox_by_inbox_id**](docs/ExpiredControllerApi.md#get_expired_inbox_by_inbox_id) | **Get** /expired/inbox/{inboxId} | Get expired inbox record for a previously existing inbox
*ExpiredControllerApi* | [**get_expired_inbox_record**](docs/ExpiredControllerApi.md#get_expired_inbox_record) | **Get** /expired/{expiredId} | Get an expired inbox record
*ExpiredControllerApi* | [**get_expired_inboxes**](docs/ExpiredControllerApi.md#get_expired_inboxes) | **Get** /expired | List records of expired inboxes
*FormControllerApi* | [**submit_form**](docs/FormControllerApi.md#submit_form) | **Post** /forms | Submit a form to be parsed and sent as an email to an address determined by the form fields
*GroupControllerApi* | [**add_contacts_to_group**](docs/GroupControllerApi.md#add_contacts_to_group) | **Put** /groups/{groupId}/contacts | Add contacts to a group
*GroupControllerApi* | [**create_group**](docs/GroupControllerApi.md#create_group) | **Post** /groups | Create a group
*GroupControllerApi* | [**delete_group**](docs/GroupControllerApi.md#delete_group) | **Delete** /groups/{groupId} | Delete group
*GroupControllerApi* | [**get_all_groups**](docs/GroupControllerApi.md#get_all_groups) | **Get** /groups/paginated | Get all Contact Groups in paginated format
*GroupControllerApi* | [**get_group**](docs/GroupControllerApi.md#get_group) | **Get** /groups/{groupId} | Get group
*GroupControllerApi* | [**get_group_with_contacts**](docs/GroupControllerApi.md#get_group_with_contacts) | **Get** /groups/{groupId}/contacts | Get group and contacts belonging to it
*GroupControllerApi* | [**get_group_with_contacts_paginated**](docs/GroupControllerApi.md#get_group_with_contacts_paginated) | **Get** /groups/{groupId}/contacts-paginated | Get group and paginated contacts belonging to it
*GroupControllerApi* | [**get_groups**](docs/GroupControllerApi.md#get_groups) | **Get** /groups | Get all groups
*GroupControllerApi* | [**remove_contacts_from_group**](docs/GroupControllerApi.md#remove_contacts_from_group) | **Delete** /groups/{groupId}/contacts | Remove contacts from a group
*InboxControllerApi* | [**create_inbox**](docs/InboxControllerApi.md#create_inbox) | **Post** /inboxes | Create an Inbox (email address)
*InboxControllerApi* | [**create_inbox_with_options**](docs/InboxControllerApi.md#create_inbox_with_options) | **Post** /inboxes/withOptions | Create an inbox with additional options
*InboxControllerApi* | [**delete_all_inboxes**](docs/InboxControllerApi.md#delete_all_inboxes) | **Delete** /inboxes | Delete all inboxes
*InboxControllerApi* | [**delete_inbox**](docs/InboxControllerApi.md#delete_inbox) | **Delete** /inboxes/{inboxId} | Delete inbox
*InboxControllerApi* | [**get_all_inboxes**](docs/InboxControllerApi.md#get_all_inboxes) | **Get** /inboxes/paginated | List All Inboxes Paginated
*InboxControllerApi* | [**get_emails**](docs/InboxControllerApi.md#get_emails) | **Get** /inboxes/{inboxId}/emails | Get emails in an Inbox. This method is not idempotent as it allows retries and waits if you want certain conditions to be met before returning. For simple listing and sorting of known emails use the email controller instead.
*InboxControllerApi* | [**get_inbox**](docs/InboxControllerApi.md#get_inbox) | **Get** /inboxes/{inboxId} | Get Inbox
*InboxControllerApi* | [**get_inbox_emails_paginated**](docs/InboxControllerApi.md#get_inbox_emails_paginated) | **Get** /inboxes/{inboxId}/emails/paginated | Get inbox emails paginated
*InboxControllerApi* | [**get_inbox_sent_emails**](docs/InboxControllerApi.md#get_inbox_sent_emails) | **Get** /inboxes/{inboxId}/sent | Get Inbox Sent Emails
*InboxControllerApi* | [**get_inbox_tags**](docs/InboxControllerApi.md#get_inbox_tags) | **Get** /inboxes/tags | Get inbox tags
*InboxControllerApi* | [**get_inboxes**](docs/InboxControllerApi.md#get_inboxes) | **Get** /inboxes | List Inboxes / Email Addresses
*InboxControllerApi* | [**get_organization_inboxes**](docs/InboxControllerApi.md#get_organization_inboxes) | **Get** /inboxes/organization | List Organization Inboxes Paginated
*InboxControllerApi* | [**send_email**](docs/InboxControllerApi.md#send_email) | **Post** /inboxes/{inboxId} | Send Email
*InboxControllerApi* | [**send_email_and_confirm**](docs/InboxControllerApi.md#send_email_and_confirm) | **Post** /inboxes/{inboxId}/confirm | Send email and return sent confirmation
*InboxControllerApi* | [**set_inbox_favourited**](docs/InboxControllerApi.md#set_inbox_favourited) | **Put** /inboxes/{inboxId}/favourite | Set inbox favourited state
*InboxControllerApi* | [**update_inbox**](docs/InboxControllerApi.md#update_inbox) | **Patch** /inboxes/{inboxId} | Update Inbox
*MailServerControllerApi* | [**describe_mail_server_domain**](docs/MailServerControllerApi.md#describe_mail_server_domain) | **Post** /mail-server/describe/domain | Get DNS Mail Server records for a domain
*MailServerControllerApi* | [**get_dns_lookup**](docs/MailServerControllerApi.md#get_dns_lookup) | **Post** /mail-server/describe/dns-lookup | Lookup DNS records for a domain
*MailServerControllerApi* | [**get_ip_address**](docs/MailServerControllerApi.md#get_ip_address) | **Post** /mail-server/describe/ip-address | Get IP address for a domain
*MailServerControllerApi* | [**verify_email_address**](docs/MailServerControllerApi.md#verify_email_address) | **Post** /mail-server/verify/email-address | Verify the existence of an email address at a given mail server.
*SentEmailsControllerApi* | [**get_sent_email**](docs/SentEmailsControllerApi.md#get_sent_email) | **Get** /sent/{id} | Get sent email receipt
*SentEmailsControllerApi* | [**get_sent_emails**](docs/SentEmailsControllerApi.md#get_sent_emails) | **Get** /sent | Get all sent emails in paginated form
*SentEmailsControllerApi* | [**get_sent_organization_emails**](docs/SentEmailsControllerApi.md#get_sent_organization_emails) | **Get** /sent/organization | Get all sent organization emails in paginated form
*TemplateControllerApi* | [**create_template**](docs/TemplateControllerApi.md#create_template) | **Post** /templates | Create a Template
*TemplateControllerApi* | [**delete_template**](docs/TemplateControllerApi.md#delete_template) | **Delete** /templates/{TemplateId} | Delete Template
*TemplateControllerApi* | [**get_all_templates**](docs/TemplateControllerApi.md#get_all_templates) | **Get** /templates/paginated | Get all Templates in paginated format
*TemplateControllerApi* | [**get_template**](docs/TemplateControllerApi.md#get_template) | **Get** /templates/{TemplateId} | Get Template
*TemplateControllerApi* | [**get_templates**](docs/TemplateControllerApi.md#get_templates) | **Get** /templates | Get all Templates
*WaitForControllerApi* | [**wait_for**](docs/WaitForControllerApi.md#wait_for) | **Post** /waitFor | Wait for conditions to be met
*WaitForControllerApi* | [**wait_for_email_count**](docs/WaitForControllerApi.md#wait_for_email_count) | **Get** /waitForEmailCount | Wait for and return count number of emails 
*WaitForControllerApi* | [**wait_for_latest_email**](docs/WaitForControllerApi.md#wait_for_latest_email) | **Get** /waitForLatestEmail | Fetch inbox's latest email or if empty wait for an email to arrive
*WaitForControllerApi* | [**wait_for_matching_email**](docs/WaitForControllerApi.md#wait_for_matching_email) | **Post** /waitForMatchingEmails | Wait or return list of emails that match simple matching patterns
*WaitForControllerApi* | [**wait_for_matching_first_email**](docs/WaitForControllerApi.md#wait_for_matching_first_email) | **Post** /waitForMatchingFirstEmail | Wait for or return the first email that matches proved MatchOptions array
*WaitForControllerApi* | [**wait_for_nth_email**](docs/WaitForControllerApi.md#wait_for_nth_email) | **Get** /waitForNthEmail | Wait for or fetch the email with a given index in the inbox specified
*WebhookControllerApi* | [**create_webhook**](docs/WebhookControllerApi.md#create_webhook) | **Post** /inboxes/{inboxId}/webhooks | Attach a WebHook URL to an inbox
*WebhookControllerApi* | [**delete_webhook**](docs/WebhookControllerApi.md#delete_webhook) | **Delete** /inboxes/{inboxId}/webhooks/{webhookId} | Delete and disable a Webhook for an Inbox
*WebhookControllerApi* | [**get_all_webhooks**](docs/WebhookControllerApi.md#get_all_webhooks) | **Get** /webhooks/paginated | List Webhooks Paginated
*WebhookControllerApi* | [**get_webhook**](docs/WebhookControllerApi.md#get_webhook) | **Get** /webhooks/{webhookId} | Get a webhook for an Inbox
*WebhookControllerApi* | [**get_webhooks**](docs/WebhookControllerApi.md#get_webhooks) | **Get** /inboxes/{inboxId}/webhooks | Get all Webhooks for an Inbox
*WebhookControllerApi* | [**send_test_data**](docs/WebhookControllerApi.md#send_test_data) | **Post** /webhooks/{webhookId}/test | Send webhook test data


## Documentation For Models

 - [Alias](docs/Alias.md)
 - [AliasDto](docs/AliasDto.md)
 - [AliasProjection](docs/AliasProjection.md)
 - [AttachmentMetaData](docs/AttachmentMetaData.md)
 - [BasicAuthOptions](docs/BasicAuthOptions.md)
 - [BulkSendEmailOptions](docs/BulkSendEmailOptions.md)
 - [ContactDto](docs/ContactDto.md)
 - [ContactProjection](docs/ContactProjection.md)
 - [ContentMatchOptions](docs/ContentMatchOptions.md)
 - [CreateAliasOptions](docs/CreateAliasOptions.md)
 - [CreateContactOptions](docs/CreateContactOptions.md)
 - [CreateDomainOptions](docs/CreateDomainOptions.md)
 - [CreateGroupOptions](docs/CreateGroupOptions.md)
 - [CreateInboxDto](docs/CreateInboxDto.md)
 - [CreateTemplateOptions](docs/CreateTemplateOptions.md)
 - [CreateWebhookOptions](docs/CreateWebhookOptions.md)
 - [DescribeDomainOptions](docs/DescribeDomainOptions.md)
 - [DescribeMailServerDomainResult](docs/DescribeMailServerDomainResult.md)
 - [DnsLookupOptions](docs/DnsLookupOptions.md)
 - [DnsLookupResult](docs/DnsLookupResult.md)
 - [DnsLookupResults](docs/DnsLookupResults.md)
 - [DomainDto](docs/DomainDto.md)
 - [DomainNameRecord](docs/DomainNameRecord.md)
 - [DomainPreview](docs/DomainPreview.md)
 - [DownloadAttachmentDto](docs/DownloadAttachmentDto.md)
 - [Email](docs/Email.md)
 - [EmailAnalysis](docs/EmailAnalysis.md)
 - [EmailContentMatchResult](docs/EmailContentMatchResult.md)
 - [EmailPreview](docs/EmailPreview.md)
 - [EmailProjection](docs/EmailProjection.md)
 - [EmailTextLinesResult](docs/EmailTextLinesResult.md)
 - [EmailVerificationResult](docs/EmailVerificationResult.md)
 - [ExpirationDefaults](docs/ExpirationDefaults.md)
 - [ExpiredInboxDto](docs/ExpiredInboxDto.md)
 - [ExpiredInboxRecordProjection](docs/ExpiredInboxRecordProjection.md)
 - [ForwardEmailOptions](docs/ForwardEmailOptions.md)
 - [GroupContactsDto](docs/GroupContactsDto.md)
 - [GroupDto](docs/GroupDto.md)
 - [GroupProjection](docs/GroupProjection.md)
 - [HtmlValidationResult](docs/HtmlValidationResult.md)
 - [Inbox](docs/Inbox.md)
 - [InboxProjection](docs/InboxProjection.md)
 - [IpAddressResult](docs/IpAddressResult.md)
 - [MatchOption](docs/MatchOption.md)
 - [MatchOptions](docs/MatchOptions.md)
 - [NameServerRecord](docs/NameServerRecord.md)
 - [OrganizationInboxProjection](docs/OrganizationInboxProjection.md)
 - [PageAlias](docs/PageAlias.md)
 - [PageContactProjection](docs/PageContactProjection.md)
 - [PageEmailPreview](docs/PageEmailPreview.md)
 - [PageEmailProjection](docs/PageEmailProjection.md)
 - [PageExpiredInboxRecordProjection](docs/PageExpiredInboxRecordProjection.md)
 - [PageGroupProjection](docs/PageGroupProjection.md)
 - [PageInboxProjection](docs/PageInboxProjection.md)
 - [PageOrganizationInboxProjection](docs/PageOrganizationInboxProjection.md)
 - [PageSentEmailProjection](docs/PageSentEmailProjection.md)
 - [PageTemplateProjection](docs/PageTemplateProjection.md)
 - [PageThreadProjection](docs/PageThreadProjection.md)
 - [PageWebhookProjection](docs/PageWebhookProjection.md)
 - [Pageable](docs/Pageable.md)
 - [RawEmailJson](docs/RawEmailJson.md)
 - [ReplyToAliasEmailOptions](docs/ReplyToAliasEmailOptions.md)
 - [ReplyToEmailOptions](docs/ReplyToEmailOptions.md)
 - [SendEmailOptions](docs/SendEmailOptions.md)
 - [SentEmailDto](docs/SentEmailDto.md)
 - [SentEmailProjection](docs/SentEmailProjection.md)
 - [SetInboxFavouritedOptions](docs/SetInboxFavouritedOptions.md)
 - [SimpleSendEmailOptions](docs/SimpleSendEmailOptions.md)
 - [Sort](docs/Sort.md)
 - [TemplateDto](docs/TemplateDto.md)
 - [TemplateProjection](docs/TemplateProjection.md)
 - [TemplateVariable](docs/TemplateVariable.md)
 - [ThreadProjection](docs/ThreadProjection.md)
 - [UnreadCount](docs/UnreadCount.md)
 - [UpdateAliasOptions](docs/UpdateAliasOptions.md)
 - [UpdateDomainOptions](docs/UpdateDomainOptions.md)
 - [UpdateGroupContacts](docs/UpdateGroupContacts.md)
 - [UpdateInboxOptions](docs/UpdateInboxOptions.md)
 - [UploadAttachmentOptions](docs/UploadAttachmentOptions.md)
 - [ValidationDto](docs/ValidationDto.md)
 - [ValidationMessage](docs/ValidationMessage.md)
 - [VerifyEmailAddressOptions](docs/VerifyEmailAddressOptions.md)
 - [WaitForConditions](docs/WaitForConditions.md)
 - [WebhookDto](docs/WebhookDto.md)
 - [WebhookProjection](docs/WebhookProjection.md)
 - [WebhookTestRequest](docs/WebhookTestRequest.md)
 - [WebhookTestResponse](docs/WebhookTestResponse.md)
 - [WebhookTestResult](docs/WebhookTestResult.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



