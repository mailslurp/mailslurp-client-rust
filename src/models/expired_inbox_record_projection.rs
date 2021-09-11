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
pub struct ExpiredInboxRecordProjection {
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "emailAddress")]
    pub email_address: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}

impl ExpiredInboxRecordProjection {
    pub fn new(created_at: String, email_address: String, id: String, user_id: String) -> ExpiredInboxRecordProjection {
        ExpiredInboxRecordProjection {
            created_at,
            email_address,
            id,
            user_id,
        }
    }
}


