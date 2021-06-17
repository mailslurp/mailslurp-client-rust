/*
 * MailSlurp API
 *
 * MailSlurp is an API for sending and receiving emails from dynamically allocated email addresses. It's designed for developers and QA teams to test applications, process inbound emails, send templated notifications, attachments, and more.  ## Resources  - [Homepage](https://www.mailslurp.com) - Get an [API KEY](https://app.mailslurp.com/sign-up/) - Generated [SDK Clients](https://www.mailslurp.com/docs/) - [Examples](https://github.com/mailslurp/examples) repository 
 *
 * The version of the OpenAPI document: 6.5.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AliasDto : Email alias representation



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AliasDto {
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// The alias's email address for receiving email
    #[serde(rename = "emailAddress", skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    /// Inbox that is associated with the alias
    #[serde(rename = "inboxId", skip_serializing_if = "Option::is_none")]
    pub inbox_id: Option<String>,
    /// Has the alias been verified. You must verify an alias if the masked email address has not yet been verified by your account
    #[serde(rename = "isVerified", skip_serializing_if = "Option::is_none")]
    pub is_verified: Option<bool>,
    /// The underlying email address that is hidden and will received forwarded email
    #[serde(rename = "maskedEmailAddress", skip_serializing_if = "Option::is_none")]
    pub masked_email_address: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// If alias will generate response threads or not when email are received by it
    #[serde(rename = "useThreads", skip_serializing_if = "Option::is_none")]
    pub use_threads: Option<bool>,
    #[serde(rename = "userId")]
    pub user_id: String,
}

impl AliasDto {
    /// Email alias representation
    pub fn new(id: String, user_id: String) -> AliasDto {
        AliasDto {
            created_at: None,
            email_address: None,
            id,
            inbox_id: None,
            is_verified: None,
            masked_email_address: None,
            name: None,
            updated_at: None,
            use_threads: None,
            user_id,
        }
    }
}


