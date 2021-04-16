# DomainDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**catch_all_inbox_id** | Option<**String**> | The optional catch all inbox that will receive emails sent to the domain that cannot be matched. | [optional]
**created_at** | **String** |  | 
**dkim_tokens** | Option<**Vec<String>**> | Unique token DKIM tokens | [optional]
**domain** | Option<**String**> | Custom domain name | [optional]
**domain_name_records** | Option<[**Vec<crate::models::DomainNameRecord>**](DomainNameRecord)> | List of DNS domain name records (C, MX, TXT) etc that you must add to the DNS server associated with your domain provider. | [optional]
**id** | **String** |  | 
**is_verified** | Option<**bool**> | Whether domain has been verified or not. If the domain is not verified after 72 hours there is most likely an issue with the domains DNS records. | [optional]
**updated_at** | **String** |  | 
**user_id** | **String** |  | 
**verification_token** | Option<**String**> | Verification tokens | [optional]

[[Back to Model list]](../README#documentation-for-models) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to README]](../README)


