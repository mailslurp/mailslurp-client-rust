use hyper;
use serde;
use serde_json;

#[derive(Debug)]
pub enum Error<T> {
    UriError(hyper::error::UriError),
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    ApiError(ApiError<T>),
}

#[derive(Debug)]
pub struct ApiError<T> {
    pub code: hyper::StatusCode,
    pub content: Option<T>,
}

impl<'de, T> From<(hyper::StatusCode, &'de [u8])> for Error<T> 
    where T: serde::Deserialize<'de> {
    fn from(e: (hyper::StatusCode, &'de [u8])) -> Self {
        if e.1.len() == 0 {
            return Error::ApiError(ApiError{
                code: e.0,
                content: None,
            });
        }
        match serde_json::from_slice::<T>(e.1) {
            Ok(t) => Error::ApiError(ApiError{
                code: e.0,
                content: Some(t),
            }),
            Err(e) => {
                Error::from(e)
            }
        }
    }
}

impl<T> From<hyper::Error> for Error<T> {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

mod request;

mod alias_controller_api;
pub use self::alias_controller_api::{ AliasControllerApi, AliasControllerApiClient };
mod attachment_controller_api;
pub use self::attachment_controller_api::{ AttachmentControllerApi, AttachmentControllerApiClient };
mod bulk_actions_controller_api;
pub use self::bulk_actions_controller_api::{ BulkActionsControllerApi, BulkActionsControllerApiClient };
mod common_actions_controller_api;
pub use self::common_actions_controller_api::{ CommonActionsControllerApi, CommonActionsControllerApiClient };
mod contact_controller_api;
pub use self::contact_controller_api::{ ContactControllerApi, ContactControllerApiClient };
mod domain_controller_api;
pub use self::domain_controller_api::{ DomainControllerApi, DomainControllerApiClient };
mod email_controller_api;
pub use self::email_controller_api::{ EmailControllerApi, EmailControllerApiClient };
mod form_controller_api;
pub use self::form_controller_api::{ FormControllerApi, FormControllerApiClient };
mod group_controller_api;
pub use self::group_controller_api::{ GroupControllerApi, GroupControllerApiClient };
mod inbox_controller_api;
pub use self::inbox_controller_api::{ InboxControllerApi, InboxControllerApiClient };
mod mail_server_controller_api;
pub use self::mail_server_controller_api::{ MailServerControllerApi, MailServerControllerApiClient };
mod sent_emails_controller_api;
pub use self::sent_emails_controller_api::{ SentEmailsControllerApi, SentEmailsControllerApiClient };
mod template_controller_api;
pub use self::template_controller_api::{ TemplateControllerApi, TemplateControllerApiClient };
mod wait_for_controller_api;
pub use self::wait_for_controller_api::{ WaitForControllerApi, WaitForControllerApiClient };
mod webhook_controller_api;
pub use self::webhook_controller_api::{ WebhookControllerApi, WebhookControllerApiClient };

pub mod configuration;
pub mod client;
