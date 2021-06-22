/*
 * MailSlurp API
 *
 * MailSlurp is an API for sending and receiving emails from dynamically allocated email addresses. It's designed for developers and QA teams to test applications, process inbound emails, send templated notifications, attachments, and more.  ## Resources  - [Homepage](https://www.mailslurp.com) - Get an [API KEY](https://app.mailslurp.com/sign-up/) - Generated [SDK Clients](https://www.mailslurp.com/docs/) - [Examples](https://github.com/mailslurp/examples) repository 
 *
 * The version of the OpenAPI document: 6.5.2
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SentEmailProjection {
    #[serde(rename = "attachments")]
    pub attachments: Vec<String>,
    #[serde(rename = "bcc")]
    pub bcc: Vec<String>,
    #[serde(rename = "bodyMD5Hash", skip_serializing_if = "Option::is_none")]
    pub body_md5_hash: Option<String>,
    #[serde(rename = "cc")]
    pub cc: Vec<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "inboxId")]
    pub inbox_id: String,
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(rename = "to")]
    pub to: Vec<String>,
    #[serde(rename = "userId")]
    pub user_id: String,
}

impl SentEmailProjection {
    pub fn new(attachments: Vec<String>, bcc: Vec<String>, cc: Vec<String>, created_at: String, id: String, inbox_id: String, to: Vec<String>, user_id: String) -> SentEmailProjection {
        SentEmailProjection {
            attachments,
            bcc,
            body_md5_hash: None,
            cc,
            created_at,
            from: None,
            id,
            inbox_id,
            subject: None,
            to,
            user_id,
        }
    }
}


