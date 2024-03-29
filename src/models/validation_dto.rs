/*
 * MailSlurp API
 *
 * MailSlurp is an API for sending and receiving emails from dynamically allocated email addresses. It's designed for developers and QA teams to test applications, process inbound emails, send templated notifications, attachments, and more.  ## Resources  - [Homepage](https://www.mailslurp.com) - Get an [API KEY](https://app.mailslurp.com/sign-up/) - Generated [SDK Clients](https://www.mailslurp.com/docs/) - [Examples](https://github.com/mailslurp/examples) repository
 *
 * The version of the OpenAPI document: 6.5.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ValidationDto : Response object for email validation operation



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidationDto {
    /// ID of the email validated
    #[serde(rename = "emailId", skip_serializing_if = "Option::is_none")]
    pub email_id: Option<String>,
    #[serde(rename = "html", skip_serializing_if = "Option::is_none")]
    pub html: Option<Box<crate::models::HtmlValidationResult>>,
}

impl ValidationDto {
    /// Response object for email validation operation
    pub fn new() -> ValidationDto {
        ValidationDto {
            email_id: None,
            html: None,
        }
    }
}


