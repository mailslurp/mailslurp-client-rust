/*
 * MailSlurp API
 *
 * MailSlurp is an API for sending and receiving emails from dynamically allocated email addresses. It's designed for developers and QA teams to test applications, process inbound emails, send templated notifications, attachments, and more.  ## Resources  - [Homepage](https://www.mailslurp.com) - Get an [API KEY](https://app.mailslurp.com/sign-up/) - Generated [SDK Clients](https://www.mailslurp.com/docs/) - [Examples](https://github.com/mailslurp/examples) repository 
 *
 * The version of the OpenAPI document: 6.5.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PageEmailPreview : Paginated email preview results. EmailProjections and EmailPreviews are essentially the same but have legacy naming issues. Page index starts at zero. Projection results may omit larger entity fields. For fetching a full entity use the projection ID with individual method calls. For emails there are several methods for fetching message bodies and attachments.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PageEmailPreview {
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<crate::models::EmailPreview>>,
    #[serde(rename = "empty", skip_serializing_if = "Option::is_none")]
    pub empty: Option<bool>,
    #[serde(rename = "first", skip_serializing_if = "Option::is_none")]
    pub first: Option<bool>,
    #[serde(rename = "last", skip_serializing_if = "Option::is_none")]
    pub last: Option<bool>,
    #[serde(rename = "number", skip_serializing_if = "Option::is_none")]
    pub number: Option<i32>,
    #[serde(rename = "numberOfElements", skip_serializing_if = "Option::is_none")]
    pub number_of_elements: Option<i32>,
    #[serde(rename = "pageable", skip_serializing_if = "Option::is_none")]
    pub pageable: Option<Box<crate::models::Pageable>>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "sort", skip_serializing_if = "Option::is_none")]
    pub sort: Option<Box<crate::models::Sort>>,
    #[serde(rename = "totalElements", skip_serializing_if = "Option::is_none")]
    pub total_elements: Option<i64>,
    #[serde(rename = "totalPages", skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<i32>,
}

impl PageEmailPreview {
    /// Paginated email preview results. EmailProjections and EmailPreviews are essentially the same but have legacy naming issues. Page index starts at zero. Projection results may omit larger entity fields. For fetching a full entity use the projection ID with individual method calls. For emails there are several methods for fetching message bodies and attachments.
    pub fn new() -> PageEmailPreview {
        PageEmailPreview {
            content: None,
            empty: None,
            first: None,
            last: None,
            number: None,
            number_of_elements: None,
            pageable: None,
            size: None,
            sort: None,
            total_elements: None,
            total_pages: None,
        }
    }
}


