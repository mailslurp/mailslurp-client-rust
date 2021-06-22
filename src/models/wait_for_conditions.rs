/*
 * MailSlurp API
 *
 * MailSlurp is an API for sending and receiving emails from dynamically allocated email addresses. It's designed for developers and QA teams to test applications, process inbound emails, send templated notifications, attachments, and more.  ## Resources  - [Homepage](https://www.mailslurp.com) - Get an [API KEY](https://app.mailslurp.com/sign-up/) - Generated [SDK Clients](https://www.mailslurp.com/docs/) - [Examples](https://github.com/mailslurp/examples) repository 
 *
 * The version of the OpenAPI document: 6.5.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WaitForConditions : Conditions that a `waitForXEmails` endpoint operates on. The methods wait until given conditions are met or a timeout is reached. If the conditions are met without needing to wait the results will be returned immediately. Can include `unreadOnly` to ignore already read emails that were returned in an API call or viewing in the dashboard. Can also include matches for emails containing `from`, `subject`, `hasAttachments` etc.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WaitForConditions {
    /// Number of results that should match conditions. Either exactly or at least this amount based on the `countType`. If count condition is not met and the timeout has not been reached the `waitFor` method will retry the operation.
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// How should the found count be compared to the expected count.
    #[serde(rename = "countType", skip_serializing_if = "Option::is_none")]
    pub count_type: Option<CountType>,
    /// ID of inbox to search within and apply conditions to. Essentially filtering the emails found to give a count.
    #[serde(rename = "inboxId", skip_serializing_if = "Option::is_none")]
    pub inbox_id: Option<String>,
    /// Conditions that should be matched for an email to qualify for results. Each condition will be applied in order to each email within an inbox to filter a result list of matching emails you are waiting for.
    #[serde(rename = "matches", skip_serializing_if = "Option::is_none")]
    pub matches: Option<Vec<crate::models::MatchOption>>,
    /// Direction to sort matching emails by created time
    #[serde(rename = "sortDirection", skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<SortDirection>,
    /// Max time in milliseconds to retry the `waitFor` operation until conditions are met.
    #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// Apply conditions only to **unread** emails. All emails begin with `read=false`. An email is marked `read=true` when an `EmailDto` representation of it has been returned to the user at least once. For example you have called `getEmail` or `waitForLatestEmail` etc., or you have viewed the email in the dashboard. 
    #[serde(rename = "unreadOnly", skip_serializing_if = "Option::is_none")]
    pub unread_only: Option<bool>,
}

impl WaitForConditions {
    /// Conditions that a `waitForXEmails` endpoint operates on. The methods wait until given conditions are met or a timeout is reached. If the conditions are met without needing to wait the results will be returned immediately. Can include `unreadOnly` to ignore already read emails that were returned in an API call or viewing in the dashboard. Can also include matches for emails containing `from`, `subject`, `hasAttachments` etc.
    pub fn new() -> WaitForConditions {
        WaitForConditions {
            count: None,
            count_type: None,
            inbox_id: None,
            matches: None,
            sort_direction: None,
            timeout: None,
            unread_only: None,
        }
    }
}

/// How should the found count be compared to the expected count.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CountType {
    #[serde(rename = "EXACTLY")]
    EXACTLY,
    #[serde(rename = "ATLEAST")]
    ATLEAST,
}
/// Direction to sort matching emails by created time
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SortDirection {
    #[serde(rename = "ASC")]
    ASC,
    #[serde(rename = "DESC")]
    DESC,
}

