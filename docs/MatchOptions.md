# MatchOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**conditions** | Option<[**Vec<crate::models::ConditionOption>**](ConditionOption)> | Zero or more conditions such as `{ condition: 'HAS_ATTACHMENTS', value: 'TRUE' }`. Note the values are the strings `TRUE|FALSE` not booleans. | [optional]
**matches** | Option<[**Vec<crate::models::MatchOption>**](MatchOption)> | Zero or more match options such as `{ field: 'SUBJECT', should: 'CONTAIN', value: 'Welcome' }`. Options are additive so if one does not match the email is excluded from results | [optional]

[[Back to Model list]](../README#documentation-for-models) [[Back to API list]](../README#documentation-for-api-endpoints) [[Back to README]](../README)


