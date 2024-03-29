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
pub struct ContactDto {
    #[serde(rename = "company", skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "emailAddresses")]
    pub email_addresses: Vec<String>,
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(rename = "metaData", skip_serializing_if = "Option::is_none")]
    pub meta_data: Option<serde_json::Value>,
    #[serde(rename = "optOut", skip_serializing_if = "Option::is_none")]
    pub opt_out: Option<bool>,
    #[serde(rename = "primaryEmailAddress", skip_serializing_if = "Option::is_none")]
    pub primary_email_address: Option<String>,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
}

impl ContactDto {
    pub fn new(created_at: String, email_addresses: Vec<String>, id: String, tags: Vec<String>) -> ContactDto {
        ContactDto {
            company: None,
            created_at,
            email_addresses,
            first_name: None,
            group_id: None,
            id,
            last_name: None,
            meta_data: None,
            opt_out: None,
            primary_email_address: None,
            tags,
        }
    }
}


