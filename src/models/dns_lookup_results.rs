/*
 * MailSlurp API
 *
 * MailSlurp is an API for sending and receiving emails from dynamically allocated email addresses. It's designed for developers and QA teams to test applications, process inbound emails, send templated notifications, attachments, and more.  ## Resources  - [Homepage](https://www.mailslurp.com) - Get an [API KEY](https://app.mailslurp.com/sign-up/) - Generated [SDK Clients](https://www.mailslurp.com/docs/) - [Examples](https://github.com/mailslurp/examples) repository
 *
 * The version of the OpenAPI document: 6.5.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DnsLookupResults : Results of query on domain name servers



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DnsLookupResults {
    #[serde(rename = "results")]
    pub results: Vec<crate::models::DnsLookupResult>,
}

impl DnsLookupResults {
    /// Results of query on domain name servers
    pub fn new(results: Vec<crate::models::DnsLookupResult>) -> DnsLookupResults {
        DnsLookupResults {
            results,
        }
    }
}


