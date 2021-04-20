/*
 * MailSlurp API
 *
 * MailSlurp is an API for sending and receiving emails from dynamically allocated email addresses. It's designed for developers and QA teams to test applications, process inbound emails, send templated notifications, attachments, and more.   ## Resources - [Homepage](https://www.mailslurp.com) - Get an [API KEY](https://app.mailslurp.com/sign-up/) - Generated [SDK Clients](https://www.mailslurp.com/docs/) - [Examples](https://github.com/mailslurp/examples) repository 
 *
 * The version of the OpenAPI document: 6.5.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// MatchOptions : Optional filter for matching emails based on fields. For instance filter results to only include emails whose `SUBJECT` value does `CONTAIN` given match value. An example payload would be `{ matches: [{ field: 'SUBJECT', should: 'CONTAIN', value: 'Welcome' }] }`. If you wish to extract regex matches inside the email content see the `getEmailContentMatch` method in the EmailController.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MatchOptions {
    /// 1 or more match options. Options are additive so if one does not match the email is excluded from results
    #[serde(rename = "matches", skip_serializing_if = "Option::is_none")]
    pub matches: Option<Vec<crate::models::MatchOption>>,
}

impl MatchOptions {
    /// Optional filter for matching emails based on fields. For instance filter results to only include emails whose `SUBJECT` value does `CONTAIN` given match value. An example payload would be `{ matches: [{ field: 'SUBJECT', should: 'CONTAIN', value: 'Welcome' }] }`. If you wish to extract regex matches inside the email content see the `getEmailContentMatch` method in the EmailController.
    pub fn new() -> MatchOptions {
        MatchOptions {
            matches: None,
        }
    }
}


