use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient {
    alias_controller_api: Box<dyn crate::apis::AliasControllerApi>,
    attachment_controller_api: Box<dyn crate::apis::AttachmentControllerApi>,
    bulk_actions_controller_api: Box<dyn crate::apis::BulkActionsControllerApi>,
    common_actions_controller_api: Box<dyn crate::apis::CommonActionsControllerApi>,
    contact_controller_api: Box<dyn crate::apis::ContactControllerApi>,
    domain_controller_api: Box<dyn crate::apis::DomainControllerApi>,
    email_controller_api: Box<dyn crate::apis::EmailControllerApi>,
    expired_controller_api: Box<dyn crate::apis::ExpiredControllerApi>,
    form_controller_api: Box<dyn crate::apis::FormControllerApi>,
    group_controller_api: Box<dyn crate::apis::GroupControllerApi>,
    inbox_controller_api: Box<dyn crate::apis::InboxControllerApi>,
    mail_server_controller_api: Box<dyn crate::apis::MailServerControllerApi>,
    sent_emails_controller_api: Box<dyn crate::apis::SentEmailsControllerApi>,
    template_controller_api: Box<dyn crate::apis::TemplateControllerApi>,
    wait_for_controller_api: Box<dyn crate::apis::WaitForControllerApi>,
    webhook_controller_api: Box<dyn crate::apis::WebhookControllerApi>,
}

impl APIClient {
    pub fn new<C: hyper::client::Connect>(configuration: Configuration<C>) -> APIClient {
        let rc = Rc::new(configuration);

        APIClient {
            alias_controller_api: Box::new(crate::apis::AliasControllerApiClient::new(rc.clone())),
            attachment_controller_api: Box::new(crate::apis::AttachmentControllerApiClient::new(rc.clone())),
            bulk_actions_controller_api: Box::new(crate::apis::BulkActionsControllerApiClient::new(rc.clone())),
            common_actions_controller_api: Box::new(crate::apis::CommonActionsControllerApiClient::new(rc.clone())),
            contact_controller_api: Box::new(crate::apis::ContactControllerApiClient::new(rc.clone())),
            domain_controller_api: Box::new(crate::apis::DomainControllerApiClient::new(rc.clone())),
            email_controller_api: Box::new(crate::apis::EmailControllerApiClient::new(rc.clone())),
            expired_controller_api: Box::new(crate::apis::ExpiredControllerApiClient::new(rc.clone())),
            form_controller_api: Box::new(crate::apis::FormControllerApiClient::new(rc.clone())),
            group_controller_api: Box::new(crate::apis::GroupControllerApiClient::new(rc.clone())),
            inbox_controller_api: Box::new(crate::apis::InboxControllerApiClient::new(rc.clone())),
            mail_server_controller_api: Box::new(crate::apis::MailServerControllerApiClient::new(rc.clone())),
            sent_emails_controller_api: Box::new(crate::apis::SentEmailsControllerApiClient::new(rc.clone())),
            template_controller_api: Box::new(crate::apis::TemplateControllerApiClient::new(rc.clone())),
            wait_for_controller_api: Box::new(crate::apis::WaitForControllerApiClient::new(rc.clone())),
            webhook_controller_api: Box::new(crate::apis::WebhookControllerApiClient::new(rc.clone())),
        }
    }

    pub fn alias_controller_api(&self) -> &dyn crate::apis::AliasControllerApi{
        self.alias_controller_api.as_ref()
    }

    pub fn attachment_controller_api(&self) -> &dyn crate::apis::AttachmentControllerApi{
        self.attachment_controller_api.as_ref()
    }

    pub fn bulk_actions_controller_api(&self) -> &dyn crate::apis::BulkActionsControllerApi{
        self.bulk_actions_controller_api.as_ref()
    }

    pub fn common_actions_controller_api(&self) -> &dyn crate::apis::CommonActionsControllerApi{
        self.common_actions_controller_api.as_ref()
    }

    pub fn contact_controller_api(&self) -> &dyn crate::apis::ContactControllerApi{
        self.contact_controller_api.as_ref()
    }

    pub fn domain_controller_api(&self) -> &dyn crate::apis::DomainControllerApi{
        self.domain_controller_api.as_ref()
    }

    pub fn email_controller_api(&self) -> &dyn crate::apis::EmailControllerApi{
        self.email_controller_api.as_ref()
    }

    pub fn expired_controller_api(&self) -> &dyn crate::apis::ExpiredControllerApi{
        self.expired_controller_api.as_ref()
    }

    pub fn form_controller_api(&self) -> &dyn crate::apis::FormControllerApi{
        self.form_controller_api.as_ref()
    }

    pub fn group_controller_api(&self) -> &dyn crate::apis::GroupControllerApi{
        self.group_controller_api.as_ref()
    }

    pub fn inbox_controller_api(&self) -> &dyn crate::apis::InboxControllerApi{
        self.inbox_controller_api.as_ref()
    }

    pub fn mail_server_controller_api(&self) -> &dyn crate::apis::MailServerControllerApi{
        self.mail_server_controller_api.as_ref()
    }

    pub fn sent_emails_controller_api(&self) -> &dyn crate::apis::SentEmailsControllerApi{
        self.sent_emails_controller_api.as_ref()
    }

    pub fn template_controller_api(&self) -> &dyn crate::apis::TemplateControllerApi{
        self.template_controller_api.as_ref()
    }

    pub fn wait_for_controller_api(&self) -> &dyn crate::apis::WaitForControllerApi{
        self.wait_for_controller_api.as_ref()
    }

    pub fn webhook_controller_api(&self) -> &dyn crate::apis::WebhookControllerApi{
        self.webhook_controller_api.as_ref()
    }

}
