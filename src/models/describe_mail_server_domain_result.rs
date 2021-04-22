/*
 * MailSlurp API
 *
 * MailSlurp is an API for sending and receiving emails from dynamically allocated email addresses. It's designed for developers and QA teams to test applications, process inbound emails, send templated notifications, attachments, and more.   ## Resources - [Homepage](https://www.mailslurp.com) - Get an [API KEY](https://app.mailslurp.com/sign-up/) - Generated [SDK Clients](https://www.mailslurp.com/docs/) - [Examples](https://github.com/mailslurp/examples) repository 
 *
 * The version of the OpenAPI document: 6.5.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DescribeMailServerDomainResult : Name Server lookup result



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DescribeMailServerDomainResult {
    #[serde(rename = "domain")]
    pub domain: String,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "mxRecords")]
    pub mx_records: Vec<crate::models::NameServerRecord>,
}

impl DescribeMailServerDomainResult {
    /// Name Server lookup result
    pub fn new(domain: String, mx_records: Vec<crate::models::NameServerRecord>) -> DescribeMailServerDomainResult {
        DescribeMailServerDomainResult {
            domain,
            message: None,
            mx_records,
        }
    }
}


