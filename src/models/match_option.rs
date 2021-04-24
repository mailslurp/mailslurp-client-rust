/*
 * MailSlurp API
 *
 * MailSlurp is an API for sending and receiving emails from dynamically allocated email addresses. It's designed for developers and QA teams to test applications, process inbound emails, send templated notifications, attachments, and more.   ## Resources - [Homepage](https://www.mailslurp.com) - Get an [API KEY](https://app.mailslurp.com/sign-up/) - Generated [SDK Clients](https://www.mailslurp.com/docs/) - [Examples](https://github.com/mailslurp/examples) repository 
 *
 * The version of the OpenAPI document: 6.5.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// MatchOption : Options for matching emails in an inbox. Each match option object contains a `field`, `should` and `value` property. Together they form logical conditions such as `SUBJECT` should `CONTAIN` value.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MatchOption {
    /// The email property to match on. One of SUBJECT, TO, BCC, CC or FROM
    #[serde(rename = "field", skip_serializing_if = "Option::is_none")]
    pub field: Option<Field>,
    /// What criteria to apply. CONTAIN or EQUAL. Note CONTAIN is recommended due to some SMTP servers adding new lines to fields and body content.
    #[serde(rename = "should", skip_serializing_if = "Option::is_none")]
    pub should: Option<Should>,
    /// The value you wish to compare with the value of the field specified using the `should` value passed. For example `BODY` should `CONTAIN` a value passed.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl MatchOption {
    /// Options for matching emails in an inbox. Each match option object contains a `field`, `should` and `value` property. Together they form logical conditions such as `SUBJECT` should `CONTAIN` value.
    pub fn new() -> MatchOption {
        MatchOption {
            field: None,
            should: None,
            value: None,
        }
    }
}

/// The email property to match on. One of SUBJECT, TO, BCC, CC or FROM
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Field {
    #[serde(rename = "SUBJECT")]
    SUBJECT,
    #[serde(rename = "TO")]
    TO,
    #[serde(rename = "BCC")]
    BCC,
    #[serde(rename = "CC")]
    CC,
    #[serde(rename = "FROM")]
    FROM,
}
/// What criteria to apply. CONTAIN or EQUAL. Note CONTAIN is recommended due to some SMTP servers adding new lines to fields and body content.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Should {
    #[serde(rename = "CONTAIN")]
    CONTAIN,
    #[serde(rename = "EQUAL")]
    EQUAL,
}

