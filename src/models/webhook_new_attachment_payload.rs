/*
 * MailSlurp API
 *
 * MailSlurp is an API for sending and receiving emails from dynamically allocated email addresses. It's designed for developers and QA teams to test applications, process inbound emails, send templated notifications, attachments, and more.  ## Resources  - [Homepage](https://www.mailslurp.com) - Get an [API KEY](https://app.mailslurp.com/sign-up/) - Generated [SDK Clients](https://www.mailslurp.com/docs/) - [Examples](https://github.com/mailslurp/examples) repository
 *
 * The version of the OpenAPI document: 6.5.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WebhookNewAttachmentPayload : NEW_ATTACHMENT webhook payload. Sent to your webhook url endpoint via HTTP POST when an email is received by the inbox that your webhook is attached to that contains an attachment. You can use the attachmentId to download the attachment.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookNewAttachmentPayload {
    /// ID of attachment. Use the `AttachmentController` to
    #[serde(rename = "attachmentId", skip_serializing_if = "Option::is_none")]
    pub attachment_id: Option<String>,
    /// Size of attachment in bytes
    #[serde(rename = "contentLength", skip_serializing_if = "Option::is_none")]
    pub content_length: Option<i64>,
    /// Content type of attachment such as 'image/png' or 'application/pdf
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// Name of the event type webhook is being triggered for.
    #[serde(rename = "eventName", skip_serializing_if = "Option::is_none")]
    pub event_name: Option<EventName>,
    /// Idempotent message ID. Store this ID locally or in a database to prevent message duplication.
    #[serde(rename = "messageId", skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// Filename of the attachment if present
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// ID of webhook entity being triggered
    #[serde(rename = "webhookId", skip_serializing_if = "Option::is_none")]
    pub webhook_id: Option<String>,
    /// Name of the webhook being triggered
    #[serde(rename = "webhookName", skip_serializing_if = "Option::is_none")]
    pub webhook_name: Option<String>,
}

impl WebhookNewAttachmentPayload {
    /// NEW_ATTACHMENT webhook payload. Sent to your webhook url endpoint via HTTP POST when an email is received by the inbox that your webhook is attached to that contains an attachment. You can use the attachmentId to download the attachment.
    pub fn new() -> WebhookNewAttachmentPayload {
        WebhookNewAttachmentPayload {
            attachment_id: None,
            content_length: None,
            content_type: None,
            event_name: None,
            message_id: None,
            name: None,
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

