/*
 * MailSlurp API
 *
 * MailSlurp is an API for sending and receiving emails from dynamically allocated email addresses. It's designed for developers and QA teams to test applications, process inbound emails, send templated notifications, attachments, and more.  ## Resources  - [Homepage](https://www.mailslurp.com) - Get an [API KEY](https://app.mailslurp.com/sign-up/) - Generated [SDK Clients](https://www.mailslurp.com/docs/) - [Examples](https://github.com/mailslurp/examples) repository 
 *
 * The version of the OpenAPI document: 6.5.2
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method `create_new_inbox_ruleset`
#[derive(Clone, Debug)]
pub struct CreateNewInboxRulesetParams {
    /// createInboxRulesetOptions
    pub create_inbox_ruleset_options: crate::models::CreateInboxRulesetOptions,
    /// Inbox id to attach ruleset to
    pub inbox_id: Option<String>
}

/// struct for passing parameters to the method `delete_inbox_ruleset`
#[derive(Clone, Debug)]
pub struct DeleteInboxRulesetParams {
    /// ID of inbox ruleset
    pub id: String
}

/// struct for passing parameters to the method `delete_inbox_rulesets`
#[derive(Clone, Debug)]
pub struct DeleteInboxRulesetsParams {
    /// Optional inbox id to attach ruleset to
    pub inbox_id: Option<String>
}

/// struct for passing parameters to the method `get_inbox_ruleset`
#[derive(Clone, Debug)]
pub struct GetInboxRulesetParams {
    /// ID of inbox ruleset
    pub id: String
}

/// struct for passing parameters to the method `get_inbox_rulesets`
#[derive(Clone, Debug)]
pub struct GetInboxRulesetsParams {
    /// Optional inbox id to get rulesets from
    pub inbox_id: Option<String>,
    /// Optional page index in inbox ruleset list pagination
    pub page: Option<i32>,
    /// Optional search filter
    pub search_filter: Option<String>,
    /// Optional page size in inbox ruleset list pagination
    pub size: Option<i32>,
    /// Optional createdAt sort direction ASC or DESC
    pub sort: Option<String>
}

/// struct for passing parameters to the method `test_inbox_ruleset`
#[derive(Clone, Debug)]
pub struct TestInboxRulesetParams {
    /// ID of inbox ruleset
    pub id: String,
    /// inboxRulesetTestOptions
    pub inbox_ruleset_test_options: crate::models::InboxRulesetTestOptions
}

/// struct for passing parameters to the method `test_inbox_rulesets_for_inbox`
#[derive(Clone, Debug)]
pub struct TestInboxRulesetsForInboxParams {
    /// ID of inbox
    pub inbox_id: String,
    /// inboxRulesetTestOptions
    pub inbox_ruleset_test_options: crate::models::InboxRulesetTestOptions
}

/// struct for passing parameters to the method `test_new_inbox_ruleset`
#[derive(Clone, Debug)]
pub struct TestNewInboxRulesetParams {
    /// testNewInboxRulesetOptions
    pub test_new_inbox_ruleset_options: crate::models::TestNewInboxRulesetOptions
}


/// struct for typed errors of method `create_new_inbox_ruleset`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateNewInboxRulesetError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `delete_inbox_ruleset`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteInboxRulesetError {
    Status401(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `delete_inbox_rulesets`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteInboxRulesetsError {
    Status401(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_inbox_ruleset`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetInboxRulesetError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_inbox_rulesets`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetInboxRulesetsError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `test_inbox_ruleset`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TestInboxRulesetError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `test_inbox_rulesets_for_inbox`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TestInboxRulesetsForInboxError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `test_new_inbox_ruleset`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TestNewInboxRulesetError {
    Status401(),
    Status403(),
    UnknownValue(serde_json::Value),
}


/// Create a new inbox rule for forwarding, blocking, and allowing emails when sending and receiving
pub async fn create_new_inbox_ruleset(configuration: &configuration::Configuration, params: CreateNewInboxRulesetParams) -> Result<crate::models::InboxRulesetDto, Error<CreateNewInboxRulesetError>> {
    // unbox the parameters
    let create_inbox_ruleset_options = params.create_inbox_ruleset_options;
    let inbox_id = params.inbox_id;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/rulesets", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = inbox_id {
        local_var_req_builder = local_var_req_builder.query(&[("inboxId", &local_var_str.to_string())]);
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
    local_var_req_builder = local_var_req_builder.json(&create_inbox_ruleset_options);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateNewInboxRulesetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete inbox ruleset
pub async fn delete_inbox_ruleset(configuration: &configuration::Configuration, params: DeleteInboxRulesetParams) -> Result<(), Error<DeleteInboxRulesetError>> {
    // unbox the parameters
    let id = params.id;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/rulesets/{id}", configuration.base_path, id=id);
    let mut local_var_req_builder = local_var_client.delete(local_var_uri_str.as_str());

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
        Ok(())
    } else {
        let local_var_entity: Option<DeleteInboxRulesetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete inbox rulesets. Accepts optional inboxId filter.
pub async fn delete_inbox_rulesets(configuration: &configuration::Configuration, params: DeleteInboxRulesetsParams) -> Result<(), Error<DeleteInboxRulesetsError>> {
    // unbox the parameters
    let inbox_id = params.inbox_id;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/rulesets", configuration.base_path);
    let mut local_var_req_builder = local_var_client.delete(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = inbox_id {
        local_var_req_builder = local_var_req_builder.query(&[("inboxId", &local_var_str.to_string())]);
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
        Ok(())
    } else {
        let local_var_entity: Option<DeleteInboxRulesetsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get inbox ruleset
pub async fn get_inbox_ruleset(configuration: &configuration::Configuration, params: GetInboxRulesetParams) -> Result<crate::models::InboxRulesetDto, Error<GetInboxRulesetError>> {
    // unbox the parameters
    let id = params.id;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/rulesets/{id}", configuration.base_path, id=id);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

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
        let local_var_entity: Option<GetInboxRulesetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all rulesets attached to an inbox
pub async fn get_inbox_rulesets(configuration: &configuration::Configuration, params: GetInboxRulesetsParams) -> Result<crate::models::PageInboxRulesetDto, Error<GetInboxRulesetsError>> {
    // unbox the parameters
    let inbox_id = params.inbox_id;
    let page = params.page;
    let search_filter = params.search_filter;
    let size = params.size;
    let sort = params.sort;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/rulesets", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = inbox_id {
        local_var_req_builder = local_var_req_builder.query(&[("inboxId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = search_filter {
        local_var_req_builder = local_var_req_builder.query(&[("searchFilter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = size {
        local_var_req_builder = local_var_req_builder.query(&[("size", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetInboxRulesetsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Test an inbox ruleset
pub async fn test_inbox_ruleset(configuration: &configuration::Configuration, params: TestInboxRulesetParams) -> Result<crate::models::InboxRulesetTestResult, Error<TestInboxRulesetError>> {
    // unbox the parameters
    let id = params.id;
    let inbox_ruleset_test_options = params.inbox_ruleset_test_options;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/rulesets/{id}/test", configuration.base_path, id=id);
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
    local_var_req_builder = local_var_req_builder.json(&inbox_ruleset_test_options);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<TestInboxRulesetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Test inbox rulesets for inbox
pub async fn test_inbox_rulesets_for_inbox(configuration: &configuration::Configuration, params: TestInboxRulesetsForInboxParams) -> Result<crate::models::InboxRulesetTestResult, Error<TestInboxRulesetsForInboxError>> {
    // unbox the parameters
    let inbox_id = params.inbox_id;
    let inbox_ruleset_test_options = params.inbox_ruleset_test_options;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/rulesets", configuration.base_path);
    let mut local_var_req_builder = local_var_client.put(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("inboxId", &inbox_id.to_string())]);
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
    local_var_req_builder = local_var_req_builder.json(&inbox_ruleset_test_options);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<TestInboxRulesetsForInboxError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Test new inbox ruleset
pub async fn test_new_inbox_ruleset(configuration: &configuration::Configuration, params: TestNewInboxRulesetParams) -> Result<crate::models::InboxRulesetTestResult, Error<TestNewInboxRulesetError>> {
    // unbox the parameters
    let test_new_inbox_ruleset_options = params.test_new_inbox_ruleset_options;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/rulesets", configuration.base_path);
    let mut local_var_req_builder = local_var_client.patch(local_var_uri_str.as_str());

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
    local_var_req_builder = local_var_req_builder.json(&test_new_inbox_ruleset_options);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<TestNewInboxRulesetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

