/*
 * MailSlurp API
 *
 * MailSlurp is an API for sending and receiving emails from dynamically allocated email addresses. It's designed for developers and QA teams to test applications, process inbound emails, send templated notifications, attachments, and more.  ## Resources  - [Homepage](https://www.mailslurp.com) - Get an [API KEY](https://app.mailslurp.com/sign-up/) - Generated [SDK Clients](https://www.mailslurp.com/docs/) - [Examples](https://github.com/mailslurp/examples) repository
 *
 * The version of the OpenAPI document: 6.5.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ExportLink : Export download link



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportLink {
    #[serde(rename = "downloadLink")]
    pub download_link: String,
}

impl ExportLink {
    /// Export download link
    pub fn new(download_link: String) -> ExportLink {
        ExportLink {
            download_link,
        }
    }
}


