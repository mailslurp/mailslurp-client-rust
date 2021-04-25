/*
 * MailSlurp API
 *
 * MailSlurp is an API for sending and receiving emails from dynamically allocated email addresses. It's designed for developers and QA teams to test applications, process inbound emails, send templated notifications, attachments, and more.   ## Resources - [Homepage](https://www.mailslurp.com) - Get an [API KEY](https://app.mailslurp.com/sign-up/) - Generated [SDK Clients](https://www.mailslurp.com/docs/) - [Examples](https://github.com/mailslurp/examples) repository 
 *
 * The version of the OpenAPI document: 6.5.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UpdateAliasOptions : Update an email alias



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateAliasOptions {
    /// Optional name for alias
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UpdateAliasOptions {
    /// Update an email alias
    pub fn new() -> UpdateAliasOptions {
        UpdateAliasOptions {
            name: None,
        }
    }
}


