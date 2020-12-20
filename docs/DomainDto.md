# DomainDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **String** |  | 
**dkim_tokens** | Option<**Vec<String>**> | DNS records for DKIM approval | [optional]
**domain** | Option<**String**> | Custom domain name | [optional]
**id** | **String** |  | 
**is_verified** | Option<**bool**> | Whether domain has been verified or not. If the domain is not verified after 72 hours there is most likely an issue with the domains DNS records. | [optional]
**updated_at** | **String** |  | 
**user_id** | **String** |  | 
**verification_token** | Option<**String**> | A TXT record that you must place in the DNS settings of the domain to complete domain verification | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


