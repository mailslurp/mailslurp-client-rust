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

/// struct for passing parameters to the method `create_new_email_address`
#[derive(Clone, Debug)]
pub struct CreateNewEmailAddressParams {
    /// allowTeamAccess
    pub allow_team_access: Option<bool>,
    /// expiresAt
    pub expires_at: Option<String>,
    /// expiresIn
    pub expires_in: Option<i64>,
    /// useDomainPool
    pub use_domain_pool: Option<bool>
}

/// struct for passing parameters to the method `create_new_email_address1`
#[derive(Clone, Debug)]
pub struct CreateNewEmailAddress1Params {
    /// allowTeamAccess
    pub allow_team_access: Option<bool>,
    /// expiresAt
    pub expires_at: Option<String>,
    /// expiresIn
    pub expires_in: Option<i64>,
    /// useDomainPool
    pub use_domain_pool: Option<bool>
}

/// struct for passing parameters to the method `empty_inbox`
#[derive(Clone, Debug)]
pub struct EmptyInboxParams {
    /// inboxId
    pub inbox_id: String
}

/// struct for passing parameters to the method `send_email_simple`
#[derive(Clone, Debug)]
pub struct SendEmailSimpleParams {
    /// emailOptions
    pub email_options: crate::models::SimpleSendEmailOptions
}


/// struct for typed errors of method `create_new_email_address`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateNewEmailAddressError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `create_new_email_address1`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateNewEmailAddress1Error {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `empty_inbox`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmptyInboxError {
    Status401(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `send_email_simple`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendEmailSimpleError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}


/// Returns an Inbox with an `id` and an `emailAddress`
pub async fn create_new_email_address(configuration: &configuration::Configuration, params: CreateNewEmailAddressParams) -> Result<crate::models::Inbox, Error<CreateNewEmailAddressError>> {
    // unbox the parameters
    let allow_team_access = params.allow_team_access;
    let expires_at = params.expires_at;
    let expires_in = params.expires_in;
    let use_domain_pool = params.use_domain_pool;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/createInbox", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = allow_team_access {
        local_var_req_builder = local_var_req_builder.query(&[("allowTeamAccess", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = expires_at {
        local_var_req_builder = local_var_req_builder.query(&[("expiresAt", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = expires_in {
        local_var_req_builder = local_var_req_builder.query(&[("expiresIn", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = use_domain_pool {
        local_var_req_builder = local_var_req_builder.query(&[("useDomainPool", &local_var_str.to_string())]);
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
        let local_var_entity: Option<CreateNewEmailAddressError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns an Inbox with an `id` and an `emailAddress`
pub async fn create_new_email_address1(configuration: &configuration::Configuration, params: CreateNewEmailAddress1Params) -> Result<crate::models::Inbox, Error<CreateNewEmailAddress1Error>> {
    // unbox the parameters
    let allow_team_access = params.allow_team_access;
    let expires_at = params.expires_at;
    let expires_in = params.expires_in;
    let use_domain_pool = params.use_domain_pool;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/newEmailAddress", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = allow_team_access {
        local_var_req_builder = local_var_req_builder.query(&[("allowTeamAccess", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = expires_at {
        local_var_req_builder = local_var_req_builder.query(&[("expiresAt", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = expires_in {
        local_var_req_builder = local_var_req_builder.query(&[("expiresIn", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = use_domain_pool {
        local_var_req_builder = local_var_req_builder.query(&[("useDomainPool", &local_var_str.to_string())]);
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
        let local_var_entity: Option<CreateNewEmailAddress1Error> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes all emails
pub async fn empty_inbox(configuration: &configuration::Configuration, params: EmptyInboxParams) -> Result<(), Error<EmptyInboxError>> {
    // unbox the parameters
    let inbox_id = params.inbox_id;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/emptyInbox", configuration.base_path);
    let mut local_var_req_builder = local_var_client.delete(local_var_uri_str.as_str());

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

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<EmptyInboxError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// If no senderId or inboxId provided a random email address will be used to send from.
pub async fn send_email_simple(configuration: &configuration::Configuration, params: SendEmailSimpleParams) -> Result<(), Error<SendEmailSimpleError>> {
    // unbox the parameters
    let email_options = params.email_options;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/sendEmail", configuration.base_path);
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
    local_var_req_builder = local_var_req_builder.json(&email_options);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<SendEmailSimpleError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

