/*
 * MailSlurp API
 *
 * MailSlurp is an API for sending and receiving emails from dynamically allocated email addresses. It's designed for developers and QA teams to test applications, process inbound emails, send templated notifications, attachments, and more.  ## Resources  - [Homepage](https://www.mailslurp.com) - Get an [API KEY](https://app.mailslurp.com/sign-up/) - Generated [SDK Clients](https://www.mailslurp.com/docs/) - [Examples](https://github.com/mailslurp/examples) repository
 *
 * The version of the OpenAPI document: 6.5.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ForwardEmailOptions : Options for forwarding an email



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForwardEmailOptions {
    /// Optional bcc recipients
    #[serde(rename = "bcc", skip_serializing_if = "Option::is_none")]
    pub bcc: Option<Vec<String>>,
    /// Optional cc recipients
    #[serde(rename = "cc", skip_serializing_if = "Option::is_none")]
    pub cc: Option<Vec<String>>,
    /// Optional from override
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// Subject for forwarded email
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// To recipients for forwarded email
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec<String>>,
    /// Optionally use inbox name as display name for sender email address
    #[serde(rename = "useInboxName", skip_serializing_if = "Option::is_none")]
    pub use_inbox_name: Option<bool>,
}

impl ForwardEmailOptions {
    /// Options for forwarding an email
    pub fn new() -> ForwardEmailOptions {
        ForwardEmailOptions {
            bcc: None,
            cc: None,
            from: None,
            subject: None,
            to: None,
            use_inbox_name: None,
        }
    }
}


