# TrackRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**variant_key** | **String** | Variant key returned by evaluate (e.g. \"A\", \"v2\"). | 
**event_type** | **String** | Type of event (e.g. \"success\", \"failure\", \"error\"). | 
**reward** | Option<**f32**> | Numeric reward associated with event (e.g. 1.0 for conversion). Default 0. | [optional]
**context** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Arbitrary context passed by SDK (user id, session, metadata). | [optional]
**created_at** | Option<**String**> | Event timestamp. If omitted, server time will be used. | [optional]
**dedup_key** | Option<**String**> | Optional idempotency key to deduplicate duplicate events from SDK retries. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


