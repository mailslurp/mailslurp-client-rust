/*
 * MailSlurp API
 *
 * MailSlurp is an API for sending and receiving emails from dynamically allocated email addresses. It's designed for developers and QA teams to test applications, process inbound emails, send templated notifications, attachments, and more.  ## Resources  - [Homepage](https://www.mailslurp.com) - Get an [API KEY](https://app.mailslurp.com/sign-up/) - Generated [SDK Clients](https://www.mailslurp.com/docs/) - [Examples](https://github.com/mailslurp/examples) repository 
 *
 * The version of the OpenAPI document: 6.5.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WebhookNewContactPayload : NEW_CONTACT webhook payload. Sent to your webhook url endpoint via HTTP POST when an email is received by the inbox that your webhook is attached to that contains a recipient that has not been saved as a contact.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookNewContactPayload {
    #[serde(rename = "company", skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(rename = "contactId")]
    pub contact_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "emailAddresses")]
    pub email_addresses: Vec<String>,
    /// Name of the event type webhook is being triggered for.
    #[serde(rename = "eventName", skip_serializing_if = "Option::is_none")]
    pub event_name: Option<EventName>,
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Idempotent message ID. Store this ID locally or in a database to prevent message duplication.
    #[serde(rename = "messageId", skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(rename = "metaData", skip_serializing_if = "Option::is_none")]
    pub meta_data: Option<serde_json::Value>,
    #[serde(rename = "optOut", skip_serializing_if = "Option::is_none")]
    pub opt_out: Option<bool>,
    #[serde(rename = "primaryEmailAddress", skip_serializing_if = "Option::is_none")]
    pub primary_email_address: Option<String>,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    /// ID of webhook entity being triggered
    #[serde(rename = "webhookId", skip_serializing_if = "Option::is_none")]
    pub webhook_id: Option<String>,
    /// Name of the webhook being triggered
    #[serde(rename = "webhookName", skip_serializing_if = "Option::is_none")]
    pub webhook_name: Option<String>,
}

impl WebhookNewContactPayload {
    /// NEW_CONTACT webhook payload. Sent to your webhook url endpoint via HTTP POST when an email is received by the inbox that your webhook is attached to that contains a recipient that has not been saved as a contact.
    pub fn new(contact_id: String, created_at: String, email_addresses: Vec<String>, tags: Vec<String>) -> WebhookNewContactPayload {
        WebhookNewContactPayload {
            company: None,
            contact_id,
            created_at,
            email_addresses,
            event_name: None,
            first_name: None,
            group_id: None,
            last_name: None,
            message_id: None,
            meta_data: None,
            opt_out: None,
            primary_email_address: None,
            tags,
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
}

