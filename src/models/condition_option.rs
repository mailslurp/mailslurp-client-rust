/*
 * MailSlurp API
 *
 * MailSlurp is an API for sending and receiving emails from dynamically allocated email addresses. It's designed for developers and QA teams to test applications, process inbound emails, send templated notifications, attachments, and more.  ## Resources  - [Homepage](https://www.mailslurp.com) - Get an [API KEY](https://app.mailslurp.com/sign-up/) - Generated [SDK Clients](https://www.mailslurp.com/docs/) - [Examples](https://github.com/mailslurp/examples) repository
 *
 * The version of the OpenAPI document: 6.5.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ConditionOption : Options for matching emails in an inbox based on a condition such as `HAS_ATTACHMENTS=TRUE`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConditionOption {
    /// The condition to evaluate against the email
    #[serde(rename = "condition", skip_serializing_if = "Option::is_none")]
    pub condition: Option<Condition>,
    /// What the condition should evaluate to. A string 'TRUE|FALSE' not a boolean.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
}

impl ConditionOption {
    /// Options for matching emails in an inbox based on a condition such as `HAS_ATTACHMENTS=TRUE`
    pub fn new() -> ConditionOption {
        ConditionOption {
            condition: None,
            value: None,
        }
    }
}

/// The condition to evaluate against the email
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Condition {
    #[serde(rename = "HAS_ATTACHMENTS")]
    HASATTACHMENTS,
}
/// What the condition should evaluate to. A string 'TRUE|FALSE' not a boolean.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Value {
    #[serde(rename = "TRUE")]
    _TRUE,
    #[serde(rename = "FALSE")]
    _FALSE,
}

