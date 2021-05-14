/*
 * MailSlurp API
 *
 * MailSlurp is an API for sending and receiving emails from dynamically allocated email addresses. It's designed for developers and QA teams to test applications, process inbound emails, send templated notifications, attachments, and more.  ## Resources  - [Homepage](https://www.mailslurp.com) - Get an [API KEY](https://app.mailslurp.com/sign-up/) - Generated [SDK Clients](https://www.mailslurp.com/docs/) - [Examples](https://github.com/mailslurp/examples) repository 
 *
 * The version of the OpenAPI document: 6.5.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DnsLookupOptions : Options for DNS query. 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DnsLookupOptions {
    /// List of record types you wish to query such as MX, DNS, TXT, NS, A etc.
    #[serde(rename = "hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// Optionally control whether to omit the final dot in full DNS name values.
    #[serde(rename = "omitFinalDNSDot", skip_serializing_if = "Option::is_none")]
    pub omit_final_dns_dot: Option<bool>,
    /// List of record types you wish to query such as MX, DNS, TXT, NS, A etc.
    #[serde(rename = "recordTypes", skip_serializing_if = "Option::is_none")]
    pub record_types: Option<Vec<RecordTypes>>,
}

impl DnsLookupOptions {
    /// Options for DNS query. 
    pub fn new() -> DnsLookupOptions {
        DnsLookupOptions {
            hostname: None,
            omit_final_dns_dot: None,
            record_types: None,
        }
    }
}

/// List of record types you wish to query such as MX, DNS, TXT, NS, A etc.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RecordTypes {
    #[serde(rename = "A")]
    A,
    #[serde(rename = "NS")]
    NS,
    #[serde(rename = "MD")]
    MD,
    #[serde(rename = "MF")]
    MF,
    #[serde(rename = "CNAME")]
    CNAME,
    #[serde(rename = "SOA")]
    SOA,
    #[serde(rename = "MB")]
    MB,
    #[serde(rename = "MG")]
    MG,
    #[serde(rename = "MR")]
    MR,
    #[serde(rename = "NULL")]
    NULL,
    #[serde(rename = "WKS")]
    WKS,
    #[serde(rename = "PTR")]
    PTR,
    #[serde(rename = "HINFO")]
    HINFO,
    #[serde(rename = "MINFO")]
    MINFO,
    #[serde(rename = "MX")]
    MX,
    #[serde(rename = "TXT")]
    TXT,
    #[serde(rename = "RP")]
    RP,
    #[serde(rename = "AFSDB")]
    AFSDB,
    #[serde(rename = "X25")]
    X25,
    #[serde(rename = "ISDN")]
    ISDN,
    #[serde(rename = "RT")]
    RT,
    #[serde(rename = "NSAP")]
    NSAP,
    #[serde(rename = "NSAP_PTR")]
    NSAPPTR,
    #[serde(rename = "SIG")]
    SIG,
    #[serde(rename = "KEY")]
    KEY,
    #[serde(rename = "PX")]
    PX,
    #[serde(rename = "GPOS")]
    GPOS,
    #[serde(rename = "AAAA")]
    AAAA,
    #[serde(rename = "LOC")]
    LOC,
    #[serde(rename = "NXT")]
    NXT,
    #[serde(rename = "EID")]
    EID,
    #[serde(rename = "NIMLOC")]
    NIMLOC,
    #[serde(rename = "SRV")]
    SRV,
    #[serde(rename = "ATMA")]
    ATMA,
    #[serde(rename = "NAPTR")]
    NAPTR,
    #[serde(rename = "KX")]
    KX,
    #[serde(rename = "CERT")]
    CERT,
    #[serde(rename = "A6")]
    A6,
    #[serde(rename = "DNAME")]
    DNAME,
    #[serde(rename = "SINK")]
    SINK,
    #[serde(rename = "OPT")]
    OPT,
    #[serde(rename = "APL")]
    APL,
    #[serde(rename = "DS")]
    DS,
    #[serde(rename = "SSHFP")]
    SSHFP,
    #[serde(rename = "IPSECKEY")]
    IPSECKEY,
    #[serde(rename = "RRSIG")]
    RRSIG,
    #[serde(rename = "NSEC")]
    NSEC,
    #[serde(rename = "DNSKEY")]
    DNSKEY,
    #[serde(rename = "DHCID")]
    DHCID,
    #[serde(rename = "NSEC3")]
    NSEC3,
    #[serde(rename = "NSEC3PARAM")]
    NSEC3PARAM,
    #[serde(rename = "TLSA")]
    TLSA,
    #[serde(rename = "SMIMEA")]
    SMIMEA,
    #[serde(rename = "HIP")]
    HIP,
    #[serde(rename = "NINFO")]
    NINFO,
    #[serde(rename = "RKEY")]
    RKEY,
    #[serde(rename = "TALINK")]
    TALINK,
    #[serde(rename = "CDS")]
    CDS,
    #[serde(rename = "CDNSKEY")]
    CDNSKEY,
    #[serde(rename = "OPENPGPKEY")]
    OPENPGPKEY,
    #[serde(rename = "CSYNC")]
    CSYNC,
    #[serde(rename = "ZONEMD")]
    ZONEMD,
    #[serde(rename = "SVCB")]
    SVCB,
    #[serde(rename = "HTTPS")]
    HTTPS,
    #[serde(rename = "SPF")]
    SPF,
    #[serde(rename = "UINFO")]
    UINFO,
    #[serde(rename = "UID")]
    UID,
    #[serde(rename = "GID")]
    GID,
    #[serde(rename = "UNSPEC")]
    UNSPEC,
    #[serde(rename = "NID")]
    NID,
    #[serde(rename = "L32")]
    L32,
    #[serde(rename = "L64")]
    L64,
    #[serde(rename = "LP")]
    LP,
    #[serde(rename = "EUI48")]
    EUI48,
    #[serde(rename = "EUI64")]
    EUI64,
    #[serde(rename = "TKEY")]
    TKEY,
    #[serde(rename = "TSIG")]
    TSIG,
    #[serde(rename = "IXFR")]
    IXFR,
    #[serde(rename = "AXFR")]
    AXFR,
    #[serde(rename = "MAILB")]
    MAILB,
    #[serde(rename = "MAILA")]
    MAILA,
    #[serde(rename = "ANY")]
    ANY,
    #[serde(rename = "URI")]
    URI,
    #[serde(rename = "CAA")]
    CAA,
    #[serde(rename = "AVC")]
    AVC,
    #[serde(rename = "DOA")]
    DOA,
    #[serde(rename = "AMTRELAY")]
    AMTRELAY,
    #[serde(rename = "TA")]
    TA,
    #[serde(rename = "DLV")]
    DLV,
}

