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

/// struct for passing parameters to the method `submit_form`
#[derive(Clone, Debug)]
pub struct SubmitFormParams {
    /// Email address of the submitting user. Include this if you wish to record the submitters email address and reply to it later.
    pub _email_address: Option<String>,
    /// Optional URL to redirect form submitter to after submission. If not present user will see a success message.
    pub _redirect_to: Option<String>,
    /// Optional but recommended field that catches spammers out. Include as a hidden form field but LEAVE EMPTY. Spam-bots will usually fill every field. If the _spamCheck field is filled the form submission will be ignored.
    pub _spam_check: Option<String>,
    /// Optional subject of the email that will be sent.
    pub _subject: Option<String>,
    /// Optional success message to display if no _redirectTo present.
    pub _success_message: Option<String>,
    /// The email address that submitted form should be sent to.
    pub _to: Option<String>,
    /// All other parameters or fields will be accepted and attached to the sent email. This includes files and any HTML form field with a name. These fields will become the body of the email that is sent.
    pub other_parameters: Option<String>
}


/// struct for typed errors of method `submit_form`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubmitFormError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}


/// This endpoint allows you to submit HTML forms and receive the field values and files via email.   #### Parameters The endpoint looks for special meta parameters in the form fields OR in the URL request parameters. The meta parameters can be used to specify the behaviour of the email.   You must provide at-least a `_to` email address to tell the endpoint where the form should be emailed. These can be submitted as hidden HTML input fields with the corresponding `name` attributes or as URL query parameters such as `?_to=test@example.com`  The endpoint takes all other form fields that are named and includes them in the message body of the email. Files are sent as attachments.  #### Submitting This endpoint accepts form submission via POST method. It accepts `application/x-www-form-urlencoded`, and `multipart/form-data` content-types.  #### HTML Example ```html <form    action=\"https://api.mailslurp.com/forms\"   method=\"post\" >   <input name=\"_to\" type=\"hidden\" value=\"test@example.com\"/>   <textarea name=\"feedback\"></textarea>   <button type=\"submit\">Submit</button> </form> ```  #### URL Example ```html <form    action=\"https://api.mailslurp.com/forms?_to=test@example.com\"   method=\"post\" >   <textarea name=\"feedback\"></textarea>   <button type=\"submit\">Submit</button> </form> ```    The email address is specified by a `_to` field OR is extracted from an email alias specified by a `_toAlias` field (see the alias controller for more information).  Endpoint accepts .  You can specify a content type in HTML forms using the `enctype` attribute, for instance: `<form enctype=\"multipart/form-data\">`.  
pub async fn submit_form(configuration: &configuration::Configuration, params: SubmitFormParams) -> Result<String, Error<SubmitFormError>> {
    // unbox the parameters
    let _email_address = params._email_address;
    let _redirect_to = params._redirect_to;
    let _spam_check = params._spam_check;
    let _subject = params._subject;
    let _success_message = params._success_message;
    let _to = params._to;
    let other_parameters = params.other_parameters;


    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/forms", configuration.base_path);
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
    let mut local_var_form = reqwest::multipart::Form::new();
    if let Some(local_var_param_value) = _email_address {
        local_var_form = local_var_form.text("_emailAddress", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = _redirect_to {
        local_var_form = local_var_form.text("_redirectTo", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = _spam_check {
        local_var_form = local_var_form.text("_spamCheck", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = _subject {
        local_var_form = local_var_form.text("_subject", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = _success_message {
        local_var_form = local_var_form.text("_successMessage", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = _to {
        local_var_form = local_var_form.text("_to", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = other_parameters {
        local_var_form = local_var_form.text("otherParameters", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.multipart(local_var_form);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SubmitFormError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

