/*
 * MailSlurp API
 *
 * MailSlurp is an API for sending and receiving emails from dynamically allocated email addresses. It's designed for developers and QA teams to test applications, process inbound emails, send templated notifications, attachments, and more.  ## Resources  - [Homepage](https://www.mailslurp.com) - Get an [API KEY](https://app.mailslurp.com/sign-up/) - Generated [SDK Clients](https://www.mailslurp.com/docs/) - [Examples](https://github.com/mailslurp/examples) repository
 *
 * The version of the OpenAPI document: 6.5.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WebhookEmailOpenedPayload : EMAIL_OPENED webhook payload. Sent to your webhook url endpoint via HTTP POST when an email containing a tracking pixel is opened and the pixel image is loaded by a reader.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookEmailOpenedPayload {
    /// Date time of event creation
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Name of the event type webhook is being triggered for.
    #[serde(rename = "eventName", skip_serializing_if = "Option::is_none")]
    pub event_name: Option<EventName>,
    /// Id of the inbox that received an email
    #[serde(rename = "inboxId", skip_serializing_if = "Option::is_none")]
    pub inbox_id: Option<String>,
    /// Idempotent message ID. Store this ID locally or in a database to prevent message duplication.
    #[serde(rename = "messageId", skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// ID of the tracking pixel
    #[serde(rename = "pixelId", skip_serializing_if = "Option::is_none")]
    pub pixel_id: Option<String>,
    /// Email address for the recipient of the tracking pixel
    #[serde(rename = "recipient", skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    /// ID of sent email
    #[serde(rename = "sentEmailId", skip_serializing_if = "Option::is_none")]
    pub sent_email_id: Option<String>,
    /// ID of webhook entity being triggered
    #[serde(rename = "webhookId", skip_serializing_if = "Option::is_none")]
    pub webhook_id: Option<String>,
    /// Name of the webhook being triggered
    #[serde(rename = "webhookName", skip_serializing_if = "Option::is_none")]
    pub webhook_name: Option<String>,
}

impl WebhookEmailOpenedPayload {
    /// EMAIL_OPENED webhook payload. Sent to your webhook url endpoint via HTTP POST when an email containing a tracking pixel is opened and the pixel image is loaded by a reader.
    pub fn new() -> WebhookEmailOpenedPayload {
        WebhookEmailOpenedPayload {
            created_at: None,
            event_name: None,
            inbox_id: None,
            message_id: None,
            pixel_id: None,
            recipient: None,
            sent_email_id: None,
            webhook_id: None,
            webhook_name: None,
        }
    }
}

/// Name of the event type webhook is being triggered for.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventName {
    #[serde(rename = "EMAIL_RECEIVED")]
    EMAILRECEIVED,
    #[serde(rename = "NEW_EMAIL")]
    NEWEMAIL,
    #[serde(rename = "NEW_CONTACT")]
    NEWCONTACT,
    #[serde(rename = "NEW_ATTACHMENT")]
    NEWATTACHMENT,
    #[serde(rename = "EMAIL_OPENED")]
    EMAILOPENED,
    #[serde(rename = "EMAIL_READ")]
    EMAILREAD,
}

