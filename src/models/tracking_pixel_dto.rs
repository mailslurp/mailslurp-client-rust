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
pub struct TrackingPixelDto {
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "html")]
    pub html: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "inboxId", skip_serializing_if = "Option::is_none")]
    pub inbox_id: Option<String>,
    #[serde(rename = "recipient", skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    #[serde(rename = "seen")]
    pub seen: bool,
    #[serde(rename = "seenAt", skip_serializing_if = "Option::is_none")]
    pub seen_at: Option<String>,
    #[serde(rename = "sentEmailId", skip_serializing_if = "Option::is_none")]
    pub sent_email_id: Option<String>,
    #[serde(rename = "url")]
    pub url: String,
}

impl TrackingPixelDto {
    pub fn new(created_at: String, html: String, id: String, seen: bool, url: String) -> TrackingPixelDto {
        TrackingPixelDto {
            created_at,
            html,
            id,
            inbox_id: None,
            recipient: None,
            seen,
            seen_at: None,
            sent_email_id: None,
            url,
        }
    }
}


