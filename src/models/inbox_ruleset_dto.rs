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
pub struct InboxRulesetDto {
    #[serde(rename = "action")]
    pub action: Action,
    #[serde(rename = "handler")]
    pub handler: Handler,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "inboxId")]
    pub inbox_id: String,
    #[serde(rename = "scope")]
    pub scope: Scope,
    #[serde(rename = "target")]
    pub target: String,
}

impl InboxRulesetDto {
    pub fn new(action: Action, handler: Handler, id: String, inbox_id: String, scope: Scope, target: String) -> InboxRulesetDto {
        InboxRulesetDto {
            action,
            handler,
            id,
            inbox_id,
            scope,
            target,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "BLOCK")]
    BLOCK,
    #[serde(rename = "ALLOW")]
    ALLOW,
    #[serde(rename = "FORWARD")]
    FORWARD,
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Handler {
    #[serde(rename = "EXCEPTION")]
    EXCEPTION,
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Scope {
    #[serde(rename = "RECEIVING_EMAILS")]
    RECEIVINGEMAILS,
    #[serde(rename = "SENDING_EMAILS")]
    SENDINGEMAILS,
}

