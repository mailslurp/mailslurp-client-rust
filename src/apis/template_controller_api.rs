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

pub struct TemplateControllerApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> TemplateControllerApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> TemplateControllerApiClient<C> {
        TemplateControllerApiClient {
            configuration,
        }
    }
}

pub trait TemplateControllerApi {
    fn create_template(&self, create_template_options: crate::models::CreateTemplateOptions) -> Box<dyn Future<Item = crate::models::TemplateDto, Error = Error<serde_json::Value>>>;
    fn delete_template(&self, template_id: &str) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn get_all_templates(&self, page: Option<i32>, size: Option<i32>, sort: Option<&str>) -> Box<dyn Future<Item = crate::models::PageTemplateProjection, Error = Error<serde_json::Value>>>;
    fn get_template(&self, template_id: &str) -> Box<dyn Future<Item = crate::models::TemplateDto, Error = Error<serde_json::Value>>>;
    fn get_templates(&self, ) -> Box<dyn Future<Item = Vec<crate::models::TemplateProjection>, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>TemplateControllerApi for TemplateControllerApiClient<C> {
    fn create_template(&self, create_template_options: crate::models::CreateTemplateOptions) -> Box<dyn Future<Item = crate::models::TemplateDto, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/templates".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "x-api-key".to_owned(),
            }))
        ;
        req = req.with_body_param(create_template_options);

        req.execute(self.configuration.borrow())
    }

    fn delete_template(&self, template_id: &str) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/templates/{TemplateId}".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "x-api-key".to_owned(),
            }))
        ;
        req = req.with_path_param("TemplateId".to_string(), template_id.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn get_all_templates(&self, page: Option<i32>, size: Option<i32>, sort: Option<&str>) -> Box<dyn Future<Item = crate::models::PageTemplateProjection, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/templates/paginated".to_string())
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

        req.execute(self.configuration.borrow())
    }

    fn get_template(&self, template_id: &str) -> Box<dyn Future<Item = crate::models::TemplateDto, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/templates/{TemplateId}".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "x-api-key".to_owned(),
            }))
        ;
        req = req.with_path_param("TemplateId".to_string(), template_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_templates(&self, ) -> Box<dyn Future<Item = Vec<crate::models::TemplateProjection>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/templates".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "x-api-key".to_owned(),
            }))
        ;

        req.execute(self.configuration.borrow())
    }

}
