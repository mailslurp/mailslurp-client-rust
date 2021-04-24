/*
 * MailSlurp API
 *
 * MailSlurp is an API for sending and receiving emails from dynamically allocated email addresses. It's designed for developers and QA teams to test applications, process inbound emails, send templated notifications, attachments, and more.   ## Resources - [Homepage](https://www.mailslurp.com) - Get an [API KEY](https://app.mailslurp.com/sign-up/) - Generated [SDK Clients](https://www.mailslurp.com/docs/) - [Examples](https://github.com/mailslurp/examples) repository 
 *
 * The version of the OpenAPI document: 6.5.2
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method `wait_for`
#[derive(Clone, Debug)]
pub struct WaitForParams {
    /// Conditions to wait for
    pub wait_for_conditions: Option<crate::models::WaitForConditions>
}

/// struct for passing parameters to the method `wait_for_email_count`
#[derive(Clone, Debug)]
pub struct WaitForEmailCountParams {
    /// Number of emails to wait for. Must be greater that 1
    pub count: Option<i32>,
    /// Id of the inbox we are fetching emails from
    pub inbox_id: Option<String>,
    /// Max milliseconds to wait
    pub timeout: Option<i64>,
    /// Optional filter for unread only
    pub unread_only: Option<bool>
}

/// struct for passing parameters to the method `wait_for_latest_email`
#[derive(Clone, Debug)]
pub struct WaitForLatestEmailParams {
    /// Id of the inbox we are fetching emails from
    pub inbox_id: Option<String>,
    /// Max milliseconds to wait
    pub timeout: Option<i64>,
    /// Optional filter for unread only.
    pub unread_only: Option<bool>
}

/// struct for passing parameters to the method `wait_for_matching_email`
#[derive(Clone, Debug)]
pub struct WaitForMatchingEmailParams {
    /// matchOptions
    pub match_options: crate::models::MatchOptions,
    /// Number of emails to wait for. Must be greater that 1
    pub count: Option<i32>,
    /// Id of the inbox we are fetching emails from
    pub inbox_id: Option<String>,
    /// Max milliseconds to wait
    pub timeout: Option<i64>,
    /// Optional filter for unread only
    pub unread_only: Option<bool>
}

/// struct for passing parameters to the method `wait_for_matching_first_email`
#[derive(Clone, Debug)]
pub struct WaitForMatchingFirstEmailParams {
    /// matchOptions
    pub match_options: crate::models::MatchOptions,
    /// Id of the inbox we are matching an email for
    pub inbox_id: Option<String>,
    /// Max milliseconds to wait
    pub timeout: Option<i64>,
    /// Optional filter for unread only
    pub unread_only: Option<bool>
}

/// struct for passing parameters to the method `wait_for_nth_email`
#[derive(Clone, Debug)]
pub struct WaitForNthEmailParams {
    /// Id of the inbox you are fetching emails from
    pub inbox_id: Option<String>,
    /// Zero based index of the email to wait for. If an inbox has 1 email already and you want to wait for the 2nd email pass index=1
    pub index: Option<i32>,
    /// Max milliseconds to wait for the nth email if not already present
    pub timeout: Option<i64>,
    /// Optional filter for unread only
    pub unread_only: Option<bool>
}


/// struct for typed errors of method `wait_for`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WaitForError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `wait_for_email_count`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WaitForEmailCountError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `wait_for_latest_email`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WaitForLatestEmailError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `wait_for_matching_email`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WaitForMatchingEmailError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `wait_for_matching_first_email`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WaitForMatchingFirstEmailError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `wait_for_nth_email`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WaitForNthEmailError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}


/// Generic waitFor method that will wait until an inbox meets given conditions or return immediately if already met
pub async fn wait_for(configuration: &configuration::Configuration, params: WaitForParams) -> Result<Vec<crate::models::EmailPreview>, Error<WaitForError>> {
    // unbox the parameters
    let wait_for_conditions = params.wait_for_conditions;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/waitFor", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("x-api-key", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&wait_for_conditions);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<WaitForError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// If inbox contains count or more emails at time of request then return count worth of emails. If not wait until the count is reached and return those or return an error if timeout is exceeded.
pub async fn wait_for_email_count(configuration: &configuration::Configuration, params: WaitForEmailCountParams) -> Result<Vec<crate::models::EmailPreview>, Error<WaitForEmailCountError>> {
    // unbox the parameters
    let count = params.count;
    let inbox_id = params.inbox_id;
    let timeout = params.timeout;
    let unread_only = params.unread_only;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/waitForEmailCount", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = count {
        local_var_req_builder = local_var_req_builder.query(&[("count", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = inbox_id {
        local_var_req_builder = local_var_req_builder.query(&[("inboxId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = timeout {
        local_var_req_builder = local_var_req_builder.query(&[("timeout", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = unread_only {
        local_var_req_builder = local_var_req_builder.query(&[("unreadOnly", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("x-api-key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<WaitForEmailCountError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Will return either the last received email or wait for an email to arrive and return that. If you need to wait for an email for a non-empty inbox set `unreadOnly=true` or see the other receive methods such as `waitForNthEmail` or `waitForEmailCount`.
pub async fn wait_for_latest_email(configuration: &configuration::Configuration, params: WaitForLatestEmailParams) -> Result<crate::models::Email, Error<WaitForLatestEmailError>> {
    // unbox the parameters
    let inbox_id = params.inbox_id;
    let timeout = params.timeout;
    let unread_only = params.unread_only;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/waitForLatestEmail", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = inbox_id {
        local_var_req_builder = local_var_req_builder.query(&[("inboxId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = timeout {
        local_var_req_builder = local_var_req_builder.query(&[("timeout", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = unread_only {
        local_var_req_builder = local_var_req_builder.query(&[("unreadOnly", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("x-api-key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<WaitForLatestEmailError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Perform a search of emails in an inbox with the given patterns. If results match expected count then return or else retry the search until results are found or timeout is reached. Match options allow simple CONTAINS or EQUALS filtering on SUBJECT, TO, BCC, CC, and FROM. See the `MatchOptions` object for options. An example payload is `{ matches: [{field: 'SUBJECT',should:'CONTAIN',value:'needle'}] }`. You can use an array of matches and they will be applied sequentially to filter out emails. If you want to perform matches and extractions of content using Regex patterns see the EmailController `getEmailContentMatch` method.
pub async fn wait_for_matching_email(configuration: &configuration::Configuration, params: WaitForMatchingEmailParams) -> Result<Vec<crate::models::EmailPreview>, Error<WaitForMatchingEmailError>> {
    // unbox the parameters
    let match_options = params.match_options;
    let count = params.count;
    let inbox_id = params.inbox_id;
    let timeout = params.timeout;
    let unread_only = params.unread_only;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/waitForMatchingEmails", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = count {
        local_var_req_builder = local_var_req_builder.query(&[("count", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = inbox_id {
        local_var_req_builder = local_var_req_builder.query(&[("inboxId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = timeout {
        local_var_req_builder = local_var_req_builder.query(&[("timeout", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = unread_only {
        local_var_req_builder = local_var_req_builder.query(&[("unreadOnly", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("x-api-key", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&match_options);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<WaitForMatchingEmailError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Perform a search of emails in an inbox with the given patterns. If a result if found then return or else retry the search until a result is found or timeout is reached. Match options allow simple CONTAINS or EQUALS filtering on SUBJECT, TO, BCC, CC, and FROM. See the `MatchOptions` object for options. An example payload is `{ matches: [{field: 'SUBJECT',should:'CONTAIN',value:'needle'}] }`. You can use an array of matches and they will be applied sequentially to filter out emails. If you want to perform matches and extractions of content using Regex patterns see the EmailController `getEmailContentMatch` method.
pub async fn wait_for_matching_first_email(configuration: &configuration::Configuration, params: WaitForMatchingFirstEmailParams) -> Result<crate::models::Email, Error<WaitForMatchingFirstEmailError>> {
    // unbox the parameters
    let match_options = params.match_options;
    let inbox_id = params.inbox_id;
    let timeout = params.timeout;
    let unread_only = params.unread_only;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/waitForMatchingFirstEmail", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = inbox_id {
        local_var_req_builder = local_var_req_builder.query(&[("inboxId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = timeout {
        local_var_req_builder = local_var_req_builder.query(&[("timeout", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = unread_only {
        local_var_req_builder = local_var_req_builder.query(&[("unreadOnly", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("x-api-key", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&match_options);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<WaitForMatchingFirstEmailError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// If nth email is already present in inbox then return it. If not hold the connection open until timeout expires or the nth email is received and returned.
pub async fn wait_for_nth_email(configuration: &configuration::Configuration, params: WaitForNthEmailParams) -> Result<crate::models::Email, Error<WaitForNthEmailError>> {
    // unbox the parameters
    let inbox_id = params.inbox_id;
    let index = params.index;
    let timeout = params.timeout;
    let unread_only = params.unread_only;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/waitForNthEmail", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = inbox_id {
        local_var_req_builder = local_var_req_builder.query(&[("inboxId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = index {
        local_var_req_builder = local_var_req_builder.query(&[("index", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = timeout {
        local_var_req_builder = local_var_req_builder.query(&[("timeout", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = unread_only {
        local_var_req_builder = local_var_req_builder.query(&[("unreadOnly", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("x-api-key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<WaitForNthEmailError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

