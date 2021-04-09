/*
 * MailSlurp API
 *
 * MailSlurp is an API for sending and receiving emails from dynamically allocated email addresses. It's designed for developers and QA teams to test applications, process inbound emails, send templated notifications, attachments, and more.   ## Resources - [Homepage](https://www.mailslurp.com) - Get an [API KEY](https://app.mailslurp.com/sign-up/) - Generated [SDK Clients](https://www.mailslurp.com/docs/) - [Examples](https://github.com/mailslurp/examples) repository 
 *
 * The version of the OpenAPI document: 6.5.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EmailAnalysis : Analysis result for email. Each verdict property is a string PASS|FAIL|GRAY or dynamic error message



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailAnalysis {
    /// Verdict of DomainKeys Identified Mail analysis
    #[serde(rename = "dkimVerdict", skip_serializing_if = "Option::is_none")]
    pub dkim_verdict: Option<String>,
    /// Verdict of Domain-based Message Authentication Reporting and Conformance analysis
    #[serde(rename = "dmarcVerdict", skip_serializing_if = "Option::is_none")]
    pub dmarc_verdict: Option<String>,
    /// Verdict of spam ranking analysis
    #[serde(rename = "spamVerdict", skip_serializing_if = "Option::is_none")]
    pub spam_verdict: Option<String>,
    /// Verdict of Send Policy Framework record spoofing analysis
    #[serde(rename = "spfVerdict", skip_serializing_if = "Option::is_none")]
    pub spf_verdict: Option<String>,
    /// Verdict of virus scan analysis
    #[serde(rename = "virusVerdict", skip_serializing_if = "Option::is_none")]
    pub virus_verdict: Option<String>,
}

impl EmailAnalysis {
    /// Analysis result for email. Each verdict property is a string PASS|FAIL|GRAY or dynamic error message
    pub fn new() -> EmailAnalysis {
        EmailAnalysis {
            dkim_verdict: None,
            dmarc_verdict: None,
            spam_verdict: None,
            spf_verdict: None,
            virus_verdict: None,
        }
    }
}


