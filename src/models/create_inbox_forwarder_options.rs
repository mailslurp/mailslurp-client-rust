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
pub struct CreateInboxForwarderOptions {
    #[serde(rename = "field")]
    pub field: Field,
    #[serde(rename = "match")]
    pub _match: String,
    #[serde(rename = "forwardToRecipients")]
    pub forward_to_recipients: Vec<String>,
}

impl CreateInboxForwarderOptions {
    pub fn new(field: Field, _match: String, forward_to_recipients: Vec<String>) -> CreateInboxForwarderOptions {
        CreateInboxForwarderOptions {
            field,
            _match,
            forward_to_recipients,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Field {
    #[serde(rename = "RECIPIENTS")]
    RECIPIENTS,
    #[serde(rename = "SENDER")]
    SENDER,
    #[serde(rename = "SUBJECT")]
    SUBJECT,
    #[serde(rename = "ATTACHMENTS")]
    ATTACHMENTS,
}

