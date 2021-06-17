/*
 * MailSlurp API
 *
 * MailSlurp is an API for sending and receiving emails from dynamically allocated email addresses. It's designed for developers and QA teams to test applications, process inbound emails, send templated notifications, attachments, and more.  ## Resources  - [Homepage](https://www.mailslurp.com) - Get an [API KEY](https://app.mailslurp.com/sign-up/) - Generated [SDK Clients](https://www.mailslurp.com/docs/) - [Examples](https://github.com/mailslurp/examples) repository 
 *
 * The version of the OpenAPI document: 6.5.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DomainDto : Domain plus verification records and status



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainDto {
    /// The optional catch all inbox that will receive emails sent to the domain that cannot be matched.
    #[serde(rename = "catchAllInboxId", skip_serializing_if = "Option::is_none")]
    pub catch_all_inbox_id: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// Unique token DKIM tokens
    #[serde(rename = "dkimTokens", skip_serializing_if = "Option::is_none")]
    pub dkim_tokens: Option<Vec<String>>,
    /// Custom domain name
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// List of DNS domain name records (C, MX, TXT) etc that you must add to the DNS server associated with your domain provider.
    #[serde(rename = "domainNameRecords", skip_serializing_if = "Option::is_none")]
    pub domain_name_records: Option<Vec<crate::models::DomainNameRecord>>,
    /// The type of domain. SMTP or HTTP domains differ in what inboxes can be used with them.
    #[serde(rename = "domainType", skip_serializing_if = "Option::is_none")]
    pub domain_type: Option<DomainType>,
    #[serde(rename = "id")]
    pub id: String,
    /// Whether domain has been verified or not. If the domain is not verified after 72 hours there is most likely an issue with the domains DNS records.
    #[serde(rename = "isVerified", skip_serializing_if = "Option::is_none")]
    pub is_verified: Option<bool>,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    /// Verification tokens
    #[serde(rename = "verificationToken", skip_serializing_if = "Option::is_none")]
    pub verification_token: Option<String>,
}

impl DomainDto {
    /// Domain plus verification records and status
    pub fn new(created_at: String, id: String, updated_at: String, user_id: String) -> DomainDto {
        DomainDto {
            catch_all_inbox_id: None,
            created_at,
            dkim_tokens: None,
            domain: None,
            domain_name_records: None,
            domain_type: None,
            id,
            is_verified: None,
            updated_at,
            user_id,
            verification_token: None,
        }
    }
}

/// The type of domain. SMTP or HTTP domains differ in what inboxes can be used with them.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DomainType {
    #[serde(rename = "HTTP_INBOX")]
    HTTPINBOX,
    #[serde(rename = "SMTP_DOMAIN")]
    SMTPDOMAIN,
}

