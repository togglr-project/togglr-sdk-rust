# \DefaultApi

All URIs are relative to *http://localhost:8090*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_feature_health**](DefaultApi.md#get_feature_health) | **GET** /sdk/v1/features/{feature_key}/health | Get health status of feature (including auto-disable state)
[**report_feature_error**](DefaultApi.md#report_feature_error) | **POST** /sdk/v1/features/{feature_key}/report-error | Report feature execution error (for auto-disable)
[**sdk_v1_features_feature_key_evaluate_post**](DefaultApi.md#sdk_v1_features_feature_key_evaluate_post) | **POST** /sdk/v1/features/{feature_key}/evaluate | Evaluate feature for given context
[**sdk_v1_health_get**](DefaultApi.md#sdk_v1_health_get) | **GET** /sdk/v1/health | Health check for SDK server
[**track_feature_event**](DefaultApi.md#track_feature_event) | **POST** /sdk/v1/features/{feature_key}/track | Track event for a feature (impression / conversion / error / custom)



## get_feature_health

> models::FeatureHealth get_feature_health(feature_key)
Get health status of feature (including auto-disable state)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feature_key** | **String** |  | [required] |

### Return type

[**models::FeatureHealth**](FeatureHealth.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## report_feature_error

> report_feature_error(feature_key, feature_error_report)
Report feature execution error (for auto-disable)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feature_key** | **String** |  | [required] |
**feature_error_report** | [**FeatureErrorReport**](FeatureErrorReport.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sdk_v1_features_feature_key_evaluate_post

> models::EvaluateResponse sdk_v1_features_feature_key_evaluate_post(feature_key, request_body)
Evaluate feature for given context

Returns feature evaluation result for given project and context. The project is derived from the API key. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feature_key** | **String** |  | [required] |
**request_body** | [**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) |  | [required] |

### Return type

[**models::EvaluateResponse**](EvaluateResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sdk_v1_health_get

> models::HealthResponse sdk_v1_health_get()
Health check for SDK server

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HealthResponse**](HealthResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## track_feature_event

> track_feature_event(feature_key, track_request)
Track event for a feature (impression / conversion / error / custom)

Send a feedback event related to a feature evaluation. Events are written to TimescaleDB (hypertable) and used for analytics, auto-disable and training MAB algorithms. The project is derived from the API key. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feature_key** | **String** |  | [required] |
**track_request** | [**TrackRequest**](TrackRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

