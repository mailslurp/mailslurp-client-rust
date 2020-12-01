/*
 * MailSlurp API
 *
 * MailSlurp is an API for sending and receiving emails from dynamically allocated email addresses. It's designed for developers and QA teams to test applications, process inbound emails, send templated notifications, attachments, and more.   ## Resources - [Homepage](https://www.mailslurp.com) - Get an [API KEY](https://app.mailslurp.com/sign-up/) - Generated [SDK Clients](https://www.mailslurp.com/docs/) - [Examples](https://github.com/mailslurp/examples) repository 
 *
 * The version of the OpenAPI document: 6.5.2
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use hyper;
use serde_json;
use futures::Future;

use super::{Error, configuration};
use super::request as __internal_request;

pub struct InboxControllerApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> InboxControllerApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> InboxControllerApiClient<C> {
        InboxControllerApiClient {
            configuration,
        }
    }
}

pub trait InboxControllerApi {
    fn create_inbox(&self, description: Option<&str>, email_address: Option<&str>, expires_at: Option<String>, favourite: Option<bool>, name: Option<&str>, tags: Option<Vec<String>>) -> Box<dyn Future<Item = crate::models::Inbox, Error = Error<serde_json::Value>>>;
    fn delete_all_inboxes(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn delete_inbox(&self, inbox_id: &str) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn get_all_inboxes(&self, favourite: Option<bool>, page: Option<i32>, search: Option<&str>, size: Option<i32>, sort: Option<&str>, tag: Option<&str>) -> Box<dyn Future<Item = crate::models::PageInboxProjection, Error = Error<serde_json::Value>>>;
    fn get_emails(&self, inbox_id: &str, limit: Option<i32>, min_count: Option<i64>, retry_timeout: Option<i64>, since: Option<String>, size: Option<i32>, sort: Option<&str>) -> Box<dyn Future<Item = Vec<crate::models::EmailPreview>, Error = Error<serde_json::Value>>>;
    fn get_inbox(&self, inbox_id: &str) -> Box<dyn Future<Item = crate::models::Inbox, Error = Error<serde_json::Value>>>;
    fn get_inbox_emails_paginated(&self, inbox_id: &str, page: Option<i32>, size: Option<i32>, sort: Option<&str>) -> Box<dyn Future<Item = crate::models::PageEmailPreview, Error = Error<serde_json::Value>>>;
    fn get_inbox_sent_emails(&self, inbox_id: &str, page: Option<i32>, size: Option<i32>, sort: Option<&str>) -> Box<dyn Future<Item = crate::models::PageSentEmailProjection, Error = Error<serde_json::Value>>>;
    fn get_inbox_tags(&self, ) -> Box<dyn Future<Item = Vec<String>, Error = Error<serde_json::Value>>>;
    fn get_inboxes(&self, size: Option<i32>, sort: Option<&str>) -> Box<dyn Future<Item = Vec<crate::models::Inbox>, Error = Error<serde_json::Value>>>;
    fn send_email(&self, inbox_id: &str, send_email_options: Option<crate::models::SendEmailOptions>) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn send_email_and_confirm(&self, inbox_id: &str, send_email_options: Option<crate::models::SendEmailOptions>) -> Box<dyn Future<Item = crate::models::SentEmailDto, Error = Error<serde_json::Value>>>;
    fn set_inbox_favourited(&self, inbox_id: &str, set_inbox_favourited_options: crate::models::SetInboxFavouritedOptions) -> Box<dyn Future<Item = crate::models::Inbox, Error = Error<serde_json::Value>>>;
    fn update_inbox(&self, inbox_id: &str, update_inbox_options: crate::models::UpdateInboxOptions) -> Box<dyn Future<Item = crate::models::Inbox, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>InboxControllerApi for InboxControllerApiClient<C> {
    fn create_inbox(&self, description: Option<&str>, email_address: Option<&str>, expires_at: Option<String>, favourite: Option<bool>, name: Option<&str>, tags: Option<Vec<String>>) -> Box<dyn Future<Item = crate::models::Inbox, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/inboxes".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "x-api-key".to_owned(),
            }))
        ;
        if let Some(ref s) = description {
            req = req.with_query_param("description".to_string(), s.to_string());
        }
        if let Some(ref s) = email_address {
            req = req.with_query_param("emailAddress".to_string(), s.to_string());
        }
        if let Some(ref s) = expires_at {
            req = req.with_query_param("expiresAt".to_string(), s.to_string());
        }
        if let Some(ref s) = favourite {
            req = req.with_query_param("favourite".to_string(), s.to_string());
        }
        if let Some(ref s) = name {
            req = req.with_query_param("name".to_string(), s.to_string());
        }
        if let Some(ref s) = tags {
            req = req.with_query_param("tags".to_string(), s.join(",").to_string());
        }

        req.execute(self.configuration.borrow())
    }

    fn delete_all_inboxes(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/inboxes".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "x-api-key".to_owned(),
            }))
        ;
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn delete_inbox(&self, inbox_id: &str) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/inboxes/{inboxId}".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "x-api-key".to_owned(),
            }))
        ;
        req = req.with_path_param("inboxId".to_string(), inbox_id.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn get_all_inboxes(&self, favourite: Option<bool>, page: Option<i32>, search: Option<&str>, size: Option<i32>, sort: Option<&str>, tag: Option<&str>) -> Box<dyn Future<Item = crate::models::PageInboxProjection, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/inboxes/paginated".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "x-api-key".to_owned(),
            }))
        ;
        if let Some(ref s) = favourite {
            req = req.with_query_param("favourite".to_string(), s.to_string());
        }
        if let Some(ref s) = page {
            req = req.with_query_param("page".to_string(), s.to_string());
        }
        if let Some(ref s) = search {
            req = req.with_query_param("search".to_string(), s.to_string());
        }
        if let Some(ref s) = size {
            req = req.with_query_param("size".to_string(), s.to_string());
        }
        if let Some(ref s) = sort {
            req = req.with_query_param("sort".to_string(), s.to_string());
        }
        if let Some(ref s) = tag {
            req = req.with_query_param("tag".to_string(), s.to_string());
        }

        req.execute(self.configuration.borrow())
    }

    fn get_emails(&self, inbox_id: &str, limit: Option<i32>, min_count: Option<i64>, retry_timeout: Option<i64>, since: Option<String>, size: Option<i32>, sort: Option<&str>) -> Box<dyn Future<Item = Vec<crate::models::EmailPreview>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/inboxes/{inboxId}/emails".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "x-api-key".to_owned(),
            }))
        ;
        if let Some(ref s) = limit {
            req = req.with_query_param("limit".to_string(), s.to_string());
        }
        if let Some(ref s) = min_count {
            req = req.with_query_param("minCount".to_string(), s.to_string());
        }
        if let Some(ref s) = retry_timeout {
            req = req.with_query_param("retryTimeout".to_string(), s.to_string());
        }
        if let Some(ref s) = since {
            req = req.with_query_param("since".to_string(), s.to_string());
        }
        if let Some(ref s) = size {
            req = req.with_query_param("size".to_string(), s.to_string());
        }
        if let Some(ref s) = sort {
            req = req.with_query_param("sort".to_string(), s.to_string());
        }
        req = req.with_path_param("inboxId".to_string(), inbox_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_inbox(&self, inbox_id: &str) -> Box<dyn Future<Item = crate::models::Inbox, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/inboxes/{inboxId}".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "x-api-key".to_owned(),
            }))
        ;
        req = req.with_path_param("inboxId".to_string(), inbox_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_inbox_emails_paginated(&self, inbox_id: &str, page: Option<i32>, size: Option<i32>, sort: Option<&str>) -> Box<dyn Future<Item = crate::models::PageEmailPreview, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/inboxes/{inboxId}/emails/paginated".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "x-api-key".to_owned(),
            }))
        ;
        if let Some(ref s) = page {
            req = req.with_query_param("page".to_string(), s.to_string());
        }
        if let Some(ref s) = size {
            req = req.with_query_param("size".to_string(), s.to_string());
        }
        if let Some(ref s) = sort {
            req = req.with_query_param("sort".to_string(), s.to_string());
        }
        req = req.with_path_param("inboxId".to_string(), inbox_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_inbox_sent_emails(&self, inbox_id: &str, page: Option<i32>, size: Option<i32>, sort: Option<&str>) -> Box<dyn Future<Item = crate::models::PageSentEmailProjection, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/inboxes/{inboxId}/sent".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "x-api-key".to_owned(),
            }))
        ;
        if let Some(ref s) = page {
            req = req.with_query_param("page".to_string(), s.to_string());
        }
        if let Some(ref s) = size {
            req = req.with_query_param("size".to_string(), s.to_string());
        }
        if let Some(ref s) = sort {
            req = req.with_query_param("sort".to_string(), s.to_string());
        }
        req = req.with_path_param("inboxId".to_string(), inbox_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_inbox_tags(&self, ) -> Box<dyn Future<Item = Vec<String>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/inboxes/tags".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "x-api-key".to_owned(),
            }))
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_inboxes(&self, size: Option<i32>, sort: Option<&str>) -> Box<dyn Future<Item = Vec<crate::models::Inbox>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/inboxes".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "x-api-key".to_owned(),
            }))
        ;
        if let Some(ref s) = size {
            req = req.with_query_param("size".to_string(), s.to_string());
        }
        if let Some(ref s) = sort {
            req = req.with_query_param("sort".to_string(), s.to_string());
        }

        req.execute(self.configuration.borrow())
    }

    fn send_email(&self, inbox_id: &str, send_email_options: Option<crate::models::SendEmailOptions>) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/inboxes/{inboxId}".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "x-api-key".to_owned(),
            }))
        ;
        req = req.with_path_param("inboxId".to_string(), inbox_id.to_string());
        req = req.with_body_param(send_email_options);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn send_email_and_confirm(&self, inbox_id: &str, send_email_options: Option<crate::models::SendEmailOptions>) -> Box<dyn Future<Item = crate::models::SentEmailDto, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/inboxes/{inboxId}/confirm".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "x-api-key".to_owned(),
            }))
        ;
        req = req.with_path_param("inboxId".to_string(), inbox_id.to_string());
        req = req.with_body_param(send_email_options);

        req.execute(self.configuration.borrow())
    }

    fn set_inbox_favourited(&self, inbox_id: &str, set_inbox_favourited_options: crate::models::SetInboxFavouritedOptions) -> Box<dyn Future<Item = crate::models::Inbox, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Put, "/inboxes/{inboxId}/favourite".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "x-api-key".to_owned(),
            }))
        ;
        req = req.with_path_param("inboxId".to_string(), inbox_id.to_string());
        req = req.with_body_param(set_inbox_favourited_options);

        req.execute(self.configuration.borrow())
    }

    fn update_inbox(&self, inbox_id: &str, update_inbox_options: crate::models::UpdateInboxOptions) -> Box<dyn Future<Item = crate::models::Inbox, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Patch, "/inboxes/{inboxId}".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "x-api-key".to_owned(),
            }))
        ;
        req = req.with_path_param("inboxId".to_string(), inbox_id.to_string());
        req = req.with_body_param(update_inbox_options);

        req.execute(self.configuration.borrow())
    }

}
