/*
 * MailSlurp API
 *
 * MailSlurp is an API for sending and receiving emails from dynamically allocated email addresses. It's designed for developers and QA teams to test applications, process inbound emails, send templated notifications, attachments, and more.  ## Resources  - [Homepage](https://www.mailslurp.com) - Get an [API KEY](https://app.mailslurp.com/sign-up/) - Generated [SDK Clients](https://www.mailslurp.com/docs/) - [Examples](https://github.com/mailslurp/examples) repository
 *
 * The version of the OpenAPI document: 6.5.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AliasProjection : Representation of a alias



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AliasProjection {
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "emailAddress")]
    pub email_address: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "inboxId")]
    pub inbox_id: String,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "useThreads", skip_serializing_if = "Option::is_none")]
    pub use_threads: Option<bool>,
    #[serde(rename = "userId")]
    pub user_id: String,
}

impl AliasProjection {
    /// Representation of a alias
    pub fn new(created_at: String, email_address: String, id: String, inbox_id: String, updated_at: String, user_id: String) -> AliasProjection {
        AliasProjection {
            created_at,
            email_address,
            id,
            inbox_id,
            name: None,
            updated_at,
            use_threads: None,
            user_id,
        }
    }
}


