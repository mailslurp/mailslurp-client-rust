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
pub struct InboxProjection {
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "emailAddress", skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(rename = "favourite")]
    pub favourite: bool,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "inboxType", skip_serializing_if = "Option::is_none")]
    pub inbox_type: Option<InboxType>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "teamAccess")]
    pub team_access: bool,
}

impl InboxProjection {
    pub fn new(created_at: String, favourite: bool, id: String, team_access: bool) -> InboxProjection {
        InboxProjection {
            created_at,
            email_address: None,
            favourite,
            id,
            inbox_type: None,
            name: None,
            tags: None,
            team_access,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InboxType {
    #[serde(rename = "HTTP_INBOX")]
    HTTPINBOX,
    #[serde(rename = "SMTP_INBOX")]
    SMTPINBOX,
}

