/*
 * MailSlurp API
 *
 * MailSlurp is an API for sending and receiving emails from dynamically allocated email addresses. It's designed for developers and QA teams to test applications, process inbound emails, send templated notifications, attachments, and more.   ## Resources - [Homepage](https://www.mailslurp.com) - Get an [API KEY](https://app.mailslurp.com/sign-up/) - Generated [SDK Clients](https://www.mailslurp.com/docs/) - [Examples](https://github.com/mailslurp/examples) repository 
 *
 * The version of the OpenAPI document: 6.5.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Email : Email model (also referred to as EmailDto). Represents an email that was received by an inbox. If you want the original SMTP message see the `getRawEmail` endpoint.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Email {
    #[serde(rename = "analysis", skip_serializing_if = "Option::is_none")]
    pub analysis: Option<crate::models::EmailAnalysis>,
    /// List of IDs of attachments found in the email. Use these IDs with the Inbox and Email Controllers to download attachments and attachment meta data such as filesize, name, extension.
    #[serde(rename = "attachments", skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<String>>,
    /// List of `BCC` recipients email was addressed to
    #[serde(rename = "bcc", skip_serializing_if = "Option::is_none")]
    pub bcc: Option<Vec<String>>,
    /// The body of the email message
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// A hash signature of the email message
    #[serde(rename = "bodyMD5Hash", skip_serializing_if = "Option::is_none")]
    pub body_md5_hash: Option<String>,
    /// List of `CC` recipients email was addressed to
    #[serde(rename = "cc", skip_serializing_if = "Option::is_none")]
    pub cc: Option<Vec<String>>,
    /// Detected character set of the email body such as UTF-8
    #[serde(rename = "charset", skip_serializing_if = "Option::is_none")]
    pub charset: Option<String>,
    /// When was the email received by MailSlurp
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Who the email was sent from
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none")]
    pub headers: Option<::std::collections::HashMap<String, String>>,
    /// ID of the email
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// ID of the inbox that received the email
    #[serde(rename = "inboxId", skip_serializing_if = "Option::is_none")]
    pub inbox_id: Option<String>,
    /// Was HTML sent in the email body
    #[serde(rename = "isHTML", skip_serializing_if = "Option::is_none")]
    pub is_html: Option<bool>,
    /// Has the email been viewed ever
    #[serde(rename = "read", skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
    /// The replyTo field on the received email
    #[serde(rename = "replyTo", skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    /// The subject line of the email message
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// List of `To` recipients email was addressed to
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec<String>>,
    /// When was the email last updated
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// ID of user that email belongs
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl Email {
    /// Email model (also referred to as EmailDto). Represents an email that was received by an inbox. If you want the original SMTP message see the `getRawEmail` endpoint.
    pub fn new() -> Email {
        Email {
            analysis: None,
            attachments: None,
            bcc: None,
            body: None,
            body_md5_hash: None,
            cc: None,
            charset: None,
            created_at: None,
            from: None,
            headers: None,
            id: None,
            inbox_id: None,
            is_html: None,
            read: None,
            reply_to: None,
            subject: None,
            to: None,
            updated_at: None,
            user_id: None,
        }
    }
}


