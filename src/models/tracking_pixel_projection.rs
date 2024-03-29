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
pub struct TrackingPixelProjection {
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "inboxId", skip_serializing_if = "Option::is_none")]
    pub inbox_id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "recipient", skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    #[serde(rename = "seen")]
    pub seen: bool,
    #[serde(rename = "seenAt", skip_serializing_if = "Option::is_none")]
    pub seen_at: Option<String>,
    #[serde(rename = "sentEmailId", skip_serializing_if = "Option::is_none")]
    pub sent_email_id: Option<String>,
    #[serde(rename = "userId")]
    pub user_id: String,
}

impl TrackingPixelProjection {
    pub fn new(created_at: String, id: String, seen: bool, user_id: String) -> TrackingPixelProjection {
        TrackingPixelProjection {
            created_at,
            id,
            inbox_id: None,
            name: None,
            recipient: None,
            seen,
            seen_at: None,
            sent_email_id: None,
            user_id,
        }
    }
}


