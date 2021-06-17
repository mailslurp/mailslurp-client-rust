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
pub struct WebhookResultEntity {
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "httpMethod")]
    pub http_method: HttpMethod,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "inboxId")]
    pub inbox_id: String,
    #[serde(rename = "messageId")]
    pub message_id: String,
    #[serde(rename = "responseBodyExtract", skip_serializing_if = "Option::is_none")]
    pub response_body_extract: Option<String>,
    #[serde(rename = "responseStatus", skip_serializing_if = "Option::is_none")]
    pub response_status: Option<i32>,
    #[serde(rename = "responseTimeMillis")]
    pub response_time_millis: i64,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "webhookEvent")]
    pub webhook_event: WebhookEvent,
    #[serde(rename = "webhookId")]
    pub webhook_id: String,
    #[serde(rename = "webhookUrl")]
    pub webhook_url: String,
}

impl WebhookResultEntity {
    pub fn new(created_at: String, http_method: HttpMethod, inbox_id: String, message_id: String, response_time_millis: i64, updated_at: String, user_id: String, webhook_event: WebhookEvent, webhook_id: String, webhook_url: String) -> WebhookResultEntity {
        WebhookResultEntity {
            created_at,
            http_method,
            id: None,
            inbox_id,
            message_id,
            response_body_extract: None,
            response_status: None,
            response_time_millis,
            updated_at,
            user_id,
            webhook_event,
            webhook_id,
            webhook_url,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum HttpMethod {
    #[serde(rename = "GET")]
    GET,
    #[serde(rename = "HEAD")]
    HEAD,
    #[serde(rename = "POST")]
    POST,
    #[serde(rename = "PUT")]
    PUT,
    #[serde(rename = "PATCH")]
    PATCH,
    #[serde(rename = "DELETE")]
    DELETE,
    #[serde(rename = "OPTIONS")]
    OPTIONS,
    #[serde(rename = "TRACE")]
    TRACE,
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WebhookEvent {
    #[serde(rename = "EMAIL_RECEIVED")]
    EMAILRECEIVED,
    #[serde(rename = "NEW_EMAIL")]
    NEWEMAIL,
    #[serde(rename = "NEW_CONTACT")]
    NEWCONTACT,
    #[serde(rename = "NEW_ATTACHMENT")]
    NEWATTACHMENT,
}

