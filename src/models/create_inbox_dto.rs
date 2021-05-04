/*
 * MailSlurp API
 *
 * MailSlurp is an API for sending and receiving emails from dynamically allocated email addresses. It's designed for developers and QA teams to test applications, process inbound emails, send templated notifications, attachments, and more.   ## Resources - [Homepage](https://www.mailslurp.com) - Get an [API KEY](https://app.mailslurp.com/sign-up/) - Generated [SDK Clients](https://www.mailslurp.com/docs/) - [Examples](https://github.com/mailslurp/examples) repository 
 *
 * The version of the OpenAPI document: 6.5.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreateInboxDto : Options for creating an inbox. An inbox has a real email address that can send and receive emails. Inboxes can be permanent or expire at a given time. They can use a custom email address (by verifying your own domain) or a randomly assigned email ending in either `mailslurp.com` or if `useDomainPool` is enabled then ending in a similar domains such as `mailslurp.xyz` (selected at random). 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateInboxDto {
    /// Grant team access to this inbox and the emails that belong to it for team members of your organization.
    #[serde(rename = "allowTeamAccess", skip_serializing_if = "Option::is_none")]
    pub allow_team_access: Option<bool>,
    /// Optional description of the inbox for labelling purposes. Is shown in the dashboard and can be used with
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A custom email address to use with the inbox. Defaults to null. When null MailSlurp will assign a random email address to the inbox such as `123@mailslurp.com`. If you use the `useDomainPool` option when the email address is null it will generate an email address with a more varied domain ending such as `123@mailslurp.info` or `123@mailslurp.biz`. When a custom email address is provided the address is split into a domain and the domain is queried against your user. If you have created the domain in the MailSlurp dashboard and verified it you can use any email address that ends with the domain. Send an email to this address and the inbox will receive and store it for you. To retrieve the email use the Inbox and Email Controller endpoints with the inbox ID.
    #[serde(rename = "emailAddress", skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// Optional inbox expiration date. If null then this inbox is permanent and the emails in it won't be deleted. If an expiration date is provided or is required by your plan the inbox will be closed when the expiration time is reached. Expired inboxes still contain their emails but can no longer send or receive emails. An ExpiredInboxRecord is created when an inbox and the email address and inbox ID are recorded. The expiresAt property is a timestamp string in ISO DateTime Format yyyy-MM-dd'T'HH:mm:ss.SSSXXX.
    #[serde(rename = "expiresAt", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Number of milliseconds that inbox should exist for
    #[serde(rename = "expiresIn", skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<i64>,
    /// Is the inbox favorited. Favouriting inboxes is typically done in the dashboard for quick access or filtering
    #[serde(rename = "favourite", skip_serializing_if = "Option::is_none")]
    pub favourite: Option<bool>,
    /// HTTP or SMTP inbox
    #[serde(rename = "inboxType", skip_serializing_if = "Option::is_none")]
    pub inbox_type: Option<InboxType>,
    /// Optional name of the inbox. Displayed in the dashboard for easier search
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Tags that inbox has been tagged with. Tags can be added to inboxes to group different inboxes within an account. You can also search for inboxes by tag in the dashboard UI.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Use the MailSlurp domain name pool with this inbox when creating the email address. Defaults to null. If enabled the inbox will be an email address with a domain randomly chosen from a list of the MailSlurp domains. This is useful when the default `@mailslurp.com` email addresses used with inboxes are blocked or considered spam by a provider or receiving service. When domain pool is enabled an email address will be generated ending in `@mailslurp.{world,info,xyz,...}` . This means a TLD is randomly selecting from a list of `.biz`, `.info`, `.xyz` etc to add variance to the generated email addresses. When null or false MailSlurp uses the default behavior of `@mailslurp.com` or custom email address provided by the emailAddress field.
    #[serde(rename = "useDomainPool", skip_serializing_if = "Option::is_none")]
    pub use_domain_pool: Option<bool>,
}

impl CreateInboxDto {
    /// Options for creating an inbox. An inbox has a real email address that can send and receive emails. Inboxes can be permanent or expire at a given time. They can use a custom email address (by verifying your own domain) or a randomly assigned email ending in either `mailslurp.com` or if `useDomainPool` is enabled then ending in a similar domains such as `mailslurp.xyz` (selected at random). 
    pub fn new() -> CreateInboxDto {
        CreateInboxDto {
            allow_team_access: None,
            description: None,
            email_address: None,
            expires_at: None,
            expires_in: None,
            favourite: None,
            inbox_type: None,
            name: None,
            tags: None,
            use_domain_pool: None,
        }
    }
}

/// HTTP or SMTP inbox
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InboxType {
    #[serde(rename = "HTTP_INBOX")]
    HTTPINBOX,
    #[serde(rename = "SMTP_INBOX")]
    SMTPINBOX,
}

