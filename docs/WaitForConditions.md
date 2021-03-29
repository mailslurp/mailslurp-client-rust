# WaitForConditions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**count** | Option<**i32**> | Number of results that should match conditions. Either exactly or at least this amount based on the `countType`. If count condition is not met and the timeout has not been reached the `waitFor` method will retry the operation. | [optional]
**count_type** | Option<**String**> | How should the found count be compared to the expected count. | [optional]
**inbox_id** | Option<**String**> | ID of inbox to search within and apply conditions to. Essentially filtering the emails found to give a count. | [optional]
**matches** | Option<[**Vec<crate::models::MatchOption>**](MatchOption.md)> | Conditions that should be matched for an email to qualify for results. Each condition will be applied in order to each email within an inbox to filter a result list of matching emails you are waiting for. | [optional]
**sort_direction** | Option<**String**> | Direction to sort matching emails by created time | [optional]
**timeout** | Option<**i64**> | Max time in milliseconds to retry the `waitFor` operation until conditions are met. | [optional]
**unread_only** | Option<**bool**> | Apply conditions only to **unread** emails. All emails begin with `read=false`. An email is marked `read=true` when an `EmailDto` representation of it has been returned to the user at least once. For example you have called `getEmail` or `waitForLatestEmail` etc., or you have viewed the email in the dashboard.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


