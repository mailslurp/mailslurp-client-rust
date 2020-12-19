/*
 * MailSlurp API
 *
 * MailSlurp is an API for sending and receiving emails from dynamically allocated email addresses. It's designed for developers and QA teams to test applications, process inbound emails, send templated notifications, attachments, and more.   ## Resources - [Homepage](https://www.mailslurp.com) - Get an [API KEY](https://app.mailslurp.com/sign-up/) - Generated [SDK Clients](https://www.mailslurp.com/docs/) - [Examples](https://github.com/mailslurp/examples) repository 
 *
 * The version of the OpenAPI document: 6.5.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreateWebhookOptions : Options for creating a webhook. Webhooks can be attached to inboxes and MailSlurp will POST a webhook payload to the URL specified whenever the inbox receives an email. Webhooks are great for processing many inbound emails.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateWebhookOptions {
    #[serde(rename = "basicAuth", skip_serializing_if = "Option::is_none")]
    pub basic_auth: Option<crate::models::BasicAuthOptions>,
    /// Optional name for the webhook
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Public URL on your server that MailSlurp can post WebhookNotification payload to when an email is received. The payload of the submitted JSON is described by https://api.mailslurp.com/schemas/webhook-payload
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl CreateWebhookOptions {
    /// Options for creating a webhook. Webhooks can be attached to inboxes and MailSlurp will POST a webhook payload to the URL specified whenever the inbox receives an email. Webhooks are great for processing many inbound emails.
    pub fn new() -> CreateWebhookOptions {
        CreateWebhookOptions {
            basic_auth: None,
            name: None,
            url: None,
        }
    }
}


